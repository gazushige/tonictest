//汎用的なライブラリ

use dotenv::dotenv;

pub struct GrpcResponse{
    pub status: String,
    pub body: String,
}

// .envファイルを読み込む関数
pub fn load_env() {
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
pub fn env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{}の環境変数が見つかりません", key))
}