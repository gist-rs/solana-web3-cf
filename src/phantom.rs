pub const PHANTOM_ENCRYPTION_PUBKEY_KEY_NAME: &str = "PHANTOM_ENCRYPTION_PUBLIC_KEY";

// WIP: Some what wrong implement.
pub fn verify_session_data(encryption_pubkey: &str, session_str: &str) -> anyhow::Result<String> {
    let session_bytes = bs58::decode(session_str)
        .into_vec()
        .expect("ERROR: expect session str.");

    let pubkey_bytes = bs58::decode(encryption_pubkey)
        .into_vec()
        .expect("ERROR: expect pk.");

    let verified_session_data =
        nacl::sign::open(&session_bytes, &pubkey_bytes).expect("ERROR: expect verified session.");

    let session_data = str::from_utf8(&verified_session_data).expect("ERROR: expect session data");

    Ok(session_data.to_owned())
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use crate::web3lite::verify_session_data;

    #[test]
    fn test_parse_session_data() {
        // Load environment variables from .env file
        dotenv().ok();

        // 1. Dynamic input.
        let session_str = env::var("MOCKED_PHANTOM_SESSION").unwrap();

        // 2. Static input.
        let phantom_encryption_pubkey = env::var("PHANTOM_ENCRYPTION_PUBKEY_KEY_NAME").unwrap();
        println!("phantom_encryption_pubkey:{:#?}", phantom_encryption_pubkey);

        // 3. Parse session data.
        assert!(
            !verify_session_data(&phantom_encryption_pubkey, &session_str)
                .unwrap()
                .is_empty()
        )
    }
}
