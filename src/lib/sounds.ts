/**
 * Lightweight audio feedback using the Web Audio API.
 * No external files — tones are synthesised on the fly.
 */

let ctx: AudioContext | null = null;

function getCtx(): AudioContext {
  if (!ctx || ctx.state === "closed") {
    ctx = new AudioContext();
  }
  return ctx;
}

function playTone(
  frequency: number,
  duration: number,
  type: OscillatorType = "sine",
  volume = 0.3,
  fadeOut = true
) {
  const ac = getCtx();
  const osc = ac.createOscillator();
  const gain = ac.createGain();

  osc.connect(gain);
  gain.connect(ac.destination);

  osc.type = type;
  osc.frequency.setValueAtTime(frequency, ac.currentTime);

  gain.gain.setValueAtTime(volume, ac.currentTime);
  if (fadeOut) {
    gain.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + duration);
  }

  osc.start(ac.currentTime);
  osc.stop(ac.currentTime + duration);
}

/** Short rising beep — played when recording starts. */
export function playStartSound() {
  playTone(520, 0.12, "sine", 0.25);
}

/** Short falling beep — played when recording stops. */
export function playStopSound() {
  playTone(380, 0.12, "sine", 0.25);
}

/** Two-note chime — played when transcription is injected. */
export function playDoneSound() {
  const ac = getCtx();
  // First note
  const osc1 = ac.createOscillator();
  const gain1 = ac.createGain();
  osc1.connect(gain1);
  gain1.connect(ac.destination);
  osc1.type = "sine";
  osc1.frequency.setValueAtTime(523, ac.currentTime); // C5
  gain1.gain.setValueAtTime(0.25, ac.currentTime);
  gain1.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + 0.18);
  osc1.start(ac.currentTime);
  osc1.stop(ac.currentTime + 0.18);

  // Second note (slightly delayed)
  const osc2 = ac.createOscillator();
  const gain2 = ac.createGain();
  osc2.connect(gain2);
  gain2.connect(ac.destination);
  osc2.type = "sine";
  osc2.frequency.setValueAtTime(784, ac.currentTime + 0.12); // G5
  gain2.gain.setValueAtTime(0.001, ac.currentTime + 0.12);
  gain2.gain.linearRampToValueAtTime(0.25, ac.currentTime + 0.18);
  gain2.gain.exponentialRampToValueAtTime(0.001, ac.currentTime + 0.38);
  osc2.start(ac.currentTime + 0.12);
  osc2.stop(ac.currentTime + 0.38);
}
