use fundsp::hacker::{clamp01, lerp};
use fundsp::prelude::{lfo_in, shared, var, An, Atomic, EnvelopeIn, Frame, U5};
use fundsp::Float;

pub fn adsr<F: Float + Atomic>(
) -> An<EnvelopeIn<F, F, impl Fn(F, &Frame<F, U5>) -> F + Sized + Clone, U5, F>> {
    let neg1 = F::from_f64(-1.0);
    let zero = F::from_f64(0.0);
    let a = shared(neg1);
    let b = shared(neg1);
    let attack_start = var(&a);
    let release_start = var(&b);
    lfo_in(move |time, control| {
        let attack = control[0];
        let decay = control[1];
        let sustain = control[2];
        let release = control[3];
        let control = control[4];
        if attack_start.value() < zero && control > zero {
            attack_start.set_value(time);
            release_start.set_value(neg1);
        } else if release_start.value() < zero && control <= zero {
            release_start.set_value(time);
            attack_start.set_value(neg1);
        }
        clamp01(if release_start.value() < zero {
            ads(attack, decay, sustain, time - attack_start.value())
        } else {
            releasing(sustain, release, time - release_start.value())
        })
    })
}

fn ads<F: Float>(attack: F, decay: F, sustain: F, time: F) -> F {
    if time < attack {
        lerp(F::from_f64(0.0), F::from_f64(1.0), time / attack)
    } else {
        let decay_time = time - attack;
        if decay_time < decay {
            lerp(F::from_f64(1.0), sustain, decay_time / decay)
        } else {
            sustain
        }
    }
}

fn releasing<F: Float>(sustain: F, release: F, release_time: F) -> F {
    if release_time > release {
        F::from_f64(0.0)
    } else {
        lerp(sustain, F::from_f64(0.0), release_time / release)
    }
}
