use fundsp::audionode::AudioNode;
use fundsp::math::clamp;
use fundsp::prelude::{shared, An, BufferMut, BufferRef, Shared};
use fundsp::Frame;
use typenum::{U0, U1};

#[derive(Clone)]
pub struct Param {
    value: Shared,
    clamp: (f32, f32),
    process: Option<(fn(value: f32) -> f32)>,
    modulation: Shared,
}

impl Param {
    pub fn new(value: f32, clamp: (f32, f32), process: Option<(fn(value: f32) -> f32)>) -> Self {
        Self {
            value: shared(value),
            clamp,
            process,
            modulation: shared(0.0),
        }
    }

    pub fn set_value(&self, value: f32) {
        self.value
            .set_value(clamp(self.clamp.0, self.clamp.1, value))
    }

    pub fn unmodulated_value(&self) -> f32 {
        self.value.value()
    }

    pub fn set_modulation(&self, value: f32) {
        self.modulation.set_value(value)
    }

    pub fn value(&self) -> f32 {
        clamp(
            self.clamp.0,
            self.clamp.1,
            self.value.value() + self.modulation.value(),
        )
    }
}

/// Outputs the value of a shared variable.
#[derive(Clone)]
pub struct ParamVar {
    param: Param,
}

impl ParamVar {
    pub fn new(param: &Param) -> Self {
        Self {
            param: param.clone(),
        }
    }

    /// Set the value of this variable.
    pub fn set_value(&self, value: f32) {
        self.param.set_value(value)
    }

    /// Get the value of this variable.
    pub fn value(&self) -> f32 {
        self.param.value()
    }
}

impl AudioNode for ParamVar {
    const ID: u64 = 1337;

    type Inputs = U0;
    type Outputs = U1;

    #[inline]
    fn tick(&mut self, _: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let sample: f32 = self.value();
        [sample].into()
    }

    fn process(&mut self, size: usize, _input: &BufferRef, output: &mut BufferMut) {
        let sample = self.value();
        output.set(0, size, sample.into());
    }
}

pub fn param(param: &Param) -> An<ParamVar> {
    An(ParamVar::new(param))
}

#[derive(Clone)]
pub struct ParamSink {
    param: Param,
}

impl ParamSink {
    pub fn new(param: &Param) -> Self {
        Self {
            param: param.clone(),
        }
    }

    pub fn set_value(&self, value: f32) {
        self.param.set_value(value)
    }

    pub fn value(&self) -> f32 {
        self.param.value()
    }
}

impl AudioNode for ParamSink {
    const ID: u64 = 1338;

    type Inputs = U1;
    type Outputs = U0;

    #[inline]
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        self.param.set_modulation(input[0]);
        Frame::default()
    }

    fn process(&mut self, _size: usize, input: &BufferRef, _output: &mut BufferMut) {
        //self.param.set_modulation(input.at(0, 0));
        ()
    }
}

pub fn param_sink(param: &Param) -> An<ParamSink> {
    An(ParamSink::new(param))
}
