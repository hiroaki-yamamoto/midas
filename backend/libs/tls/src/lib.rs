use ::std::convert::TryInto;
use ::std::fs::read_to_string;
use ::std::path::Path;

use ::hyper::{
  client::connect::HttpConnector, Body, Client, Error, Request, Response, Uri,
};
use ::hyper_openssl::HttpsConnector;
use ::openssl::ssl::{SslConnector, SslMethod};
use ::openssl::x509::X509;
use ::tonic::body::BoxBody;
use ::tonic_openssl::ALPN_H2_WIRE;
use ::tower_util::BoxService;
use ::types::GenericResult;

pub fn init_tls_connection<A, B, C>(
  ca_path: A,
  dest_url: B,
) -> GenericResult<BoxService<Request<BoxBody>, Response<Body>, Error>>
where
  A: AsRef<Path>,
  B: TryInto<Uri>,
  <B as std::convert::TryInto<hyper::Uri>>::Error: std::error::Error + 'static,
{
  let uri: Uri = dest_url.try_into()?;
  let ca = read_to_string(ca_path)?;
  let ca = X509::from_pem(&ca.into_bytes()[..])?;
  let mut con = SslConnector::builder(SslMethod::tls())?;
  con.cert_store_mut().add_cert(ca)?;
  con.set_alpn_protos(ALPN_H2_WIRE)?;

  let mut http = HttpConnector::new();
  http.enforce_http(false);
  let https = HttpsConnector::with_connector(http, con)?;
  let hyper = Client::builder().http2_only(true).build(https);
  let ret = tower::service_fn(move |mut req: Request<BoxBody>| {
    let uri = Uri::builder()
      .scheme(uri.scheme().unwrap().clone())
      .authority(uri.authority().unwrap().clone())
      .path_and_query(req.uri().path_and_query().unwrap().clone())
      .build()
      .unwrap();

    *req.uri_mut() = uri;

    hyper.request(req)
  });
  let ret = BoxService::new(ret);
  return Ok(ret);
}
