#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pantheon::new(8080)?.await
}
