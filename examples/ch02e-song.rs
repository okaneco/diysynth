//! Short melody played by a sine wave and saw wave synth
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

    // Short riff which plays once with a sine wave and once as a saw wave
    let mut phase = 0.0;
    for i in 0..samples {
        let quarter_note = i * 4 / sample_rate;
        let quarter_note_pct = ((i * 4) % sample_rate) as f32 / sample_rate as f32;

        if i == sample_rate * 3 / 4 + sample_rate / 8 {
            data.push(-1.0);
            continue;
        }
        match quarter_note {
            0 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 0.0),
                sample_rate as f32,
            )),
            1 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 2.0),
                sample_rate as f32,
            )),
            2 | 3 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0),
                sample_rate as f32,
            )),
            4 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0 - quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            5 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 3.0 + quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            6 => data.push(diysynth::oscillator::advance_sine_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0 - quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            8 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 0.0),
                sample_rate as f32,
            )),
            9 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 2.0),
                sample_rate as f32,
            )),
            10 | 11 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0),
                sample_rate as f32,
            )),
            12 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0 - quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            13 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 3.0 + quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            14 => data.push(diysynth::oscillator::advance_saw_osc(
                &mut phase,
                diysynth::note_frequency(4.0, 5.0 - quarter_note_pct * 2.0),
                sample_rate as f32,
            )),
            _ => data.push(0.0),
        }
    }

    // Normalize the data by the largest absolute value it contains
    let max = data.iter().fold(0.0, |mut max, &a| {
        if a.abs() > max {
            max = a.abs();
        }
        max
    });
    data.iter_mut().for_each(|a| *a /= max);

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("song.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file::<_, i16>(
        &mut w,
        &data,
        samples as u32,
        channels as u16,
        sample_rate as u32,
    )
}
