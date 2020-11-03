//! Mono sine wave with changing notes
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

    // Mono sine wave which pops between notes
    for i in 0..samples {
        let freq = if i < samples / 2 {
            diysynth::note_frequency(3.0, 3.0)
        } else {
            diysynth::note_frequency(3.0, 4.0)
        };
        data.push((i as f32 * core::f32::consts::TAU * freq / sample_rate as f32).sin());
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sinediscon.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )?;

    data.clear();
    // Fixed popping in mono sine wave by using an oscillator
    let mut phase = 0.0;
    for i in 0..samples {
        let freq = if i < samples / 2 {
            diysynth::note_frequency(3.0, 3.0)
        } else {
            diysynth::note_frequency(3.0, 4.0)
        };

        data.push(diysynth::oscillator::advance_sine_osc(
            &mut phase,
            freq,
            sample_rate as f32,
        ));
    }

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sinecon.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )
}
