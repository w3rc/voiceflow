use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use hound::{WavSpec, WavWriter};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

use crate::state::RecordingState;

pub struct Recorder {
    stream: Option<Stream>,
}

impl Recorder {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn list_devices() -> Result<Vec<String>, String> {
        let host = cpal::default_host();
        let devices = host
            .input_devices()
            .map_err(|e| format!("Failed to enumerate input devices: {}", e))?;

        let names: Vec<String> = devices.filter_map(|d| d.name().ok()).collect();

        Ok(names)
    }

    pub fn start(
        &mut self,
        recording_state: Arc<Mutex<RecordingState>>,
        device_name: Option<&str>,
    ) -> Result<(), String> {
        let host = cpal::default_host();

        let device = if let Some(name) = device_name {
            host.input_devices()
                .map_err(|e| format!("Failed to enumerate devices: {}", e))?
                .find(|d| d.name().ok().as_deref() == Some(name))
                .ok_or_else(|| format!("Device '{}' not found", name))?
        } else {
            host.default_input_device()
                .ok_or_else(|| "No default input device available".to_string())?
        };

        let config = get_mono_config(&device)?;
        let sample_rate = config.sample_rate.0;

        // Reset recording state
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
    }
}

fn get_mono_config(device: &Device) -> Result<StreamConfig, String> {
    let supported = device
        .supported_input_configs()
        .map_err(|e| format!("Failed to get supported configs: {}", e))?;

    // Try to find a config that supports 16kHz mono f32
    for cfg in supported {
        if cfg.channels() == 1 && cfg.sample_format() == SampleFormat::F32 {
            let target_rate = cpal::SampleRate(16000);
            if cfg.min_sample_rate() <= target_rate && cfg.max_sample_rate() >= target_rate {
                return Ok(cfg.with_sample_rate(target_rate).into());
            }
        }
    }

    // Fallback: use default config and we'll resample later
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
