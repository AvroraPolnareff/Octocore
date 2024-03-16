use std::iter::{repeat_with};
use fundsp::hacker::{Shared, shared};
use crate::param::{Param};
use crate::poly::VoiceIndex;

#[derive(Clone)]
pub struct AdsrParams {
  pub a: Shared<f64>,
  pub d: Shared<f64>,
  pub s: Shared<f64>,
  pub r: Shared<f64>,
}
impl Default for AdsrParams {
  fn default() -> Self {
    Self {
      a: shared(0.01),
      d: shared(0.0),
      s: shared(1.0),
      r: shared(0.0),
    }
  }
}

#[derive(Clone)]
pub struct OpParams {
  pub ratio: Param,
  pub volume: Param,
  pub adsr_params: AdsrParams
}

impl Default for OpParams {
  fn default() -> Self {
    Self {
      ratio: Param::new(1.0, (1.0, 999.0), None),
      volume: Param::new(0.05, (0.0, 1.0), None),
      adsr_params: AdsrParams::default()
    }
  }
}

#[derive(Clone)]
pub struct VoiceParams {
  pub pitch: Shared<f64>,
  pub volume: Shared<f64>,
  pub pitch_bend: Shared<f64>,
  pub control: Shared<f64>,
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

#[derive(Clone)]
pub struct SynthParams {
  pub voice_params: Vec<VoiceParams>,
  pub ops: Vec<OpParams>,
}

impl Default for SynthParams {
  fn default() -> Self {
    Self {
      voice_params: repeat_with(|| VoiceParams::default()).take(8).collect(),
      ops: repeat_with(|| OpParams::default()).take(4).collect(),
    }
  }
}

impl SynthParams {
  pub fn new(voice_count: VoiceIndex) -> Self {
    Self {
      voice_params: repeat_with(|| VoiceParams::default()).take(voice_count as usize).collect(),
      ops: repeat_with(|| OpParams::default()).take(4).collect()
    }
  }
}