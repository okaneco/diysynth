pub mod oscillator;
mod wave;

pub use wave::MinimalWaveHeader;

/// Calculate the frequency of a supplied note. An input of (3, 3) corresponds
/// to C4.
pub fn note_frequency(octave: f32, note: f32) -> f32 {
    /*
    440 * (2 ^ (n / 12))
        0  = A
        1  = A#
        2  = B
        3  = C
        4  = C#
        5  = D
        6  = D#
        7  = E
        8  = F
        9  = F#
        10 = G
        11 = G#
    */
    440.0 * 2.0f32.powf(((octave - 4.0) * 12.0 + note) / 12.0)
}

/// Trait for writing WAVE files using the implementing type's bit depth.
pub trait WaveComponent {
    fn float_to_audio_sample(float: f32) -> Vec<u8>;
}

impl WaveComponent for u8 {
    fn float_to_audio_sample(float: f32) -> Vec<u8> {
        vec![f32::clamp_component((float + 1.0) * 127.5, 0.0, 255.0).round() as u8]
    }
}

impl WaveComponent for i16 {
    fn float_to_audio_sample(float: f32) -> Vec<u8> {
        let [byte_0, byte_1] = (f32::clamp_component(
            float * f32::from(i16::MAX),
            f32::from(i16::MIN),
            f32::from(i16::MAX),
        )
        .round() as i16)
            .to_le_bytes();

        vec![byte_0, byte_1]
    }
}

impl WaveComponent for i32 {
    fn float_to_audio_sample(float: f32) -> Vec<u8> {
        let [byte_0, byte_1, byte_2, byte_3] = (f64::clamp_component(
            f64::from(float) * f64::from(i32::MAX),
            f64::from(i32::MIN),
            f64::from(i32::MAX),
        )
        .round() as i32)
            .to_le_bytes();

        vec![byte_0, byte_1, byte_2, byte_3]
    }
}

/// Trait for enabling clamping on float components.
pub trait FloatComponent {
    fn clamp_component(n: Self, min: Self, max: Self) -> Self;
}

impl FloatComponent for f32 {
    fn clamp_component(n: Self, min: Self, max: Self) -> Self {
        if n < min {
            min
        } else if n > max {
            max
        } else {
            n
        }
    }
}

impl FloatComponent for f64 {
    fn clamp_component(n: Self, min: Self, max: Self) -> Self {
        if n < min {
            min
        } else if n > max {
            max
        } else {
            n
        }
    }
}
