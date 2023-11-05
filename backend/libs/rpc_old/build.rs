use ::glob::glob;

fn main() {
  let mut protos = vec![];
  for proto in glob("../../../proto/**/*.proto").unwrap() {
    let path = proto.unwrap();
    let path = String::from(path.to_str().unwrap());
    println!("cargo:rerun-if-changed={}", path);
    protos.push(path);
  }
  return ::prost_build::Config::new()
    .out_dir("./src")
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
    .type_attribute("entities.Exchanges", "#[derive(::clap::Parser)]")
    .type_attribute("entities.Exchanges", "#[serde(tag = \"exchange\")]")
    .type_attribute("entities.BackTestPriceBase", "#[derive(::clap::Parser)]")
    .type_attribute("historical.HistChartProg", "#[derive(Eq)]")
    .field_attribute(
      "historical.HistChartFetchReq.symbols",
      "#[serde(rename = \"symbolsList\")]",
    )
    .field_attribute(
      "historical.StopRequest.symbols",
      "#[serde(rename = \"symbolsList\")]",
    )
    .field_attribute(
      "keychain.APIKeyList.keys",
      "#[serde(rename = \"keysList\")]",
    )
    .field_attribute(
      "entities.InsertOneResult.id",
      "#[serde(skip_serializing_if = \"String::is_empty\")]",
    )
    .compile_well_known_types()
    .compile_protos(&protos, &[String::from("../../../proto")])
    .unwrap();
}
