use std::{str::FromStr, time::Duration};

use anyhow::bail;
use solana_web3_wasm::{
    core::client::{EndPoint, Web3WasmClient},
    solana_client_wasm::solana_sdk::pubkey::Pubkey,
    solana_client_wasm::{utils::rpc_config::GetConfirmedSignaturesForAddress2Config, WasmClient},
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signature},
};
use worker::Delay;

#[allow(dead_code)]
pub async fn loop_find_reference_by_memo(
    client: &WasmClient,
    reference: &Pubkey,
    memo: String,
) -> anyhow::Result<Signature> {
    // Get latest signature.
    let latest_confirmed_statuses = client
        .get_signatures_for_address(reference)
        .await
        .expect("Expected latest_confirmed_signatures");

    // Already confirm?
    let confirmed_status = latest_confirmed_statuses
        .iter()
        .find(|e| e.memo == Some(memo.to_owned()));
    if let Some(confirmed_status) = confirmed_status {
        return Ok(Signature::from_str(confirmed_status.signature.as_ref())
            .expect("Expected valid signature"));
    }

    // Keep fetch from latest signature.
    let latest_confirmed_signature = latest_confirmed_statuses
        .last()
        .expect("Expected latest_confirmed_signature");

    println!("last know signature: {latest_confirmed_signature:?}");

    // Fetch
    let mut i = 0;
    let max_loops = 20;
    loop {
        let confirmed_signatures = client
            .get_signatures_for_address_with_config(
                reference,
                GetConfirmedSignaturesForAddress2Config {
                    before: Some(
                        Signature::from_str(latest_confirmed_signature.signature.as_ref())
                            .expect("Expected valid signature"),
                    ),
                    limit: Some(100),
                    until: None,
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
            .await;

        match confirmed_signatures {
            Ok(confirmed_signatures) => {
                // Already confirm?
                let confirmed_status = confirmed_signatures
                    .iter()
                    .find(|e| e.memo == Some(memo.to_owned()));
                if let Some(confirmed_status) = confirmed_status {
                    return Ok(Signature::from_str(confirmed_status.signature.as_ref())
                        .expect("Expected valid signature"));
                }

                Delay::from(Duration::from_secs(5)).await;
                i += 1;
                println!("{}", i);
            }
            Err(error) => {
                bail!(error);
            }
        }
        if i == max_loops {
            bail!("Timeout {} loop", max_loops);
        }
    }
}

#[allow(dead_code)]
pub async fn loop_find_reference_by_memo_from_endpoint(
    endpoint: &EndPoint,
    reference: &str,
    memo: String,
) -> anyhow::Result<Signature> {
    let reference = Pubkey::from_str(reference).expect("Expected reference as Pubkey");
    let client = Web3WasmClient::new(endpoint);

    loop_find_reference_by_memo(&client, &reference, memo).await
}

#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    async fn test_loop_find_reference_by_memo_from_endpoint() {
        todo!()
    }
}
