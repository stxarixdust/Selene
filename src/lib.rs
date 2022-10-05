#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};

#[derive(Default)]
struct Selene;

impl Plugin for Selene {
    fn get_info(&self) -> Info {
        Info {
            name: "Selene".to_string(),
            unique_id: 1337,
            ..Default::default()
        }
    }
}

plugin_main!(Selene);
