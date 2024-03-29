// lib.rsファイルのモジュールをインポート
mod mylib;
use mylib::MyGreeter;
use tonic::transport::Server;
use mylib::hello_world;
use hello_world::greeter_server::GreeterServer;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();
    let addr = format!("{}:{}",env_var("ServerIP"),env_var("Port")).parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
// .envファイルを読み込む関数
fn load_env() {
    // .env ファイルのパスを取得
    let dotenv_path = ".env";

    // .env ファイルが存在するかを確認
    if let Err(_) = std::fs::metadata(dotenv_path) {
        // .env ファイルが存在しない場合、panic を起こす
        panic!("'.env' ファイルが見つかりません。アプリケーションを実行するには、'.env' ファイルが必要です。");
    }
    dotenv().ok();
}
// keyをもとに環境変数を取得する関数
fn env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{}の環境変数が見つかりません", key))
}