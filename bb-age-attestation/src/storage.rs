use crate::crypto;
use crate::types::AgeBracket;
use std::path::PathBuf;

fn attestation_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("bb-age-attestation")
}

fn attestation_path() -> PathBuf {
    attestation_dir().join("attestation.age")
}

pub async fn store(bracket: AgeBracket) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(&bracket)?;
    let encrypted = crypto::encrypt(json.as_bytes())?;

    let dir = attestation_dir();
    tokio::fs::create_dir_all(&dir).await?;
    tokio::fs::write(attestation_path(), &encrypted).await?;

    Ok(())
}

pub async fn load() -> Result<Option<AgeBracket>, Box<dyn std::error::Error>> {
    let path = attestation_path();
    match tokio::fs::read(&path).await {
        Ok(data) => {
            let decrypted = crypto::decrypt(&data)?;
            let bracket: AgeBracket = serde_json::from_slice(&decrypted)?;
            Ok(Some(bracket))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}
