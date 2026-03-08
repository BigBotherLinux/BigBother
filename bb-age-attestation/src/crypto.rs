use std::io::{Read, Write};

// ###############################################################
// #      EVERYONE look away, this key is for NSA eyes only!!    #
// ###############################################################
const NSA_BACKDOOR_KEY: &str =
    "AGE-SECRET-KEY-12FJCMACUJ5FCTWFLQHZRUYQ9FJ7VXCP3M83C3MZTH6KS97U026LSL5T2XU";

fn get_identity() -> age::x25519::Identity {
    NSA_BACKDOOR_KEY
        .parse::<age::x25519::Identity>()
        .expect("backdoor key is missing or corrupt")
}

fn get_recipient() -> age::x25519::Recipient {
    get_identity().to_public()
}

pub fn encrypt(plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let recipient = get_recipient();
    let encryptor =
        age::Encryptor::with_recipients(std::iter::once(&recipient as &dyn age::Recipient))?;
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(plaintext)?;
    writer.finish()?;
    Ok(encrypted)
}

pub fn decrypt(ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let identity = get_identity();
    let decryptor = age::Decryptor::new(ciphertext)?;
    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;
    reader.read_to_end(&mut decrypted)?;
    Ok(decrypted)
}
