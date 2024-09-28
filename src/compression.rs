use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression as FlateCompression};
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    None,
    Zlib,
}

#[derive(Debug, Error)]
pub enum CompressionError {
    #[error("Compression failed")]
    CompressionFailed,
    #[error("Decompression failed")]
    DecompressionFailed,
}

pub enum Compression {
    None,
    Zlib,
}

impl Compression {
    pub fn new(compression_type: CompressionType) -> Self {
        match compression_type {
            CompressionType::None => Compression::None,
            CompressionType::Zlib => Compression::Zlib,
        }
    }

    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match self {
            Compression::None => Ok(data.to_vec()),
            Compression::Zlib => {
                let mut encoder = ZlibEncoder::new(Vec::new(), FlateCompression::default());
                encoder
                    .write_all(data)
                    .map_err(|_| CompressionError::CompressionFailed)?;
                encoder
                    .finish()
                    .map_err(|_| CompressionError::CompressionFailed)
            }
        }
    }

    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, CompressionError> {
        match self {
            Compression::None => Ok(data.to_vec()),
            Compression::Zlib => {
                let mut decoder = ZlibDecoder::new(data);
                let mut decompressed = Vec::new();
                decoder
                    .read_to_end(&mut decompressed)
                    .map_err(|_| CompressionError::DecompressionFailed)?;
                Ok(decompressed)
            }
        }
    }
}
