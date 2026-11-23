use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use ringbuf::traits::{Consumer, Producer, Split};
use ringbuf::HeapRb;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct AudioEngine {
    _output_stream: Option<Stream>,
    _input_stream: Option<Stream>,
    is_muted: Arc<AtomicBool>,
    is_deafened: Arc<AtomicBool>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let host = cpal::default_host();
        log::info!("Audio Host: {:?}", host.id());

        let is_muted = Arc::new(AtomicBool::new(false));
        let is_deafened = Arc::new(AtomicBool::new(false));

        // 1. Output stream setup (Speakers/Headphones)
        let output_stream = if let Some(device) = host.default_output_device() {
            log::info!("Default Output Device: {:?}", device.name().unwrap_or_default());
            let config: StreamConfig = device.default_output_config()?.into();
            log::info!("Output config: sample_rate={}, channels={}", config.sample_rate.0, config.channels);

            // Ring buffer for audio playback: 48000 samples (~0.5s stereo)
            let rb = HeapRb::<f32>::new(48000 * 2);
            let (_prod, mut cons) = rb.split();

            let is_deaf = is_deafened.clone();
            let stream = device.build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if is_deaf.load(Ordering::Relaxed) {
                        for sample in data.iter_mut() {
                            *sample = 0.0;
                        }
                        return;
                    }

                    for sample in data.iter_mut() {
                        *sample = cons.try_pop().unwrap_or(0.0);
                    }
                },
                |err| log::error!("Audio output stream error: {:?}", err),
                None,
            )?;
            stream.play()?;
            Some(stream)
        } else {
            log::warn!("No audio output device found!");
            None
        };

        // 2. Input stream setup (Microphone)
        let input_stream = if let Some(device) = host.default_input_device() {
            log::info!("Default Input Device: {:?}", device.name().unwrap_or_default());
            let config: StreamConfig = device.default_input_config()?.into();
            log::info!("Input config: sample_rate={}, channels={}", config.sample_rate.0, config.channels);

            let rb = HeapRb::<f32>::new(48000 * 2);
            let (mut prod, _cons) = rb.split();

            let is_mute = is_muted.clone();
            let stream = device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if is_mute.load(Ordering::Relaxed) {
                        return;
                    }

                    // Push PCM samples to lock-free ring buffer
                    for &sample in data {
                        let _ = prod.try_push(sample);
                    }
                },
                |err| log::error!("Audio input stream error: {:?}", err),
                None,
            )?;
            stream.play()?;
            Some(stream)
        } else {
            log::warn!("No audio input device found!");
            None
        };

        Ok(Self {
            _output_stream: output_stream,
            _input_stream: input_stream,
            is_muted,
            is_deafened,
        })
    }

    pub fn controller(&self) -> AudioController {
        AudioController {
            is_muted: self.is_muted.clone(),
            is_deafened: self.is_deafened.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AudioController {
    is_muted: Arc<AtomicBool>,
    is_deafened: Arc<AtomicBool>,
}

impl AudioController {
    pub fn set_muted(&self, muted: bool) {
        self.is_muted.store(muted, Ordering::SeqCst);
    }

    pub fn set_deafened(&self, deafened: bool) {
        self.is_deafened.store(deafened, Ordering::SeqCst);
    }
}
