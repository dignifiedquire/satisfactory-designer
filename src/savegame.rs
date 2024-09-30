use anyhow::{ensure, Result};
use bytes::{Buf, Bytes};

// All formats are encoded in little endian

type Byte = u8;
type Int = i32;
type Long = i64;
type Float = f32;

#[derive(Debug)]
pub struct String {
    length: Int,
    /// Includes 1-2 termination characters
    chars: Bytes,
}

impl String {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let length = data.get_i32_le();

        if length == 0 {
            Ok(Self {
                length,
                chars: Bytes::default(),
            })
        } else if length > 0 {
            // UTF 8
            let chars = data.copy_to_bytes(length as _);
            Ok(Self { length, chars })
        } else {
            // Utf 16
            let actual_length = length * -2;
            let chars = data.copy_to_bytes(actual_length as _);
            Ok(Self { length, chars })
        }
    }
}

#[derive(Debug)]
pub struct Savegame {
    pub header: SaveFileHeader,
    pub compressed_chunks: Vec<CompressedBodyChunk>,
}

impl Savegame {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let header = SaveFileHeader::parse(data)?;

        let mut compressed_chunks = Vec::new();

        while data.has_remaining() {
            let chunk = CompressedBodyChunk::parse(data)?;
            compressed_chunks.push(chunk);
        }

        Ok(Savegame {
            header,
            compressed_chunks,
        })
    }

    pub fn uncompressed_len(&self) -> usize {
        self.compressed_chunks
            .iter()
            .map(|c| c.uncompressed_size as usize)
            .sum()
    }

    pub fn decompress(&self) -> Result<Vec<u8>> {
        use flate2::read::ZlibDecoder;
        use std::io::Read;

        let mut out = Vec::with_capacity(self.uncompressed_len());
        let mut offset = 0;
        let mut decoder = ZlibDecoder::new(&[][..]);
        for chunk in &self.compressed_chunks {
            decoder.reset(&chunk.bytes[..]);
            let read = decoder.read(&mut out[offset..])?;
            offset += read;
        }

        Ok(out)
    }
}

#[derive(derive_more::Debug)]
pub struct CompressedBodyChunk {
    pub compressed_size: Int,
    pub uncompressed_size: Int,
    #[debug("{} bytes", bytes.len())]
    pub bytes: Bytes,
}

fn read_chunk<B: Buf>(data: &mut B) -> (Int, Int, Int, Int) {
    let a = data.get_i32_le();
    let b = data.get_i32_le();
    let c = data.get_i32_le();
    let d = data.get_i32_le();
    (a, b, c, d)
}

const MAGIC_SIGNATURE: Int = -1641380927;
const MAX_CHUNK_SIZE: Int = 128 * 1024;

impl CompressedBodyChunk {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let (signature, _, maximum_chunk_size, _) = read_chunk(data);
        ensure!(signature == MAGIC_SIGNATURE, "corrupted chunk: magic");
        ensure!(
            maximum_chunk_size == MAX_CHUNK_SIZE,
            "corrupted chunk: maximum chunk size"
        );

        let _ = data.get_u8();
        let summary = read_chunk(data);
        let sub_chunk = read_chunk(data);

        let uncompressed_size = summary.3;
        ensure!(
            uncompressed_size == sub_chunk.3,
            "corrupted chunk: uncompressed size"
        );

        let compressed_size = summary.0;
        let bytes = data.copy_to_bytes(compressed_size as _);

        Ok(Self {
            compressed_size,
            uncompressed_size,
            bytes,
        })
    }
}

#[derive(Debug)]
pub struct SaveFileHeader {
    pub header_version: Int,
    pub save_version: Int,
    pub build_version: Int,
    pub map_name: String,
    pub map_options: String,
    pub session_name: String,
    pub session_play_time_secs: Int,
    pub save_timestamp_ticks: Long,
    pub session_visibility: Byte,
    pub editor_object_version: Int,
    pub mod_metadata: String,
    pub mod_flags: Int,
    pub save_identifier: String,
    pub is_partitioned_new_world: Int,
    pub save_data_hash: [u8; 20],
    pub is_creative_mode_enabled: Int,
}

impl SaveFileHeader {
    pub fn parse<B: Buf>(data: &mut B) -> Result<Self> {
        let header_version = data.get_i32_le();
        let save_version = data.get_i32_le();
        let build_version = data.get_i32_le();
        let map_name = String::parse(data)?;
        let map_options = String::parse(data)?;
        let session_name = String::parse(data)?;
        let session_play_time_secs = data.get_i32_le();
        let save_timestamp_ticks = data.get_i64_le();
        let session_visibility = data.get_u8();
        let editor_object_version = data.get_i32_le();
        let mod_metadata = String::parse(data)?;
        let mod_flags = data.get_i32_le();
        let save_identifier = String::parse(data)?;

        let is_partitioned_new_world = data.get_i32_le();
        let mut save_data_hash = [0u8; 20];
        data.copy_to_slice(&mut save_data_hash);
        let is_creative_mode_enabled = data.get_i32_le();

        Ok(Self {
            header_version,
            save_version,
            build_version,
            map_name,
            map_options,
            session_name,
            session_play_time_secs,
            save_timestamp_ticks,
            session_visibility,
            editor_object_version,
            mod_metadata,
            mod_flags,
            save_identifier,
            is_partitioned_new_world,
            save_data_hash,
            is_creative_mode_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        let file = std::fs::read("./tests/simple_save_1_0_0.sav").unwrap();

        let mut buf = &file[..];
        let save_game = Savegame::parse(&mut buf).unwrap();
        dbg!(&save_game);
        assert_eq!(save_game.header.header_version, 13);

        let file_data = save_game.decompress().unwrap();
        assert_eq!(file_data.len(), save_game.uncompressed_len());
    }
}
