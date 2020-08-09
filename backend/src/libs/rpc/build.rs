use ::std::error::Error;

use ::glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
  let mut protos = vec![];
  for proto in glob("../../../../proto/**/*.proto")? {
    let path = proto?;
    let path = String::from(path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", path);
    protos.push(path);
  }
  return match ::tonic_build::configure()
    .build_server(true)
    .build_client(false)
    .compile(&protos, &[String::from("../../../../proto")])
  {
    Err(e) => Err(Box::new(e)),
    Ok(ok) => Ok(ok),
  };
}
