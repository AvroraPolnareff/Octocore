use fundsp::hacker::{Shared, shared};

#[derive(Clone)]
pub struct VoiceParams {
  pub pitch: Shared<f64>,
  pub volume: Shared<f64>,
  pub pitch_bend: Shared<f64>,
  pub control: Shared<f64>
}

impl Default for VoiceParams {
  fn default() -> Self {
    Self {
      pitch: shared(0.0),
      volume: shared(0.0),
      pitch_bend: shared(0.0),
      control: shared(0.0),
    }
  }
}