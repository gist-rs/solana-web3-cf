use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_qs;
use worker::{Headers, Request, Response, Result, RouteContext, Url};

use crate::handlers::memo::get_reference_pubkey_from_receipt;
use fast_qr::{
    convert::{image::ImageBuilder, Builder, Shape},
    QRBuilder, ECL,
};

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
    chain_id: String,
    recipient: String,
}

// tested with http://192.168.2.40:8787/pay/solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
pub async fn handle_pay_req(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // command // solana:recipient
    let pay_command = match ctx.param("command") {
        Some(command) => {
            let mut parts = command.split(':');
            let chain_id = match parts.next() {
                Some(chain_id) => chain_id.to_owned(),
                None => return Response::error("Expect valid command chain_id".to_string(), 401),
            };
            let recipient = match parts.next() {
                Some(recipient) => recipient.to_owned(),
                None => return Response::error("Expect valid command recipient".to_string(), 401),
            };

            PayCommand {
                chain_id,
                recipient,
            }
        }
        None => return Response::error("Expect url params".to_string(), 401),
    };

    // search_params // ?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    let url = match req.url() {
        Ok(url) => url,
        Err(_) => return Response::error("Expect valid query".to_string(), 401),
    };

    let maybe_search_params;
    let query_result = match url.query() {
        Some(query) => {
            maybe_search_params = Some(query);
            serde_qs::from_str::<PayQueryParams>(query)
        }

        None => return Response::error("Expect valid query".to_string(), 401),
    };

    let search_params = match maybe_search_params {
        Some(search_params) => search_params,
        None => return Response::error("Expect valid search_params".to_string(), 401),
    };

    let pay_query_params = match query_result {
        Ok(query) => query,
        Err(_) => return Response::error("Expect valid query result".to_string(), 401),
    };

    println!("pay_query_params: {pay_query_params:?}");

    // Seem like i try to derive memo_pubkey(=reference_pubkey) from reference(=receipt) here
    // But not fin yet, will need loop_find_reference_by_memo but still 2 hops more.
    // And `recipient_pubkey+reference_str` as a seed is not enough?
    match pay_query_params.reference {
        Some(reference) => {
            let reference_pubkey_from_receipt =
                get_reference_pubkey_from_receipt(&pay_command.recipient, &reference);

            // TODO: add to watch queue
            println!("reference_pubkey_from_receipt: {reference_pubkey_from_receipt:?}");
        }
        None => (), //return Response::error("Expect pay_query_params.reference".to_string(), 401),
    };

    let chain_id = pay_command.chain_id;
    let recipient = pay_command.recipient;

    // let url = match Url::from_str( "solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=0.01&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v") {
    let url = match Url::from_str(format!("{chain_id}:{recipient}?{search_params}").as_str()) {
        Ok(url) => url,
        Err(_) => return Response::error("Expect valid url".to_string(), 401),
    };

    let qrcode = QRBuilder::new(url.to_string()).ecl(ECL::M).build().unwrap();

    let bytes = ImageBuilder::default()
        .shape(Shape::Square)
        .fit_width(400)
        .background_color([255, 255, 255, 255]) // transparency
        .to_bytes(&qrcode)
        .unwrap();

    let mut headers = Headers::new();
    headers.set("content-type", "image/png")?;
    let response = Response::from_bytes(bytes)?;

    Ok(response.with_headers(headers))

    // Response::from_html(format!(
    //     "<html><head><meta http-equiv=\"Refresh\" content=\"0; URL={url}/\"/></head><body>Redirect to {url} via <img src=\"{qr_url}\"/></body></html>"
    // ))
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
