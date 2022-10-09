use std::f64::consts::TAU;

use vst::prelude::AudioBuffer;

#[derive(Default)]
pub struct Sinebank {
    pub amplitude: f32,
    /// Partials of different harmonics, from 0 to 1
    pub partials: [f32; 16]
}

impl Sinebank {
    pub fn new () -> Self {
        Sinebank {
            amplitude: 1.0f32,
            partials: [0f32; 16]
        }
    }

    pub fn process_buffer (
        &mut self,
        buffer: &mut AudioBuffer<f32>,
        t: u32,
        velocity: u8,
        pitch: f64,
        samplerate: f64
    ) {
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();

        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                let x = (t + sample_idx as u32) as f64;
                let f = x * pitch * TAU / samplerate;

                for n in 0..self.partials.len() {
                    // If the partial is above the Nyquist-Shannon frequency,
                    // skip it (this prevents aliasing)
                    if (pitch * (n+1) as f64) < (samplerate / 2.0) {
                        // Calculate the signal of the partial
                        buff[sample_idx] +=
                            self.amplitude
                            * (velocity as f32 / 127.0)
                            * self.partials[n]
                            * (((n+1) as f64 * f) as f32).sin();
                    }
                }
            }
        }
    }
}