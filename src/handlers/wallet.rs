use serde::{Deserialize, Serialize};
use serde_json::json;

use solana_web3_wasm::solana_sdk::signature::Keypair;
use worker::{Request, Response, Result, RouteContext};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct PayQueryParams {
    amount: String,
    #[serde(rename = "spl-token")]
    spl_token: Option<String>,
    reference: Option<String>,
    label: Option<String>,
    message: Option<String>,
    memo: Option<String>,
}

pub async fn handle_wallet_req(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match ctx.param("command") {
        Some(command) => {
            match command.as_str() {
                "new" => {
                    let keypair = Keypair::new();
                    let pubkey = keypair.to_base58_string();

                    // Manage private key
                    match ctx.env.secret("MASTER_KEY") {
                        Ok(_key) => {
                            // TODO: encrypt secret with MASTER_KEY and store with key mapping.
                            // todo!();

                            // Expose public key
                            let json = json!({ "pubkey": pubkey });
                            Response::from_json(&json)
                        }
                        Err(error) => Response::error(error.to_string(), 401),
                    }
                }
                "mint" => {
                    todo!()
                }
                _ => Response::error("Invalid command".to_string(), 401),
            }
        }
        None => Response::error("Expect some command".to_string(), 401),
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test_mint() {
        todo!();
    }
}
