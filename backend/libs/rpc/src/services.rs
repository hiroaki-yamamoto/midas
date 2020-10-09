#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfo {
  #[prost(string, tag = "1")]
  pub id: std::string::String,
  #[prost(enumeration = "Strategy", tag = "2")]
  pub strategy: i32,
  #[prost(string, tag = "3")]
  pub name: std::string::String,
  #[prost(string, tag = "4")]
  pub base_currency: std::string::String,
  #[prost(string, tag = "5")]
  pub desc: std::string::String,
  #[prost(string, tag = "6")]
  pub config: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentPosition {
  #[prost(string, tag = "1")]
  pub id: std::string::String,
  #[prost(string, tag = "2")]
  pub bot_id: std::string::String,
  #[prost(string, tag = "3")]
  pub symbol: std::string::String,
  #[prost(double, tag = "4")]
  pub trading_amount: f64,
  #[prost(double, tag = "5")]
  pub profit_amount: f64,
  #[prost(double, tag = "6")]
  pub profit_percent: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfoList {
  #[prost(message, repeated, tag = "1")]
  pub bots: ::std::vec::Vec<BotInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotInfoListRequest {
  #[prost(int64, tag = "1")]
  pub offset: i64,
  #[prost(int64, tag = "2")]
  pub limit: i64,
}
#[derive(
  Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Strategy {
  Trailing = 0,
}
#[doc = r" Generated client implementations."]
pub mod bot_manager_client {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  pub struct BotManagerClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl BotManagerClient<tonic::transport::Channel> {
    #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
      D: std::convert::TryInto<tonic::transport::Endpoint>,
      D::Error: Into<StdError>,
    {
      let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
      Ok(Self::new(conn))
    }
  }
  impl<T> BotManagerClient<T>
  where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::ResponseBody: Body + HttpBody + Send + 'static,
    T::Error: Into<StdError>,
    <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
  {
    pub fn new(inner: T) -> Self {
      let inner = tonic::client::Grpc::new(inner);
      Self { inner }
    }
    pub fn with_interceptor(
      inner: T,
      interceptor: impl Into<tonic::Interceptor>,
    ) -> Self {
      let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
      Self { inner }
    }
    pub async fn list_bot_info(
      &mut self,
      request: impl tonic::IntoRequest<super::BotInfoListRequest>,
    ) -> Result<tonic::Response<super::BotInfoList>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static(
        "/services.BotManager/ListBotInfo",
      );
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
  impl<T: Clone> Clone for BotManagerClient<T> {
    fn clone(&self) -> Self {
      Self {
        inner: self.inner.clone(),
      }
    }
  }
  impl<T> std::fmt::Debug for BotManagerClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "BotManagerClient {{ ... }}")
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod bot_manager_server {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with BotManagerServer."]
  #[async_trait]
  pub trait BotManager: Send + Sync + 'static {
    async fn list_bot_info(
      &self,
      request: tonic::Request<super::BotInfoListRequest>,
    ) -> Result<tonic::Response<super::BotInfoList>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct BotManagerServer<T: BotManager> {
    inner: _Inner<T>,
  }
  struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
  impl<T: BotManager> BotManagerServer<T> {
    pub fn new(inner: T) -> Self {
      let inner = Arc::new(inner);
      let inner = _Inner(inner, None);
      Self { inner }
    }
    pub fn with_interceptor(
      inner: T,
      interceptor: impl Into<tonic::Interceptor>,
    ) -> Self {
      let inner = Arc::new(inner);
      let inner = _Inner(inner, Some(interceptor.into()));
      Self { inner }
    }
  }
  impl<T, B> Service<http::Request<B>> for BotManagerServer<T>
  where
    T: BotManager,
    B: HttpBody + Send + Sync + 'static,
    B::Error: Into<StdError> + Send + 'static,
  {
    type Response = http::Response<tonic::body::BoxBody>;
    type Error = Never;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
      &mut self,
      _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
      Poll::Ready(Ok(()))
    }
    fn call(&mut self, req: http::Request<B>) -> Self::Future {
      let inner = self.inner.clone();
      match req.uri().path() {
        "/services.BotManager/ListBotInfo" => {
          #[allow(non_camel_case_types)]
          struct ListBotInfoSvc<T: BotManager>(pub Arc<T>);
          impl<T: BotManager>
            tonic::server::UnaryService<super::BotInfoListRequest>
            for ListBotInfoSvc<T>
          {
            type Response = super::BotInfoList;
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::BotInfoListRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).list_bot_info(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = ListBotInfoSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = if let Some(interceptor) = interceptor {
              tonic::server::Grpc::with_interceptor(codec, interceptor)
            } else {
              tonic::server::Grpc::new(codec)
            };
            let res = grpc.unary(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        _ => Box::pin(async move {
          Ok(
            http::Response::builder()
              .status(200)
              .header("grpc-status", "12")
              .body(tonic::body::BoxBody::empty())
              .unwrap(),
          )
        }),
      }
    }
  }
  impl<T: BotManager> Clone for BotManagerServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self { inner }
    }
  }
  impl<T: BotManager> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone(), self.1.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: BotManager> tonic::transport::NamedService for BotManagerServer<T> {
    const NAME: &'static str = "services.BotManager";
  }
}
