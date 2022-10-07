use std::f64::consts::TAU;

use rand::random;
use vst::prelude::AudioBuffer;

#[derive(Default)]
pub struct Sinebank {
    /// Amplitudes of different harmonics, from 0 to 1
    pub amplitudes: [f32; 32]
}

impl Sinebank {
    pub fn new () -> Self {
        Sinebank {
            amplitudes: [0f32; 32]
        }
    }

    pub fn process_buffer (
        &mut self,
        buffer: &mut AudioBuffer<f32>,
        t: u32,
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

                for n in 0..self.amplitudes.len() {
                    buff[sample_idx] +=
                        self.amplitudes[n] as f32
                        * (((n+1) as f64 * f) as f32).sin();
                }
            }
        }
    }
}