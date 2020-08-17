#[derive(Clone, PartialEq, ::prost::Message)]
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
pub struct Status {
  #[prost(bool, tag = "1")]
  pub wip: bool,
}
#[doc = r" Generated server implementations."]
pub mod hist_chart_server {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with HistChartServer."]
  #[async_trait]
  pub trait HistChart: Send + Sync + 'static {
    #[doc = "Server streaming response type for the sync method."]
    type syncStream: Stream<Item = Result<super::HistChartProg, tonic::Status>>
      + Send
      + Sync
      + 'static;
    async fn sync(
      &self,
      request: tonic::Request<super::HistChartFetchReq>,
    ) -> Result<tonic::Response<Self::syncStream>, tonic::Status>;
    async fn status(
      &self,
      request: tonic::Request<()>,
    ) -> Result<tonic::Response<super::Status>, tonic::Status>;
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
      request: tonic::Request<()>,
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
            tonic::server::ServerStreamingService<super::HistChartFetchReq>
            for syncSvc<T>
          {
            type Response = super::HistChartProg;
            type ResponseStream = T::syncStream;
            type Future =
              BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
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
            let interceptor = inner.1;
            let inner = inner.0;
            let method = syncSvc(inner);
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
        "/historical.HistChart/status" => {
          #[allow(non_camel_case_types)]
          struct statusSvc<T: HistChart>(pub Arc<T>);
          impl<T: HistChart> tonic::server::UnaryService<()> for statusSvc<T> {
            type Response = super::Status;
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).status(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = statusSvc(inner);
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
          impl<T: HistChart> tonic::server::UnaryService<()> for stopSvc<T> {
            type Response = ();
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
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
