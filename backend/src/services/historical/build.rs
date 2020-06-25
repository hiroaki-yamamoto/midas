fn main() {
  ::prost_build::compile_protos(
    &["../../../../proto/historical.proto"],
    &["../../../../proto"]
  ).unwrap();
}
