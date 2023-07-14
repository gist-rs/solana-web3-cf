use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_qs;
use worker::{Request, Response, Result, RouteContext, Url};

use crate::handlers::memo::get_reference_pubkey_from_receipt;

use super::qr::get_qr_url;

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

struct PayCommand {
    chain_type: String,
    recipient: String,
}

// tested with http://192.168.2.40:8787/pay/solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
pub async fn handle_pay_req(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // command // solana:recipient
    let pay_command = match _ctx.param("command") {
        Some(command) => {
            let mut parts = command.split(':');
            let chain_type = match parts.next() {
                Some(chain_type) => chain_type.to_owned(),
                None => {
                    return Response::error("Expected valid command chain_type".to_string(), 401)
                }
            };
            let recipient = match parts.next() {
                Some(recipient) => recipient.to_owned(),
                None => {
                    return Response::error("Expected valid command recipient".to_string(), 401)
                }
            };

            PayCommand {
                chain_type,
                recipient,
            }
        }
        None => return Response::error("Expected url params".to_string(), 401),
    };

    // search_params // ?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    let url = match req.url() {
        Ok(url) => url,
        Err(_) => return Response::error("Expected valid query".to_string(), 401),
    };

    let maybe_search_params;
    let query_result = match url.query() {
        Some(query) => {
            maybe_search_params = Some(query);
            serde_qs::from_str::<PayQueryParams>(query)
        }

        None => return Response::error("Expected valid query".to_string(), 401),
    };

    let search_params = match maybe_search_params {
        Some(search_params) => search_params,
        None => return Response::error("Expected valid search_params".to_string(), 401),
    };

    let pay_query_params = match query_result {
        Ok(query) => query,
        Err(_) => return Response::error("Expected valid query result".to_string(), 401),
    };

    println!("{pay_query_params:?}");
    let reference = match pay_query_params.reference {
        Some(reference) => reference,
        None => return Response::error("Expected pay_query_params.reference".to_string(), 401),
    };
    let reference_pubkey_from_receipt =
        get_reference_pubkey_from_receipt(&pay_command.recipient, &reference);

    // TODO: add to watch queue
    println!("{reference_pubkey_from_receipt:?}");

    let chain_type = pay_command.chain_type;
    let recipient = pay_command.recipient;

    let url = match Url::from_str(format!("{chain_type}:{recipient}?{search_params}").as_str()) {
        Ok(url) => url,
        Err(_) => return Response::error("Expected valid url".to_string(), 401),
    };

    let qr_url = get_qr_url(&url, 256);
    Response::from_html(format!(
        "<html><head><meta http-equiv=\"Refresh\" content=\"0; URL={url}/\"/></head><body>Redirect to {url} via <img src=\"{qr_url}\"/></body></html>"
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pay() {
        let query = "amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        let query_result = serde_qs::from_str::<PayQueryParams>(query);
        assert_eq!(
            query_result.unwrap(),
            PayQueryParams {
                amount: "0.01".to_string(),
                spl_token: Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()),
                reference: None,
                label: None,
                message: None,
                memo: None,
            }
        );
    }
}
