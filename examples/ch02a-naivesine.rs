//! Mono sine wave
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
    let data_size = std::mem::size_of::<i16>();
    let mut data = Vec::with_capacity(samples);

    // Mono sine wave
    let freq = 1000.0;
    for i in 0..samples {
        data.push((i as f32 * core::f32::consts::TAU * freq / sample_rate as f32).sin());
    }

    // Convert the i16 samples to u8 buffer
    let data = data
        .iter()
        .fold(Vec::with_capacity(samples * data_size), |mut vec, &a| {
            if a < 0.0 {
                vec.extend_from_slice(&((a * i16::MIN as f32 * -1.0).round() as i16).to_le_bytes());
            } else {
                vec.extend_from_slice(&((a * i16::MAX as f32).round() as i16).to_le_bytes());
            }
            vec
        });

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("sinenaive.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file_u8(
        &mut w,
        &data,
        samples as u32 * data_size as u32,
        channels as u16,
        sample_rate as u32,
        data_size as u16 * 8,
    )
}
