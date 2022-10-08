use vst::{util::AtomicFloat, prelude::PluginParameters};

pub struct Parameters {
    pub osc_a_amplitude: AtomicFloat,
    pub osc_a_partials: [AtomicFloat; 16],
}

impl Default for Parameters {
    fn default() -> Parameters {
        Parameters {
            osc_a_amplitude: AtomicFloat::new(0.75),
            osc_a_partials: [AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0), AtomicFloat::new(0.0)],
        }
    }
}

impl PluginParameters for Parameters {
    // the `get_parameter` function reads the value of a parameter.
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.osc_a_amplitude.get(),
            1..=16 => self.osc_a_partials[(index-1) as usize].get(),
            _ => 0.0,
        }
    }

    // the `set_parameter` function sets the value of a parameter.
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.osc_a_amplitude.set(val),
            1..=16 => self.osc_a_partials[(index-1) as usize].set(val),
            _ => (),
        }
    }

    // This is what will display underneath our control. We can
    // format it into a string that makes the most since.
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}%", self.osc_a_amplitude.get() * 100f32),
            1..=16 => format!(
                "{:.2}%",
                self.osc_a_partials[(index-1) as usize].get() * 100f32
            ),
            _ => "".to_string(),
        }
    }

    // This shows the control's name.
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Oscillator A Amplitude".to_owned(),
            1..=16 => format!("Oscillator A Partial {}", index),
            _ => "".to_owned(),
        }
    }
}