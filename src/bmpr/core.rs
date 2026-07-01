use crc32fast::Hasher;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub const MAGIC_NUMBER: [u8; 4] = *b"BMPR";
pub const HEADER_SIZE: usize = 36; // 0x24

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BmprHeader {
    pub engine_version: u8,
    pub seed: u64,
    pub deck_id: u8,
    pub stake_id: u8,
    pub challenge_id: u8,
    pub player_id: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BmprFile {
    pub header: BmprHeader,
    pub instructions: Vec<u64>,
}

#[derive(Debug)]
pub enum BmprError {
    Io(io::Error),
    InvalidMagic,
    BufferTooSmall,
    ChecksumMismatch { expected: u32, actual: u32 },
}

impl From<io::Error> for BmprError {
    fn from(err: io::Error) -> Self {
        BmprError::Io(err)
    }
}

impl BmprHeader {
    pub fn to_bytes(&self, action_count: u32) -> [u8; HEADER_SIZE] {
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..4].copy_from_slice(&MAGIC_NUMBER);
        buf[4] = self.engine_version;
        buf[5..13].copy_from_slice(&self.seed.to_le_bytes());
        buf[13] = self.deck_id;
        buf[14] = self.stake_id;
        buf[15] = self.challenge_id;
        buf[16..24].copy_from_slice(&self.player_id.to_le_bytes());
        buf[24..32].copy_from_slice(&self.timestamp.to_le_bytes());
        buf[32..36].copy_from_slice(&action_count.to_le_bytes());
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Result<(Self, u32), BmprError> {
        if buf.len() < HEADER_SIZE {
            return Err(BmprError::BufferTooSmall);
        }
        if buf[0..4] != MAGIC_NUMBER {
            return Err(BmprError::InvalidMagic);
        }

        let engine_version = buf[4];
        let seed = u64::from_le_bytes(buf[5..13].try_into().unwrap());
        let deck_id = buf[13];
        let stake_id = buf[14];
        let challenge_id = buf[15];
        let player_id = u64::from_le_bytes(buf[16..24].try_into().unwrap());
        let timestamp = u64::from_le_bytes(buf[24..32].try_into().unwrap());
        let action_count = u32::from_le_bytes(buf[32..36].try_into().unwrap());

        Ok((
            Self {
                engine_version,
                seed,
                deck_id,
                stake_id,
                challenge_id,
                player_id,
                timestamp,
            },
            action_count,
        ))
    }
}

impl BmprFile {
    /// Save the BmprFile to disk, automatically applying CRC32 checksum and Zlib compression.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), BmprError> {
        let action_count = self.instructions.len() as u32;

        // 1. Serialize Header
        let mut uncompressed_buf = self.header.to_bytes(action_count).to_vec();

        // 2. Serialize Instructions
        for instr in &self.instructions {
            uncompressed_buf.extend_from_slice(&instr.to_le_bytes());
        }

        // 3. Compute Checksum
        let mut hasher = Hasher::new();
        hasher.update(&uncompressed_buf);
        let checksum = hasher.finalize();

        // Append checksum
        uncompressed_buf.extend_from_slice(&checksum.to_le_bytes());

        // 4. Compress & Write
        let file = File::create(path)?;
        let mut encoder = ZlibEncoder::new(file, Compression::default());
        encoder.write_all(&uncompressed_buf)?;
        encoder.finish()?;

        Ok(())
    }

    /// Load and validate a BmprFile from disk, decompressing and checking the CRC32 checksum.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, BmprError> {
        // 1. Decompress
        let file = File::open(path)?;
        let mut decoder = ZlibDecoder::new(file);
        let mut decompressed_buf = Vec::new();
        decoder.read_to_end(&mut decompressed_buf)?;

        if decompressed_buf.len() < HEADER_SIZE + 4 {
            // Header + Checksum
            return Err(BmprError::BufferTooSmall);
        }

        // 2. Verify Checksum
        let payload_len = decompressed_buf.len() - 4;
        let payload = &decompressed_buf[..payload_len];
        let checksum_bytes = &decompressed_buf[payload_len..];
        let expected_checksum = u32::from_le_bytes(checksum_bytes.try_into().unwrap());

        let mut hasher = Hasher::new();
        hasher.update(payload);
        let actual_checksum = hasher.finalize();

        if actual_checksum != expected_checksum {
            return Err(BmprError::ChecksumMismatch {
                expected: expected_checksum,
                actual: actual_checksum,
            });
        }

        // 3. Parse Header
        let (header, action_count) = BmprHeader::from_bytes(&payload[..HEADER_SIZE])?;

        // 4. Parse Instructions
        let mut instructions = Vec::with_capacity(action_count as usize);
        let mut offset = HEADER_SIZE;
        for _ in 0..action_count {
            if offset + 8 > payload_len {
                return Err(BmprError::BufferTooSmall);
            }
            let instr = u64::from_le_bytes(payload[offset..offset + 8].try_into().unwrap());
            instructions.push(instr);
            offset += 8;
        }

        Ok(Self {
            header,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const HEADER: BmprHeader = BmprHeader {
        engine_version: 100,
        seed: 1234567890,
        deck_id: 0,
        stake_id: 0,
        challenge_id: 0,
        player_id: 0,
        timestamp: 0,
    };
    #[test]
    fn test_bmpr_serialization_cycle() {
        let instructions = vec![0x1122334455667788, 0x99AABBCCDDEEFF00, 0x0102030405060708];

        let file = BmprFile {
            header: HEADER,
            instructions,
        };

        let temp_path = "test_run.bmpr";

        file.save_to_file(temp_path)
            .expect("Failed to save bmpr file");

        let loaded_file = BmprFile::load_from_file(temp_path).expect("Failed to load bmpr file");

        assert_eq!(loaded_file.header, file.header);

        assert_eq!(loaded_file.instructions, file.instructions);

        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_bmpr_invalid_magic() {
        let mut buf = [0u8; HEADER_SIZE];
        buf[0..4].copy_from_slice(b"BAD!"); // Invalid magic

        let result = BmprHeader::from_bytes(&buf);
        assert!(matches!(result, Err(BmprError::InvalidMagic)));
    }

    #[test]
    fn test_bmpr_buffer_too_small() {
        let buf = [0u8; HEADER_SIZE - 1]; // 1 byte too small

        let result = BmprHeader::from_bytes(&buf);
        assert!(matches!(result, Err(BmprError::BufferTooSmall)));
    }

    #[test]
    fn test_bmpr_checksum_mismatch() {
        let file = BmprFile {
            header: HEADER,
            instructions: vec![1, 2, 3],
        };

        let temp_path = "test_checksum.bmpr";

        let action_count = file.instructions.len() as u32;
        let mut uncompressed_buf = file.header.to_bytes(action_count).to_vec();
        for instr in &file.instructions {
            uncompressed_buf.extend_from_slice(&instr.to_le_bytes());
        }

        // Append an intentionally wrong checksum
        let wrong_checksum: u32 = 0xDEADBEEF;
        uncompressed_buf.extend_from_slice(&wrong_checksum.to_le_bytes());

        let out_file = File::create(temp_path).unwrap();
        let mut encoder = ZlibEncoder::new(out_file, Compression::default());
        encoder.write_all(&uncompressed_buf).unwrap();
        encoder.finish().unwrap();

        let result = BmprFile::load_from_file(temp_path);
        assert!(matches!(result, Err(BmprError::ChecksumMismatch { .. })));

        let _ = fs::remove_file(temp_path);
    }
}
