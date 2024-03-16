use fundsp::audionode::{AudioNode, Frame};
use fundsp::math::clamp;
use fundsp::prelude::{An, Shared, shared};
use typenum::{U0, U1};

#[derive(Clone)]
pub struct Param {
	value: Shared<f64>,
	clamp: (f64, f64),
	process: Option<(fn (value: f64) -> f64)>,
	modulation: Shared<f64>
}

impl Param {
	pub fn new(
		value: f64,
		clamp: (f64, f64),
		process: Option<(fn (value: f64) -> f64)>
	) -> Self {
		Self {
			value: shared(value),
			clamp,
			process,
			modulation: shared(0.0)
		}
	}

	pub fn set_value(&self, value: f64) {
		self.value.set_value(clamp(self.clamp.0, self.clamp.1, value))
	}
	
	pub fn unmodulated_value(&self) -> f64 {
		self.value.value()
	}
	
	pub fn set_modulation(&self, value: f64) { self.modulation.set_value(value) }
	
	pub fn value(&self) -> f64 {
		clamp(self.clamp.0, self.clamp.1, self.value.value() + self.modulation.value())
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
	pub fn set_value(&self, value: f64) {
		self.param.set_value(value)
	}

	/// Get the value of this variable.
	pub fn value(&self) -> f64 { self.param.value() }
}

impl AudioNode for ParamVar {
	const ID: u64 = 1337;

	type Sample = f64;
	type Inputs = U0;
	type Outputs = U1;
	type Setting = ();

	#[inline]
	fn tick(
		&mut self,
		_: &Frame<Self::Sample, Self::Inputs>,
	) -> Frame<Self::Sample, Self::Outputs> {
		let sample: f64 = self.value();
		[sample].into()
	}

	fn process(
		&mut self,
		size: usize,
		_input: &[&[Self::Sample]],
		output: &mut [&mut [Self::Sample]],
	) {
		let sample = self.value();
		output[0][..size].fill(sample);
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

	pub fn set_value(&self, value: f64) {
		self.param.set_value(value)
	}

	pub fn value(&self) -> f64 { self.param.value() }
}

impl AudioNode for ParamSink {
	const ID: u64 = 1338;

	type Sample = f64;
	type Inputs = U1;
	type Outputs = U0;
	type Setting = ();

	#[inline]
	fn tick(
		&mut self,
		input: &Frame<Self::Sample, Self::Inputs>,
	) -> Frame<Self::Sample, Self::Outputs> {
		self.param.set_modulation(input[0]);
		Frame::default()
	}

	fn process(
		&mut self,
		_size: usize,
		input: &[&[Self::Sample]],
		_output: &mut [&mut [Self::Sample]],
	) {
		self.param.set_modulation(input[0][0]);
	}
}

pub fn param_sink(param: &Param) -> An<ParamSink> {
	An(ParamSink::new(param))
}
