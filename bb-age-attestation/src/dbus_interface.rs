use crate::storage;
use crate::types::AgeBracket;
use zbus::object_server::SignalEmitter;
use zbus::{fdo, interface};

pub struct AgeAttestationService;

#[interface(name = "org.bigbother.AgeAttestation1")]
impl AgeAttestationService {
    /// Returns the age bracket for the calling user.
    /// Returns empty string if no attestation is on file.
    async fn get_age_bracket(&self) -> fdo::Result<String> {
        match storage::load().await {
            Ok(Some(bracket)) => Ok(bracket.as_str().to_string()),
            Ok(None) => Ok(String::new()),
            Err(e) => Err(fdo::Error::Failed(format!("Decryption failure: {e}"))),
        }
    }

    /// Set the age bracket for the calling user.
    /// `age` is the self-attested age in years.
    async fn set_age(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        age: u8,
    ) -> fdo::Result<String> {
        let bracket = AgeBracket::from(age);

        storage::store(bracket)
            .await
            .map_err(|e| fdo::Error::Failed(format!("Failed to store attestation: {e}")))?;

        Self::age_bracket_changed(&emitter, bracket.as_str())
            .await
            .ok();

        Ok(bracket.as_str().to_string())
    }

    /// Signal emitted when an age bracket changes.
    #[zbus(signal)]
    async fn age_bracket_changed(emitter: &SignalEmitter<'_>, bracket: &str) -> zbus::Result<()>;
}
