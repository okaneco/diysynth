use std::{convert::TryFrom, error::Error, io::Write};

/// Header struct for writing the data to a WAVE file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinimalWaveHeader {
    // Main chunk
    chunk_id: [u8; 4],
    chunk_size: u32,
    format: [u8; 4],
    // sub chunk 1 "fmt"
    sub_chunk_1_id: [u8; 4],
    sub_chunk_1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    // sub chunk 2 "data"
    sub_chunk_2_id: [u8; 4],
    sub_chunk_2_size: u32,
}

impl MinimalWaveHeader {
    /// Create a new WAVE header object.
    pub fn new(
        chunk_size: u32,
        sub_chunk_1_size: u32,
        audio_format: u16,
        num_channels: u16,
        sample_rate: u32,
        byte_rate: u32,
        block_align: u16,
        bits_per_sample: u16,
        sub_chunk_2_size: u32,
    ) -> Self {
        Self {
            chunk_id: [b'R', b'I', b'F', b'F'],
            chunk_size,
            format: [b'W', b'A', b'V', b'E'],
            sub_chunk_1_id: [b'f', b'm', b't', b' '],
            sub_chunk_1_size,
            audio_format,
            num_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
            sub_chunk_2_id: [b'd', b'a', b't', b'a'],
            sub_chunk_2_size,
        }
    }

    /// Write a WAVE file which is generic over the bit-depth of the output
    /// data size. The user can supply a buffer of `f32` samples ranging from
    /// -1.0 to 1.0.
    pub fn write_wave_file<W: Write, T: crate::WaveComponent>(
        w: &mut W,
        data: &[f32],
        num_samples: u32,
        num_channels: u16,
        sample_rate: u32,
    ) -> Result<(), Box<dyn Error>> {
        if data.len() != usize::try_from(num_samples)? {
            return Err("Data length not equal to the number of samples".into());
        }

        let bits_per_sample = core::mem::size_of::<T>() as u16 * 8;
        let data_size = core::mem::size_of::<T>() as u32 * num_samples;

        let header = Self::new(
            data_size + 36,
            16,
            1,
            num_channels,
            sample_rate,
            sample_rate * u32::from(num_channels) * u32::from(bits_per_sample) / 8,
            num_channels * bits_per_sample / 8,
            bits_per_sample,
            data_size,
        );
        header.write_wave_header(w)?;

        let data = data
            .iter()
            .flat_map(|&d| T::float_to_audio_sample(d))
            .collect::<Vec<u8>>();

        Ok(w.write_all(&data)?)
    }

    /// Write a WAVE file with data already existing in a `u8` buffer.
    pub fn write_wave_file_u8<W: Write>(
        w: &mut W,
        data: &[u8],
        data_size: u32,
        num_channels: u16,
        sample_rate: u32,
        bits_per_sample: u16,
    ) -> Result<(), Box<dyn Error>> {
        if data.len() != usize::try_from(data_size)? {
            return Err("Data length not equal to the number of samples".into());
        }

        let header = Self::new(
            data_size + 36,
            16,
            1,
            num_channels,
            sample_rate,
            sample_rate * u32::from(num_channels) * u32::from(bits_per_sample) / 8,
            num_channels * bits_per_sample / 8,
            bits_per_sample,
            data_size,
        );
        header.write_wave_header(w)?;

        Ok(w.write_all(data)?)
    }

    /// Write the header file's contents to a provided writer.
    pub fn write_wave_header<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn Error>> {
        w.write_all(&self.chunk_id)?;
        w.write_all(&self.chunk_size.to_le_bytes())?;
        w.write_all(&self.format)?;
        w.write_all(&self.sub_chunk_1_id)?;
        w.write_all(&self.sub_chunk_1_size.to_le_bytes())?;
        w.write_all(&self.audio_format.to_le_bytes())?;
        w.write_all(&self.num_channels.to_le_bytes())?;
        w.write_all(&self.sample_rate.to_le_bytes())?;
        w.write_all(&self.byte_rate.to_le_bytes())?;
        w.write_all(&self.block_align.to_le_bytes())?;
        w.write_all(&self.bits_per_sample.to_le_bytes())?;
        w.write_all(&self.sub_chunk_2_id)?;

        Ok(w.write_all(&self.sub_chunk_2_size.to_le_bytes())?)
    }
}
