use actix_messages::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let app = MessageApp::new(8080);
    app.run().await
}
