use bb_age_attestation::dbus_interface::AgeAttestationService;
use bb_age_attestation::types::{AGE_ATTESTATION_INTERFACE, AGE_ATTESTATION_OBJECT_PATH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::system().await?;

    connection
        .object_server()
        .at(AGE_ATTESTATION_OBJECT_PATH, AgeAttestationService)
        .await?;

    connection.request_name(AGE_ATTESTATION_INTERFACE).await?;

    println!("Listening on D-Bus for age attestation requests");

    // Run forever
    std::future::pending::<()>().await;
    Ok(())
}
