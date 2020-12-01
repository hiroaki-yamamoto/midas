use ::std::error::Error;

use ::glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
  let mut protos = vec![];
  for proto in glob("../../../proto/**/*.proto")? {
    let path = proto?;
    let path = String::from(path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", path);
    protos.push(path);
  }
  return match ::tonic_build::configure()
    .out_dir("./src")
    .build_server(true)
    .build_client(false)
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
    .type_attribute(
      "entities.Exchanges",
      "#[derive(::num_derive::FromPrimitive, ::clap::Clap)]",
    )
    .type_attribute(
      "entities.BackTestPriceBase",
      "#[derive(::num_derive::FromPrimitive, ::clap::Clap)]",
    )
    .type_attribute("entities.Exchanges", "#[serde(tag = \"exchange\")]")
    .field_attribute(
      "historical.HistChartFetchReq.symbols",
      "#[serde(rename = \"symbolsList\")]",
    )
    .field_attribute(
      "historical.StopRequest.symbols",
      "#[serde(rename = \"symbolsList\")]",
    )
    .compile(&protos, &[String::from("../../../proto")])
  {
    Err(e) => Err(Box::new(e)),
    Ok(ok) => Ok(ok),
  };
}
