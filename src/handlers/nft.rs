use std::str::FromStr;

use super::guard::{extract_web3_token, Web3Token};
use anyhow::bail;
use reqwest::Url;

use solana_web3_wasm::core::client::ClusterId;
use solana_web3_wasm::mpl_token_metadata::state::Metadata;
use solana_web3_wasm::{info::nft::NftInformation, solana_sdk::pubkey::Pubkey};
use worker::{console_log, Request, Response, Result, RouteContext};

pub async fn handle_nft_req(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let web3_token_result = extract_web3_token(&req);
    console_log!("web3_token_result: {web3_token_result:?}");

    Response::ok("TODO")

    // match web3_token_result {
    //     Ok(web3_token) => handle_nft_web3_token(&req, &ctx, web3_token).await,
    //     Err(err) => Response::error(format!("${err}"), 403),
    // }
}

#[allow(dead_code)]
async fn fetch(url: String) -> anyhow::Result<Result<Response>> {
    console_log!("url: {url:?}");

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let result = Response::from_bytes(bytes.to_vec());
    Ok(result)

    // let body = reqwest::get("https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA")
    //     .await?
    //     .text()
    //     .await?;

    // console_log!("body: {body:?}");

    // let result = Response::from_html("ok");
    // Ok(result)
}

#[allow(dead_code)]
fn get_query_param_value(url: &Url, key_name: &str) -> Option<String> {
    let query_params = url
        .query_pairs()
        .into_owned()
        .collect::<Vec<(String, String)>>();

    query_params
        .iter()
        .find(|x| x.0 == key_name)
        .map(|x| x.1.to_string())
}

#[allow(dead_code)]
async fn get_kv_text(ctx: &RouteContext<()>, namespace: &str, key_name: &str) -> Option<String> {
    // Some("https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA".to_owned())
    let kv = ctx.kv(namespace);
    match kv {
        Ok(kv_store) => match kv_store.get(key_name).text().await {
            Ok(value) => value,
            Err(_) => None,
        },
        Err(_) => None,
    }
}

#[allow(dead_code)]
async fn get_user_nft_metadata(
    cluster_id: &ClusterId,
    wallet_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
) -> anyhow::Result<Metadata> {
    println!("get_user_nft_metadata: {wallet_pubkey:?}");
    let nft_information = NftInformation::new_from_str(cluster_id.to_string().as_str())
        .expect("expect valid cluster");

    println!("find_nfts_by_mints: {mint_pubkey:?}");
    let mut nfts = nft_information
        .find_nfts_by_mints(wallet_pubkey, &[*mint_pubkey])
        .await?;

    let metadata = nfts
        .remove(&mint_pubkey.to_string())
        .expect("expect metadata");

    Ok(metadata)
}

#[allow(dead_code)]
pub async fn get_user_nft_metadata_from_url(
    url: &Url,
    mint_pubkey: &Pubkey,
    web3_token: &Web3Token,
) -> anyhow::Result<Metadata> {
    let maybe_cluster_str = get_query_param_value(url, "cluster");
    let cluster_id = match maybe_cluster_str {
        Some(cluster_str) => match cluster_str.as_str() {
            "devnet" => ClusterId::Devnet,
            _ => ClusterId::Mainnet,
        },
        None => ClusterId::Mainnet,
    };

    println!("handle_nft_web3_token: {cluster_id:?}");

    // test
    // let address = get_associated_token_address(mint_pubkey, mint_pubkey);
    // println!("address: {cluster_id:?}");

    // 1. TODO: Validate web token
    let wallet_pubkey = Pubkey::from_str(&web3_token.wallet_address)?;

    // 2. Get NFT for user if has
    let nft_metadata = match get_user_nft_metadata(&cluster_id, &wallet_pubkey, mint_pubkey).await {
        Ok(metadata) => metadata,
        Err(_) => bail!("NFT not found."),
    };

    println!("nft_metadata:{nft_metadata:#?}");

    Ok(nft_metadata)

    // // 3. Get KV
    // let value: Option<String> = get_kv_text(ctx, "NFT", &mint_address).await;

    // let url = match value {
    //     Some(value) => value,
    //     None => return Err(Error::from("ERROR: expect value.")),
    // };

    // url

    // 2. Fetch content from url
    // match fetch(url).await {
    //     Ok(result) => result,
    //     Err(error) => return Err(Error::from(error.to_string())),
    // }
}

#[cfg(test)]
mod test {
    use super::*;
    use urlencoding::decode;

    #[tokio::test]
    async fn test_get_user_nft_metadata_from_url() {
        let url = Url::from_str(
            "https://gist.rs/nft/2jxpnS9jy9RmYsopuXXeMQP8Av81JtSSTMjz4Qnb9acr?cluser=devnet",
        )
        .unwrap();

        let cookie_str = decode("cat9ZgXRQA3yCRCNaFyswDqZhQuDsJEvVnfzWfdWNdX%7C67qiYTcmK7deQ2Y4MTc31X6gwxM1w5HzufyH2KvZ3DTjXibwNyHgQi8m2ZAVxx2ios15c8Zq13dNrSZ1qcFQ2GsPpZCmkAKuW3VPSdxSDcw38XS6YD5ve2FqNxTHXRrpwTApWXP8vXjpvCSMBughKpz6JsZPKH127yXXdduF9ADurL29G3xwmrKdA92qbeYdBQFBa24jE31XvfiQmN2ScYubcrAx%7C%7B%22app_url%22%3A%22https%3A%2F%2Fgist.rs%22%2C%22timestamp%22%3A1670600698494%2C%22chain%22%3A%22solana%22%2C%22cluster%22%3A%22mainnet-beta%22%7D").unwrap();
        let web3_token = Web3Token::from_str(&cookie_str).unwrap();
        let mint_pubkey = Pubkey::from_str("2jxpnS9jy9RmYsopuXXeMQP8Av81JtSSTMjz4Qnb9acr").unwrap();

        let url = get_user_nft_metadata_from_url(&url, &mint_pubkey, &web3_token)
            .await
            .unwrap();
        dbg!(url);
    }
}
