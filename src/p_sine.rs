use fundsp::math::{cos, rnd1, sin};
use fundsp::prelude::{An, AudioNode, Frame, SignalFrame};
use fundsp::prelude::{BufferMut, BufferRef};
use fundsp::signal::{Routing, Signal};
use fundsp::{convert, F32x, Real};
use fundsp::{full_simd_items, Float, SIMD_N, SIMD_S};

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
    type Inputs = typenum::U2;
    type Outputs = typenum::U1;

    fn reset(&mut self) {
        self.phase = match self.initial_phase {
            Some(phase) => phase,
            None => T::from_f64(rnd1(self.hash as u64)),
        };
    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_duration = convert(1.0 / sample_rate);
    }

    #[inline]
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        self.phase += T::from_f32(input[0]) * self.sample_duration;
        self.phase -= self.phase.floor();
        [sin((self.phase.to_f32() + input[1]) * f32::TAU)].into()
    }

    fn process(&mut self, size: usize, input: &BufferRef, output: &mut BufferMut) {
        for i in 0..full_simd_items(size) {
            let element: [f32; SIMD_N] = core::array::from_fn(|j| {
                let tmp = self.phase.to_f32();
                self.phase +=
                    T::from_f32(input.at_f32(0, (i << SIMD_S) + j)) * self.sample_duration;
                tmp
            });
            output.set(0, i, (F32x::new(element) * f32::TAU).sin());
        }
        self.phase -= self.phase.floor();
    }

    fn set_hash(&mut self, hash: u64) {
        self.hash = hash;
        self.reset();
    }

    fn route(&mut self, input: &SignalFrame, _frequency: f64) -> SignalFrame {
        Routing::Arbitrary(0.0).route(input, self.outputs())
    }
}
