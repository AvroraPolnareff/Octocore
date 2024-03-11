use crate::param::Param;
use crate::synth_params::SynthParams;

#[derive(Clone)]
pub struct ModDestination {
	pub name: String,
	pub dest: Param,
}

pub type ModDestinations = Vec<(usize, ModDestination)>;

pub fn create_modulation_list(synth_params: &SynthParams) -> ModDestinations {
	let synth_params = synth_params.clone();
	vec![
		ModDestination {name: String::from("Vol"), dest: synth_params.ops[1].volume.clone()},
		ModDestination {name: String::from("Ratio"), dest: synth_params.ops[1].ratio.clone()},
		// ModDestination {name: String::from("Vol"), dest: synth_params.op1.volume},
		// ModDestination {name: String::from("Ratio"), dest: synth_params.op1.ratio},
	].iter().enumerate().map(|(i, dest)| (i, dest.clone())).collect()
}