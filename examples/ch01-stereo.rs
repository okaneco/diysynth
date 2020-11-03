//! Stereo sawtooth waves
fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let seconds = 4;
    let channels = 2;
    let samples = sample_rate * channels * seconds;
    let data_size = std::mem::size_of::<i32>();
    let mut data = Vec::with_capacity(samples);

    // Two sawtooth channels, left and right
    let mut value1 = 0i32;
    let mut value2 = 0i32;
    for _ in (0..samples).step_by(channels) {
        value1 = value1.wrapping_add(8_000_000);
        data.push(value1);
        value2 = value2.wrapping_add(12_000_000);
        data.push(value2);
    }

    // Convert the i32 samples to u8 buffer
    let data = data
        .iter()
        .fold(Vec::with_capacity(samples * data_size), |mut vec, a| {
            vec.extend_from_slice(&a.to_le_bytes());
            vec
        });

    // Write file out
    let mut w = std::io::BufWriter::new(std::fs::File::create("outstereo.wav")?);
    diysynth::MinimalWaveHeader::write_wave_file_u8(
        &mut w,
        &data,
        samples as u32 * data_size as u32,
        channels as u16,
        sample_rate as u32,
        data_size as u16 * 8,
    )
}
