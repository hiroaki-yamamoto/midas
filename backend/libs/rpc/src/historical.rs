#[derive(
  Clone, PartialEq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize,
)]
pub struct HistChartProg {
  #[prost(string, tag = "1")]
  pub symbol: std::string::String,
  #[prost(int64, tag = "2")]
  pub num_symbols: i64,
  #[prost(int64, tag = "3")]
  pub cur_symbol_num: i64,
  #[prost(int64, tag = "4")]
  pub num_objects: i64,
  #[prost(int64, tag = "5")]
  pub cur_object_num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistChartFetchReq {
  #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
  pub exchange: i32,
  #[prost(string, repeated, tag = "2")]
  pub symbols: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
  #[prost(enumeration = "super::entities::Exchanges", repeated, tag = "1")]
  pub exchanges: ::std::vec::Vec<i32>,
}
#[doc = r" Generated client implementations."]
pub mod hist_chart_client {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  pub struct HistChartClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl HistChartClient<tonic::transport::Channel> {
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
  impl<T> HistChartClient<T>
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
    pub async fn sync(
      &mut self,
      request: impl tonic::IntoRequest<super::HistChartFetchReq>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path =
        http::uri::PathAndQuery::from_static("/historical.HistChart/sync");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn subscribe(
      &mut self,
      request: impl tonic::IntoRequest<()>,
    ) -> Result<
      tonic::Response<tonic::codec::Streaming<super::HistChartProg>>,
      tonic::Status,
    > {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path =
        http::uri::PathAndQuery::from_static("/historical.HistChart/subscribe");
      self
        .inner
        .server_streaming(request.into_request(), path, codec)
        .await
    }
    pub async fn stop(
      &mut self,
      request: impl tonic::IntoRequest<super::StopRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path =
        http::uri::PathAndQuery::from_static("/historical.HistChart/stop");
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
  impl<T: Clone> Clone for HistChartClient<T> {
    fn clone(&self) -> Self {
      Self {
        inner: self.inner.clone(),
      }
    }
  }
  impl<T> std::fmt::Debug for HistChartClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "HistChartClient {{ ... }}")
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod hist_chart_server {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with HistChartServer."]
  #[async_trait]
  pub trait HistChart: Send + Sync + 'static {
    async fn sync(
      &self,
      request: tonic::Request<super::HistChartFetchReq>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    #[doc = "Server streaming response type for the subscribe method."]
    type subscribeStream: Stream<Item = Result<super::HistChartProg, tonic::Status>>
      + Send
      + Sync
      + 'static;
    async fn subscribe(
      &self,
      request: tonic::Request<()>,
    ) -> Result<tonic::Response<Self::subscribeStream>, tonic::Status>;
    async fn stop(
      &self,
      request: tonic::Request<super::StopRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct HistChartServer<T: HistChart> {
    inner: _Inner<T>,
  }
  struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
  impl<T: HistChart> HistChartServer<T> {
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
  impl<T, B> Service<http::Request<B>> for HistChartServer<T>
  where
    T: HistChart,
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
        "/historical.HistChart/sync" => {
          #[allow(non_camel_case_types)]
          struct syncSvc<T: HistChart>(pub Arc<T>);
          impl<T: HistChart>
            tonic::server::UnaryService<super::HistChartFetchReq>
            for syncSvc<T>
          {
            type Response = ();
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::HistChartFetchReq>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).sync(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = syncSvc(inner);
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
        "/historical.HistChart/subscribe" => {
          #[allow(non_camel_case_types)]
          struct subscribeSvc<T: HistChart>(pub Arc<T>);
          impl<T: HistChart> tonic::server::ServerStreamingService<()>
            for subscribeSvc<T>
          {
            type Response = super::HistChartProg;
            type ResponseStream = T::subscribeStream;
            type Future =
              BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).subscribe(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1;
            let inner = inner.0;
            let method = subscribeSvc(inner);
            let codec = tonic::codec::ProstCodec::default();
            let mut grpc = if let Some(interceptor) = interceptor {
              tonic::server::Grpc::with_interceptor(codec, interceptor)
            } else {
              tonic::server::Grpc::new(codec)
            };
            let res = grpc.server_streaming(method, req).await;
            Ok(res)
          };
          Box::pin(fut)
        }
        "/historical.HistChart/stop" => {
          #[allow(non_camel_case_types)]
          struct stopSvc<T: HistChart>(pub Arc<T>);
          impl<T: HistChart> tonic::server::UnaryService<super::StopRequest>
            for stopSvc<T>
          {
            type Response = ();
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::StopRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).stop(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = stopSvc(inner);
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
  impl<T: HistChart> Clone for HistChartServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self { inner }
    }
  }
  impl<T: HistChart> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone(), self.1.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: HistChart> tonic::transport::NamedService for HistChartServer<T> {
    const NAME: &'static str = "historical.HistChart";
  }
}
