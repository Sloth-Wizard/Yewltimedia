use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::models::fetch_states::*;

pub async fn fetch_html(url: &str) -> Result<String, FetchError> {
    let mut options = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &options)?;

    request
        .headers()
        .set("Accept", "text/html")?;
    
    let window = gloo::utils::window();
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `response_value` is a `Response` object.
    assert!(response_value.is_instance_of::<Response>());
    let response: Response = response_value.dyn_into().unwrap();

    let text = JsFuture::from(response.text()?).await?;

    Ok(text.as_string().unwrap())
}
