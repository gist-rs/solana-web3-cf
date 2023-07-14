use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use worker::Url;

pub fn get_qr_url(url: &Url, size: usize) -> String {
    let url = utf8_percent_encode(url.as_str(), NON_ALPHANUMERIC);
    format!("https://chart.googleapis.com/chart?cht=qr&chld=M|2&chs={size}x{size}&chl={url}")
}
