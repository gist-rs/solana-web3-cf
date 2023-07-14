use std::str::FromStr;

use solana_web3_wasm::solana_extra_wasm::program::spl_memo::ID;
use solana_web3_wasm::solana_sdk::pubkey::Pubkey;

pub fn get_reference_pubkey_from_receipt(
    recipient_pubkey_str: &str,
    reference_str: &str,
) -> anyhow::Result<Pubkey> {
    let recipient_pubkey = Pubkey::from_str(recipient_pubkey_str).expect("Expected valid pubkey");
    let (pubkey, _) = Pubkey::find_program_address(
        &[(recipient_pubkey.as_ref()), reference_str.as_bytes()],
        &ID,
    );

    Ok(pubkey)
}
