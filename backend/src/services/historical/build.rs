use ::std::error::Error;

fn main() -> Result<(), impl Error> {
  return ::tonic_build::configure()
    .build_server(true)
    .build_client(false)
    .compile(
      &["../../../../proto/historical.proto"],
      &["../../../../proto"],
    );
}
