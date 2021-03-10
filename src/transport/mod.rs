use tonic::{Request, Response, Status};

use echo::{EchoRequest, EchoResponse};
use echo::echo_server::Echo;

pub mod echo {
	tonic::include_proto!("echo.v1");

  // pub(crate) const FILE_DESCRIPTOR_SET: &'static [u8] =
  //   tonic::include_file_descriptor_set!("echo_descriptor");
}

#[derive(Debug, Default)]
pub struct EchoService;

#[tonic::async_trait]
impl Echo for EchoService {
  async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
    dbg!(request);

    let res = EchoResponse{
      message: "Hello".to_string(),
    };

    Ok(Response::new(res))
  }
}
