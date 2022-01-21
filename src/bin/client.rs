use detective::data::suspect::root_as_suspect;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:8000/info").await?;
    let buf = resp.bytes().await?;
    let suspect = root_as_suspect(&buf)?;
    println!("{:#?}", suspect);

    Ok(())
}
