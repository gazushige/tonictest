// 必要なモジュールのインポート
use tonic::{ Request, Response, Status};
use hello_world::greeter_server::Greeter;
use hello_world::{HelloReply, HelloRequest};

// サービスロジックを定義する構造体
#[derive(Debug, Default)]
pub struct MyGreeter {}
// サーバ起動に必要なモジュールを公開する
pub mod hello_world {
    tonic::include_proto!("helloworld");
}
#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

