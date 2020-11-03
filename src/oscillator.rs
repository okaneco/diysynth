/// Sine wave oscillator.
pub fn advance_sine_osc(phase: &mut f32, freq: f32, sample_rate: f32) -> f32 {
    *phase += core::f32::consts::TAU * freq / sample_rate as f32;
    while *phase >= core::f32::consts::TAU {
        *phase -= core::f32::consts::TAU;
    }
    while *phase < 0.0 {
        *phase += core::f32::consts::TAU;
    }
    phase.sin()
}

/// Square wave oscillator.
pub fn advance_square_osc(phase: &mut f32, freq: f32, sample_rate: f32) -> f32 {
    *phase += freq / sample_rate;
    while *phase > 1.0 {
        *phase -= 1.0;
    }
    while *phase < 0.0 {
        *phase += 1.0;
    }
    if *phase <= 0.5 {
        1.0
    } else {
        -1.0
    }
}

/// Saw wave oscillator.
pub fn advance_saw_osc(phase: &mut f32, freq: f32, sample_rate: f32) -> f32 {
    *phase += freq / sample_rate;
    while *phase > 1.0 {
        *phase -= 1.0;
    }
    while *phase < 0.0 {
        *phase += 1.0;
    }
    ((*phase * 2.0) - 1.0) * -1.0
}

/// Triangle wave oscillator.
pub fn advance_triangle_osc(phase: &mut f32, freq: f32, sample_rate: f32) -> f32 {
    *phase += freq / sample_rate;
    while *phase > 1.0 {
        *phase -= 1.0;
    }
    while *phase < 0.0 {
        *phase += 1.0;
    }
    if *phase <= 0.5 {
        (*phase * 4.0) - 1.0
    } else {
        ((1.0 - *phase) * 4.0) - 1.0
    }
}

/// Band-limited saw wave oscillator. If `harmonics` is 0, the maximum number of
/// harmonics under the Nyquist frequency will be used.
pub fn advance_saw_osc_bl(
    phase: &mut f32,
    mut freq: f32,
    sample_rate: f32,
    mut harmonics: i8,
) -> f32 {
    *phase += core::f32::consts::TAU * freq / sample_rate;
    while *phase >= core::f32::consts::TAU {
        *phase -= core::f32::consts::TAU;
    }
    while *phase < 0.0 {
        *phase += core::f32::consts::TAU;
    }

    // if harmonics is zero, calculate max harmonics before hitting nyquist freq
    if harmonics == 0 && freq != 0.0 {
        while freq < sample_rate * 0.5 {
            harmonics += 1;
            freq *= 2.0;
        }
    }

    let mut ret = 0.0;
    for i in 1..=harmonics as usize {
        ret += (*phase * i as f32).sin() / i as f32;
    }

    ret * 2.0 / core::f32::consts::PI
}

/// Band-limited square wave oscillator. If `harmonics` is 0, the maximum
/// number of harmonics under the Nyquist frequency will be used.
pub fn advance_square_osc_bl(
    phase: &mut f32,
    freq: f32,
    sample_rate: f32,
    mut harmonics: i8,
) -> f32 {
    *phase += core::f32::consts::TAU * freq / sample_rate;
    while *phase >= core::f32::consts::TAU {
        *phase -= core::f32::consts::TAU;
    }
    while *phase < 0.0 {
        *phase += core::f32::consts::TAU;
    }

    // if harmonics is zero, calculate max harmonics before hitting nyquist freq
    if harmonics == 0 && freq != 0.0 {
        while freq * f32::from(harmonics * 2 - 1) < sample_rate * 0.5 {
            harmonics += 1;
        }
        harmonics -= 1;
    }

    let mut ret = 0.0;
    for i in 1..=harmonics as usize {
        ret += (*phase * (i * 2 - 1) as f32).sin() / (i * 2 - 1) as f32;
    }

    ret * 4.0 / core::f32::consts::PI
}

/// Band-limited triangle wave oscillator. If `harmonics` is 0, the maximum
/// number of harmonics under the Nyquist frequency will be used.
pub fn advance_triangle_osc_bl(
    phase: &mut f32,
    freq: f32,
    sample_rate: f32,
    mut harmonics: i8,
) -> f32 {
    *phase += core::f32::consts::TAU * freq / sample_rate;
    while *phase >= core::f32::consts::TAU {
        *phase -= core::f32::consts::TAU;
    }
    while *phase < 0.0 {
        *phase += core::f32::consts::TAU;
    }

    // if harmonics is zero, calculate max harmonics before hitting nyquist freq
    if harmonics == 0 && freq != 0.0 {
        while freq * f32::from(harmonics * 2 - 1) < sample_rate * 0.5 {
            harmonics += 1;
        }
        harmonics -= 1;
    }

    let mut subtract = true;
    let mut ret = 0.0;
    for i in 1..=harmonics as usize {
        if subtract {
            ret -= (*phase * (i * 2 - 1) as f32).sin() / (i * 2 - 1).pow(2) as f32;
        } else {
            ret += (*phase * (i * 2 - 1) as f32).sin() / (i * 2 - 1).pow(2) as f32;
        }
        subtract = !subtract;
    }

    ret * 8.0 / core::f32::consts::PI.powi(2)
}
