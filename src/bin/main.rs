use mtgsdk::api::Types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let types = Types::all().await?;

    println!("{:?}", types);
    Ok(())
}