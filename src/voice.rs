#[derive(Default)]

pub struct Voice {
    pub velocity: u8,
    pub note: u8,
    /// Lifetime, this ought to continuously tick up for every sample played.
    /// This is a 32-bit unsigned integer, because it should only count up.
    pub t: u32,
}

impl Voice {
    /// Initializes a `Voice` with its velocity set to default (100)
    pub fn new(note: u8) -> Voice { Self::new_with_velocity(note, 100u8) }

    pub fn new_with_velocity(note: u8, velocity: u8) -> Voice {
        if note > 127 {
            panic!("Note number must be between 0 and 127, inclusive.")
        }
        if velocity > 127 {
            panic!("Velocity must be between 0 and 127, inclusive.")
        }
        Voice {
            note,
            velocity,
            t: 0u32
        }
    }
}