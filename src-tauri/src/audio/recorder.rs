use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use hound::{WavSpec, WavWriter};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use crate::state::RecordingState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioDevice {
    /// The internal name used for recording (PulseAudio source name or ALSA device name).
    pub name: String,
    /// Human-readable label shown in the UI.
    pub label: String,
}

pub struct Recorder {
    stream: Option<Stream>,
}

impl Recorder {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn list_devices() -> Result<Vec<AudioDevice>, String> {
        // On Linux, prefer pactl enumeration which returns PipeWire/PulseAudio
        // sources with human-readable descriptions.
        #[cfg(target_os = "linux")]
        {
            let pulse_devices = list_pulse_sources();
            if !pulse_devices.is_empty() {
                return Ok(pulse_devices);
            }
            log::warn!("pactl enumeration returned nothing, falling back to ALSA");
        }

        // Fallback: cpal ALSA enumeration (macOS / Windows / Linux without pactl)
        let host = cpal::default_host();
        let devices: Vec<AudioDevice> = host
            .input_devices()
            .map_err(|e| format!("Failed to enumerate input devices: {}", e))?
            .filter_map(|d| {
                d.name().ok().map(|n| AudioDevice {
                    label: n.clone(),
                    name: n,
                })
            })
            .collect();

        Ok(devices)
    }

    pub fn start(
        &mut self,
        recording_state: Arc<Mutex<RecordingState>>,
        device_name: Option<&str>,
    ) -> Result<(), String> {
        let host = cpal::default_host();

        let device = select_input_device(&host, device_name)?;

        let config = get_mono_config(&device)?;
        let sample_rate = config.sample_rate.0;

        {
            let mut s = recording_state
                .lock()
                .map_err(|e| format!("Lock poisoned: {}", e))?;
            s.is_recording = true;
            s.audio_buffer.clear();
            s.sample_rate = sample_rate;
        }

        let state_clone = recording_state;

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut s) = state_clone.lock() {
                        if s.is_recording {
                            s.audio_buffer.extend_from_slice(data);
                        }
                    }
                },
                move |err| {
                    log::error!("Audio stream error: {}", err);
                },
                None,
            )
            .map_err(|e| format!("Failed to build input stream: {}", e))?;

        stream
            .play()
            .map_err(|e| format!("Failed to start stream: {}", e))?;

        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.stream = None;
        // Clear PULSE_SOURCE so subsequent default recordings aren't affected.
        #[cfg(target_os = "linux")]
        unsafe {
            std::env::remove_var("PULSE_SOURCE");
        }
    }
}

/// Select the best available input device, with multiple fallback layers.
fn select_input_device(
    host: &cpal::Host,
    device_name: Option<&str>,
) -> Result<cpal::Device, String> {
    let name = match device_name {
        None => {
            return host
                .default_input_device()
                .ok_or_else(|| "No default input device available".to_string());
        }
        Some(n) => n,
    };

    // 1. Try exact match in cpal's device list
    if let Some(dev) = host
        .input_devices()
        .ok()
        .and_then(|mut i| i.find(|d| d.name().ok().as_deref() == Some(name)))
    {
        log::info!("Using ALSA device: {}", name);
        return Ok(dev);
    }

    // 2. On Linux: device is likely a PulseAudio/PipeWire source name.
    //    Set PULSE_SOURCE so the PipeWire ALSA device routes to it, then try
    //    common PipeWire/PulseAudio ALSA bridge device names in order.
    #[cfg(target_os = "linux")]
    {
        log::info!("'{}' not found in ALSA; trying PipeWire/PulseAudio bridge", name);
        // SAFETY: recordings are serialised; no concurrent env mutation.
        unsafe {
            std::env::set_var("PULSE_SOURCE", name);
            std::env::set_var("PIPEWIRE_NODE", name);
        }

        let bridge_names = ["pipewire", "pulse", "default", "sysdefault"];
        for bridge in bridge_names {
            if let Some(dev) = host
                .input_devices()
                .ok()
                .and_then(|mut i| i.find(|d| d.name().ok().as_deref() == Some(bridge)))
            {
                log::info!("Routing '{}' via ALSA bridge '{}'", name, bridge);
                return Ok(dev);
            }
        }
    }

    // 3. Last resort: system default (always works, may not be the right mic)
    log::warn!("Could not find '{}'; falling back to system default", name);
    host.default_input_device()
        .ok_or_else(|| format!("Device '{}' not found and no default available", name))
}

/// Enumerate PulseAudio/PipeWire input sources via `pactl list sources`.
/// Returns devices with human-readable descriptions (matching GNOME Sound settings).
#[cfg(target_os = "linux")]
fn list_pulse_sources() -> Vec<AudioDevice> {
    use std::process::Command;

    let output = match Command::new("pactl").args(["list", "sources"]).output() {
        Ok(o) if o.status.success() => o,
        Ok(o) => {
            log::warn!("pactl exited {}", o.status);
            return vec![];
        }
        Err(e) => {
            log::warn!("Failed to run pactl: {}", e);
            return vec![];
        }
    };

    parse_pactl_sources(&String::from_utf8_lossy(&output.stdout))
}

/// Parse `pactl list sources` output into AudioDevice entries.
/// Each source block looks like:
///   Source #N
///       Name: alsa_input.usb-...
///       Description: Microphone - USB PnP Audio Device
///       ...
#[cfg(target_os = "linux")]
fn parse_pactl_sources(text: &str) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_desc: Option<String> = None;

    for line in text.lines() {
        let trimmed = line.trim();

        if line.starts_with("Source #") {
            // Flush previous block
            if let (Some(name), Some(desc)) = (current_name.take(), current_desc.take()) {
                if !name.ends_with(".monitor") {
                    devices.push(AudioDevice { name, label: desc });
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("Name:") {
            current_name = Some(rest.trim().to_string());
        } else if let Some(rest) = trimmed.strip_prefix("Description:") {
            current_desc = Some(rest.trim().to_string());
        }
    }

    // Flush final block
    if let (Some(name), Some(desc)) = (current_name, current_desc) {
        if !name.ends_with(".monitor") {
            devices.push(AudioDevice { name, label: desc });
        }
    }

    devices
}

fn get_mono_config(device: &Device) -> Result<StreamConfig, String> {
    let supported = device
        .supported_input_configs()
        .map_err(|e| format!("Failed to get supported configs: {}", e))?;

    for cfg in supported {
        if cfg.channels() == 1 && cfg.sample_format() == SampleFormat::F32 {
            let target_rate = cpal::SampleRate(16000);
            if cfg.min_sample_rate() <= target_rate && cfg.max_sample_rate() >= target_rate {
                return Ok(cfg.with_sample_rate(target_rate).into());
            }
        }
    }

    let default_config = device
        .default_input_config()
        .map_err(|e| format!("Failed to get default config: {}", e))?;

    Ok(StreamConfig {
        channels: 1,
        sample_rate: default_config.sample_rate(),
        buffer_size: cpal::BufferSize::Default,
    })
}

pub fn samples_to_wav(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>, String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = WavWriter::new(&mut cursor, spec)
            .map_err(|e| format!("Failed to create WAV writer: {}", e))?;

        for &sample in samples {
            let amplitude = (sample * i16::MAX as f32) as i16;
            writer
                .write_sample(amplitude)
                .map_err(|e| format!("Failed to write sample: {}", e))?;
        }

        writer
            .finalize()
            .map_err(|e| format!("Failed to finalize WAV: {}", e))?;
    }

    Ok(cursor.into_inner())
}
