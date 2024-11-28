

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("./apps/cti/.env").ok();
    Ok(())
}