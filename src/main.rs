extern crate hyper;
extern crate hyper_tls;

use hyper::body::HttpBody;
use hyper::Client;

use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")?;

    let https = hyper_tls::HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = format!("https://api.telegram.org/bot{}/{}", token, "getMe").parse()?;
    let mut response = client.get(uri).await?;

    println!("Response: {}", response.status());

    while let Some(chunk) = response.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}
