//! A quiet mono sine wave and a clipping sine wave
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let seconds = 4;
    let channels = 1;
    let samples = sample_rate * channels * seconds;
    let mut data = Vec::with_capacity(samples);

    // Quiet sine wave
    let mut phase = 0.0;
    let freq = diysynth::note_frequency(3.0, 3.0);
    for _ in 0..samples {
        data.push(
            diysynth::oscillator::advance_sine_osc(&mut phase, freq, sample_rate as f32) * 0.4,
        );
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sinequiet.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )?;

    data.clear();
    // Clipping sine wave
    let mut phase = 0.0;
    let freq = diysynth::note_frequency(3.0, 3.0);
    for _ in 0..samples {
        data.push(
            diysynth::oscillator::advance_sine_osc(&mut phase, freq, sample_rate as f32) * 1.4,
        );
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sineclip.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )
}
