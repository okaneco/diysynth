//! Square, saw, and triangle wave oscillators
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

    // Square wave
    let mut phase = 0.0;
    let freq = diysynth::note_frequency(3.0, 3.0);
    for _ in 0..samples {
        data.push(diysynth::oscillator::advance_square_osc(
            &mut phase,
            freq,
            sample_rate as f32,
        ));
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("squarewave.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )?;

    data.clear();
    // Saw wave
    let mut phase = 0.0;
    let freq = diysynth::note_frequency(3.0, 3.0);
    for _ in 0..samples {
        data.push(diysynth::oscillator::advance_saw_osc(
            &mut phase,
            freq,
            sample_rate as f32,
        ));
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sawwave.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )?;

    data.clear();
    // Triangle wave
    let mut phase = 0.0;
    let freq = diysynth::note_frequency(3.0, 3.0);
    for _ in 0..samples {
        data.push(diysynth::oscillator::advance_triangle_osc(
            &mut phase,
            freq,
            sample_rate as f32,
        ));
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("trianglewave.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )
}
