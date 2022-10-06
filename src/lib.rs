mod voice;

#[macro_use]
extern crate vst;

use std::collections::VecDeque;
use vst::prelude::*;
use vst::plugin::{Info, Plugin, Category};
use rand::random;

const MAX_VOICE_COUNT: u8 = 16;

#[derive(Default)]

struct Selene {
    sample_rate: f64,
    voices: VecDeque<voice::Voice>
}

#[allow(dead_code)]
impl Selene {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data {
            [128, _, _] => self.note_off(data),
            [144, 0, _] => self.note_off(data),
            [144, _, _] => self.note_on(data),
            _ => (),
        }
    }
    
    fn note_on(&mut self, data: [u8; 3]) {
        self.voices.push_back(voice::Voice::new_with_velocity(data[1], data[2]));
        if self.voices.len() > MAX_VOICE_COUNT as usize {
            self.voices.pop_front();
        }
    }

    fn note_off(&mut self, data: [u8; 3]) {
        let mut i = 0u8;
        for v in &self.voices {
            if v.note == data[1] {
                self.voices.remove(i.into());
                break;
            }
            i += 1;
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }
}

impl Plugin for Selene {
    fn new(_host: HostCallback) -> Self {
        Selene {
            sample_rate: 44100.0,
            voices: VecDeque::with_capacity(MAX_VOICE_COUNT.into())
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Selene".to_string(),
            vendor: "Starburst Audio".to_string(),
            inputs: 0,
            outputs: 2,
            unique_id: 1337,
            category: Category::Synth,
            ..Info::default()
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                // More events can be handled here.
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();

        for sample_idx in 0..samples {
            for buf_idx in 0..output_count {
                let buff = outputs.get_mut(buf_idx);
                buff[sample_idx] = (random::<f32>() - 0.5f32) * 0.1f32 * self.voices.len() as f32;
            }
        }
    }
}

plugin_main!(Selene);
