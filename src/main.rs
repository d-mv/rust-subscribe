use rust_subscribe::run;

/// Main function
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
