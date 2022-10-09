#[derive(Default)]

pub struct Envelope {
    pub A: f32,
    pub D: f32,
    pub S: f32,
    pub R: f32,
}

impl Envelope {
  /// Initializes a `Voice` with its velocity set to default (100)
  pub fn new() -> Envelope {
    Envelope {
      A: 1.0,
      D: 1.0,
      S: 1.0,
      R: 1.0,
    }
  }

  pub fn value_for(&self, t: f32, released: bool) -> f32 {
    if released {
      return (self.S - (self.S * t) / self.R).max(0.0);
    }
    if t < self.A {
      t / self.A
    } else if t < self.A + self.D {
      1.0 - (1.0 - self.S) * (t - self.A) / self.D
    } else {
      self.S
    }
  }
}