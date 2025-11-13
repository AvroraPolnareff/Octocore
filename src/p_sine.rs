use fundsp::math::{cos, rnd, sin, TAU};
use fundsp::prelude::{An, AudioNode, Frame, SignalFrame};
use fundsp::signal::{new_signal_frame, Signal};
use fundsp::{convert, Real};

pub fn p_sine<T: Real>() -> An<PSine<T>> {
    An(PSine::new(fundsp::DEFAULT_SR))
}

/// Phase Sine oscillator.
/// - Input 0: frequency in Hz.
/// - Input 1: Phase in decimal
/// - Output 0: sine wave.
#[derive(Default, Clone)]
pub struct PSine<T: Real> {
    phase: T,
    sample_duration: T,
    hash: u64,
    initial_phase: Option<T>,
}

impl<T: Real> PSine<T> {
    pub fn new(sample_rate: f64) -> Self {
        let mut sine = PSine::default();
        sine.reset();
        sine.set_sample_rate(sample_rate);
        sine
    }

    pub fn with_phase(sample_rate: f64, initial_phase: Option<T>) -> Self {
        let mut sine = Self {
            phase: T::zero(),
            sample_duration: T::zero(),
            hash: 0,
            initial_phase,
        };
        sine.reset();
        sine.set_sample_rate(sample_rate);
        sine
    }
}

impl<T: Real> AudioNode for PSine<T> {
    const ID: u64 = 1339;
    type Sample = T;
    type Inputs = typenum::U2;
    type Outputs = typenum::U1;
    type Setting = ();

    fn reset(&mut self) {
        self.phase = match self.initial_phase {
            Some(phase) => phase,
            None => T::from_f64(rnd(self.hash as i64)),
        };
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_duration = convert(1.0 / sample_rate);
    }

    #[inline]
    fn tick(
        &mut self,
        input: &Frame<Self::Sample, Self::Inputs>,
    ) -> Frame<Self::Sample, Self::Outputs> {
        self.phase += input[0] * self.sample_duration;
        while self.phase > T::one() {
            self.phase -= T::one();
        }
        // This is supposedly faster than self.phase -= self.phase.floor();
        [sin((self.phase + input[1]) * T::from_f64(TAU))].into()
    }

    fn process(
        &mut self,
        size: usize,
        input: &[&[Self::Sample]],
        output: &mut [&mut [Self::Sample]],
    ) {
        for i in 0..size {
            self.phase += input[0][i] * self.sample_duration;
            output[0][i] = sin((self.phase + input[1][i]) * T::from_f64(TAU));
        }
        self.phase -= self.phase.floor();
    }

    fn set_hash(&mut self, hash: u64) {
        self.hash = hash;
        self.reset();
    }

    fn route(&mut self, _input: &SignalFrame, _frequency: f64) -> SignalFrame {
        let mut output = new_signal_frame(self.outputs());
        output[0] = Signal::Latency(0.0);
        output
    }
}
