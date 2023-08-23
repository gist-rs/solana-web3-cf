use fast_qr::{
    convert::{
        image::{ImageBuilder, ImageError},
        Builder, Shape,
    },
    QRBuilder, ECL,
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use worker::{console_log, Headers, Request, Response, RouteContext, Url};

#[allow(dead_code)]
pub fn get_google_qr_url(url: &Url, size: usize) -> String {
    let url = utf8_percent_encode(url.as_str(), NON_ALPHANUMERIC);
    format!("https://chart.googleapis.com/chart?cht=qr&chld=M|2&chs={size}x{size}&chl={url}")
}

pub fn get_qr_m_png(text: &str, size: u32) -> Result<Vec<u8>, ImageError> {
    // QR
    let qrcode = QRBuilder::new(text).ecl(ECL::M).build().unwrap();

    // PNG
    ImageBuilder::default()
        .shape(Shape::Square)
        .fit_width(size)
        .background_color([255, 255, 255, 255])
        .to_bytes(&qrcode)
}

// http://127.0.0.1:8787/qr/512/solana:gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq?amount=1.23&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
// http://127.0.0.1:8787/qr/512/solana:mvines9iiHiQTysrwkJjGf2gb9Ex9jXJX8ns3qwf2kN?amount=1.23&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&label=katopz&message=Hat%20tip!🦀&memo=OrderId12345
pub async fn handle_qr_req(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let size = match ctx.param("size") {
        Some(size) => match size.parse::<u32>() {
            Ok(size32) => size32,
            Err(_err) => return Response::error("Expect valid size".to_string(), 401),
        },
        None => 512u32,
    };

    // data
    let qr_data = match ctx.param("data") {
        Some(data) => data,
        None => return Response::error("Expect url params".to_string(), 401),
    };

    // Params
    let url = match req.url() {
        Ok(url) => url.query().unwrap_or("").to_owned(),
        Err(_) => return Response::error("Expect valid query".to_string(), 401),
    };

    // QR_PNG
    let all_data = format!("{qr_data}?{}", url);
    console_log!("{}", all_data);

    let qr_png: Result<Vec<u8>, ImageError> = get_qr_m_png(all_data.as_str(), size);
    match qr_png {
        Ok(bytes) => {
            // Response
            let mut headers = Headers::new();
            headers.set("content-type", "image/png")?;
            let response = Response::from_bytes(bytes)?;

            Ok(response.with_headers(headers))
        }
        Err(err) => match err {
            ImageError::IoError(err) => Response::error(err.to_string(), 500),
            ImageError::ImageError(err_string) => Response::error(err_string, 500),
            ImageError::EncodingError(err_string) => Response::error(err_string, 500),
        },
    }
}
