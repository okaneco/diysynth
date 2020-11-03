//! Example from the README, generates a 4 note chord made up of sine waves
use diysynth::{note_frequency, oscillator::advance_sine_osc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let seconds = 4;
    let channels = 1;
    let samples = sample_rate * channels * seconds;
    let mut data = Vec::with_capacity(samples);

    // Initialize 4 oscillators with their respective notes
    let mut phase0 = 0.0;
    let mut phase1 = 0.0;
    let mut phase2 = 0.0;
    let mut phase3 = 0.0;
    let freq0 = note_frequency(3.0, 3.0);
    let freq1 = note_frequency(3.0, 7.0);
    let freq2 = note_frequency(4.0, 0.0);
    let freq3 = note_frequency(4.0, 5.0);
    for _ in 0..samples {
        data.push(
            advance_sine_osc(&mut phase0, freq0, sample_rate as f32)
                + advance_sine_osc(&mut phase1, freq1, sample_rate as f32)
                + advance_sine_osc(&mut phase2, freq2, sample_rate as f32)
                + advance_sine_osc(&mut phase3, freq3, sample_rate as f32),
        );
    }

    // Normalize the data by the largest absolute value it contains
    let max = data.iter().fold(0.0, |mut max, &a| {
        if a.abs() > max {
            max = a.abs();
        }
        max
    });
    data.iter_mut().for_each(|a| *a /= max);

    // Write the data to a 16-bit wave file
    let mut w = std::io::BufWriter::new(std::fs::File::create("sinechord.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )
}
