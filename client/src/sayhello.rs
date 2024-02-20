use core::str;

// protobufで生成された関数はここに
pub use hello_world::{HelloReply, HelloRequest};
use tonic::{ Response,Result};
pub use hello_world::greeter_client::GreeterClient;
use tonic::transport::{Channel,Error};
pub use crate::mylib::env_var;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}
pub trait Connection {
    fn new(name: &str) -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;
    fn get_connection(&mut self)->impl std::future::Future<Output = Result<GreeterClient<Channel>, Error>> + Send;
    fn set_request(&mut self, name:&str);
    fn say_hello(&mut self)-> impl std::future::Future<Output = Result<Response<HelloReply>, tonic::Status>> + Send;
}
pub struct HelloService{
    client:GreeterClient<Channel>,
    request: Option<tonic::Request<HelloRequest>>,
}
impl Connection for HelloService{
    async fn new(name: &str) -> Result<Self, Error> {
        let mut service = HelloService {
            client: GreeterClient::connect(format!("{}://{}:{}",env_var("Protocol"),env_var("ServerIP"),env_var("Port"))).await?,
            request: None,
        };
        service.set_request(name);
        Ok(service)
    }
    // サーバ接続を確立してclientを返す
    async fn get_connection(&mut self)->Result<GreeterClient<Channel>, Error>{
        self.client = GreeterClient::connect(format!("{}://{}:{}",env_var("Protocol"),env_var("ServerIP"),env_var("Port"))).await?;

        Ok(self.client.clone())
    }
    // リクエストをセットする。
    fn set_request(&mut self, name: &str) {
        self.request = Some(tonic::Request::new(
            hello_world::HelloRequest {
                name: name.to_string(),
            }
        ));
    }
    // リクエストを送信する。
    async fn say_hello(&mut self)-> Result<Response<HelloReply>, tonic::Status>{
        let request = self.request.take().expect("Request not set");
        let response=self.client.say_hello(request).await?;

        Ok(response)
    }
}
