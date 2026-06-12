use tokio::net::TcpListener;

mod api;
mod usb;
mod models;
mod server;

const ADDR: &str = "127.0.0.1:5151";

#[tokio::main]
async fn main() {
    let app = server::main_router();

    let listener = TcpListener::bind(ADDR)
        .await
        .expect("Failed to create a TCP listener!");

    println!("{}", format!("Server starts on {}", ADDR));

    axum::serve(listener, app)
        .await
        .expect("Failed to start a web-server!");
}
