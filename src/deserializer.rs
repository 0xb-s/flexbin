use crate::compression::{CompressionError, CompressionType};
use crate::schema::Schema;
use crate::security::{Security, SecurityError, SecurityOptions};
use bincode;

use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlexBinError {
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] bincode::Error),
    #[error("Compression error: {0}")]
    CompressionError(#[from] CompressionError),
    #[error("Security error: {0}")]
    SecurityError(#[from] SecurityError),
}

pub struct FlexBinDeserializer {
    compression: crate::compression::Compression,
    security: SecurityOptions,
}

impl FlexBinDeserializer {
    pub fn new(compression_type: CompressionType, security_options: SecurityOptions) -> Self {
        Self {
            compression: crate::compression::Compression::new(compression_type),
            security: security_options,
        }
    }

    pub fn deserialize<T: DeserializeOwned>(
        &self,
        data: &[u8],
    ) -> Result<(Schema, T), FlexBinError> {
 
        let decrypted = if self.security.enable_encryption {
            let key = self
                .security
                .encryption_key
                .as_ref()
                .ok_or(SecurityError::DecryptionFailed)?;
            Security::decrypt(data, key)?
        } else {
            data.to_vec()
        };


        let decompressed = self.compression.decompress(&decrypted)?;

  
        let mut cursor = std::io::Cursor::new(decompressed);
        let schema: Schema = bincode::deserialize_from(&mut cursor)?;
        let data: T = bincode::deserialize_from(&mut cursor)?;

        Ok((schema, data))
    }
}
