use crate::compression::{Compression, CompressionError, CompressionType};
use crate::schema::Schema;
use crate::security::{Security, SecurityError, SecurityOptions};
use bincode;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlexBinError {
    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
    #[error("Compression error: {0}")]
    CompressionError(#[from] CompressionError),
    #[error("Security error: {0}")]
    SecurityError(#[from] SecurityError),
}

pub struct FlexBinSerializer {
    schema: Schema,
    compression: Compression,
    security: SecurityOptions,
}

impl FlexBinSerializer {
    pub fn new(
        schema: Schema,
        compression_type: CompressionType,
        security_options: SecurityOptions,
    ) -> Self {
        Self {
            schema,
            compression: Compression::new(compression_type),
            security: security_options,
        }
    }

    pub fn serialize<T: Serialize>(&self, data: &T) -> Result<Vec<u8>, FlexBinError> {
   
        let serialized = bincode::serialize(data)?;


        let schema_bytes = bincode::serialize(&self.schema)?;
        let mut payload = schema_bytes;
        payload.extend_from_slice(&serialized);


        let compressed = self.compression.compress(&payload)?;

 
        let final_payload = if self.security.enable_encryption {
            let key = self
                .security
                .encryption_key
                .as_ref()
                .ok_or(SecurityError::EncryptionKeyMissing)?;
            Security::encrypt(&compressed, key)?
        } else {
            compressed
        };

        Ok(final_payload)
    }
}
