#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshRequest {
  #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
  pub exchange: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
  #[prost(enumeration = "super::entities::Exchanges", tag = "1")]
  pub exchange: i32,
  #[prost(string, tag = "2")]
  pub status: std::string::String,
  #[prost(string, repeated, tag = "3")]
  pub symbols: ::std::vec::Vec<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
  #[prost(message, repeated, tag = "1")]
  pub symbols: ::std::vec::Vec<super::entities::SymbolInfo>,
}
#[doc = r" Generated client implementations."]
pub mod symbol_client {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  pub struct SymbolClient<T> {
    inner: tonic::client::Grpc<T>,
  }
  impl SymbolClient<tonic::transport::Channel> {
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
  impl<T> SymbolClient<T>
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
    pub async fn refresh(
      &mut self,
      request: impl tonic::IntoRequest<super::RefreshRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/symbol.Symbol/refresh");
      self.inner.unary(request.into_request(), path, codec).await
    }
    pub async fn query(
      &mut self,
      request: impl tonic::IntoRequest<super::QueryRequest>,
    ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status> {
      self.inner.ready().await.map_err(|e| {
        tonic::Status::new(
          tonic::Code::Unknown,
          format!("Service was not ready: {}", e.into()),
        )
      })?;
      let codec = tonic::codec::ProstCodec::default();
      let path = http::uri::PathAndQuery::from_static("/symbol.Symbol/query");
      self.inner.unary(request.into_request(), path, codec).await
    }
  }
  impl<T: Clone> Clone for SymbolClient<T> {
    fn clone(&self) -> Self {
      Self {
        inner: self.inner.clone(),
      }
    }
  }
  impl<T> std::fmt::Debug for SymbolClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "SymbolClient {{ ... }}")
    }
  }
}
#[doc = r" Generated server implementations."]
pub mod symbol_server {
  #![allow(unused_variables, dead_code, missing_docs)]
  use tonic::codegen::*;
  #[doc = "Generated trait containing gRPC methods that should be implemented for use with SymbolServer."]
  #[async_trait]
  pub trait Symbol: Send + Sync + 'static {
    async fn refresh(
      &self,
      request: tonic::Request<super::RefreshRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status>;
    async fn query(
      &self,
      request: tonic::Request<super::QueryRequest>,
    ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status>;
  }
  #[derive(Debug)]
  pub struct SymbolServer<T: Symbol> {
    inner: _Inner<T>,
  }
  struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
  impl<T: Symbol> SymbolServer<T> {
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
  impl<T, B> Service<http::Request<B>> for SymbolServer<T>
  where
    T: Symbol,
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
        "/symbol.Symbol/refresh" => {
          #[allow(non_camel_case_types)]
          struct refreshSvc<T: Symbol>(pub Arc<T>);
          impl<T: Symbol> tonic::server::UnaryService<super::RefreshRequest>
            for refreshSvc<T>
          {
            type Response = ();
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::RefreshRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).refresh(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = refreshSvc(inner);
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
        "/symbol.Symbol/query" => {
          #[allow(non_camel_case_types)]
          struct querySvc<T: Symbol>(pub Arc<T>);
          impl<T: Symbol> tonic::server::UnaryService<super::QueryRequest>
            for querySvc<T>
          {
            type Response = super::QueryResponse;
            type Future =
              BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
            fn call(
              &mut self,
              request: tonic::Request<super::QueryRequest>,
            ) -> Self::Future {
              let inner = self.0.clone();
              let fut = async move { (*inner).query(request).await };
              Box::pin(fut)
            }
          }
          let inner = self.inner.clone();
          let fut = async move {
            let interceptor = inner.1.clone();
            let inner = inner.0;
            let method = querySvc(inner);
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
  impl<T: Symbol> Clone for SymbolServer<T> {
    fn clone(&self) -> Self {
      let inner = self.inner.clone();
      Self { inner }
    }
  }
  impl<T: Symbol> Clone for _Inner<T> {
    fn clone(&self) -> Self {
      Self(self.0.clone(), self.1.clone())
    }
  }
  impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self.0)
    }
  }
  impl<T: Symbol> tonic::transport::NamedService for SymbolServer<T> {
    const NAME: &'static str = "symbol.Symbol";
  }
}
