// エントリーポイント用関数は出来るだけコンパクトにする
// それ以外の関数はモジュールに分ける
pub use crate::sayhello::{HelloReply, HelloRequest,HelloService,Connection};
pub mod sayhello;
pub mod mylib;
pub use crate::mylib::load_env;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    load_env();
    
    let mut instance=sayhello::HelloService::new("Gazushige impl2").await?;
    let response=instance.say_hello().await?;
    // レスポンスと HTTP ステータスコードをログに出力
    println!("RESPONSE={:?}", response);
    Ok(())
}


