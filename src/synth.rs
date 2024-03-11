use cpal::{BufferSize, Device, FromSample, SampleFormat, SampleRate, SizedSample, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::audiounit::AudioUnit64;
use fundsp::combinator::An;
use fundsp::hacker::{sine, var, Shared, NetBackend64, sine_hz, AudioNode, U1, U0, pass, oversample, constant};

use crate::adsr::adsr;
use crate::param::{param, Param, param_sink};
use crate::poly::VoiceIndex;
use crate::synth_params::{AdsrParams, OpParams, SynthParams, VoiceParams};


pub fn c_adsr(
	adsr_params: &AdsrParams,
	control: &Shared<f64>
) -> An<impl AudioNode<Inputs = U0, Outputs = U1, Sample = f64>> {
	(
		var(&adsr_params.a)
			| var(&adsr_params.d)
			| var(&adsr_params.s)
			| var(&adsr_params.r)
			| var(&control)
	) >> adsr()
}

pub fn op(
	voice_params: &VoiceParams,
	op_params: &OpParams
) -> An <impl AudioNode<Inputs = U1, Outputs = U1, Sample = f64>> {
	let bf = || var(&voice_params.pitch) * var(&voice_params.pitch_bend) * param(&op_params.ratio);
	pass() * bf() + bf() >> sine() * c_adsr(
		&op_params.adsr_params, &voice_params.control
	) * param(&op_params.volume)
}

pub fn create_sound(
	synth_params: &SynthParams,
	voice_index: VoiceIndex
) -> Box<dyn AudioUnit64> {
	let voice_params = &synth_params.voice_params[voice_index as usize];
	

	Box::new(
		constant(1.)
			>> oversample(op(&voice_params, &synth_params.ops[3]))
			>> oversample(op(&voice_params, &synth_params.ops[2]))
			>> oversample(op(&voice_params, &synth_params.ops[1]))
			>> oversample(op(&voice_params, &synth_params.ops[0]))
			* var(&voice_params.volume)
	)
}

pub fn sine_lfo(param: &Param) -> Box<dyn AudioUnit64> {
	Box::new(
		sine_hz(0.5) * 10.0 >> param_sink(param)
	)
}

pub fn pitch_bend_factor(bend: u16) -> f64 {
	2.0_f64.powf(((bend as f64 - 8192.0) / 8192.0) / 12.0)
}

fn run_synth<T: SizedSample + FromSample<f64>>(
	device: Device,
	config: StreamConfig,
	backend: NetBackend64
) {
	std::thread::spawn(move || {
		let mut backend = backend;
		backend.set_sample_rate(config.sample_rate.0 as f64);

		let mut next_value = move || backend.get_stereo();
		let channels = config.channels as usize;
		let err_fn = |err| eprintln!("an error occurred on stream: {err}");
		let stream = device
			.build_output_stream(
				&config,
				move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
					write_data(data, channels, &mut next_value)
				},
				err_fn,
				None,
			)
			.unwrap();

		stream.play().unwrap();
		loop {
			std::thread::sleep(std::time::Duration::from_millis(1));
		}
	});
}

pub fn run_output(
	backend: NetBackend64
) {
	let host = cpal::default_host();
	let device = host
		.default_output_device()
		.expect("failed to find a default output device");
	let supported_config = device.default_output_config().unwrap();
	let config = StreamConfig {
		channels: 2, sample_rate: SampleRate(48000), buffer_size: BufferSize::Fixed(256)
	};

	match supported_config.sample_format() {
		SampleFormat::F32 => {
			run_synth::<f32>(device, config.into(), backend)
		}
		SampleFormat::I16 => {
			run_synth::<i16>(device, config.into(), backend)
		}
		SampleFormat::U16 => {
			run_synth::<u16>(device, config.into(), backend)
		}
		_ => panic!("Unsupported format"),
	}
}

pub fn write_data<T: SizedSample + FromSample<f64>>(
	output: &mut [T],
	channels: usize,
	next_sample: &mut dyn FnMut() -> (f64, f64),
) {
	for frame in output.chunks_mut(channels) {
		let sample = next_sample();
		let left: T = T::from_sample(sample.0);
		let right: T = T::from_sample(sample.1);

		for (channel, sample) in frame.iter_mut().enumerate() {
			*sample = if channel & 1 == 0 { left } else { right };
		}
	}
}

