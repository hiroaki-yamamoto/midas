use ::std::error::Error;

use ::glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
  let mut protos = vec![];
  for proto in glob("../proto/**/*.proto")? {
    let path = proto?;
    let path = String::from(path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", path);
    protos.push(path);
  }
  return match ::tonic_build::configure()
    .out_dir("./src/libs/rpc/src")
    .build_server(true)
    .build_client(false)
    .type_attribute(
      "historical.HistChartProg",
      "#[derive(::serde::Serialize, ::serde::Deserialize)]",
    )
    .compile(&protos, &[String::from("../proto")])
  {
    Err(e) => Err(Box::new(e)),
    Ok(ok) => Ok(ok),
  };
}
