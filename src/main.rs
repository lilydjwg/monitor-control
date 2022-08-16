use std::fs::read_dir;

use clap::Parser;
use ddc::Ddc;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  output_name: String,
  feature_code: u8,
  feature_value: Option<u16>,
}

// /sys/class/drm/card*-{name}/i2c-*
fn get_i2c_dev(output: &str) -> String {
  let mut output_dir = None;
  for entry in read_dir("/sys/class/drm").unwrap() {
    let path = entry.unwrap().path();
    let name = path.file_name().unwrap().to_str().unwrap();
    if name.starts_with("card") && name.ends_with(output) {
      let before_name = name.len() - output.len() - 1;
      if &name[before_name..before_name+1] == "-" {
        output_dir = Some(path);
        break;
      }
    }
  };
  let mut dev = None;
  for entry in read_dir(output_dir.unwrap()).unwrap() {
    let file_name = entry.unwrap().file_name();
    let name = file_name.to_str().unwrap();
    if name.starts_with("i2c-") {
      dev = Some(name.to_owned());
    }
  }

  dev.unwrap()
}

fn main() {
  let cli = Cli::parse();
  let i2c_name = get_i2c_dev(&cli.output_name);
  let dev = format!("/dev/{}", i2c_name);
  let mut ddc = ddc_i2c::from_i2c_device(&dev).unwrap();
  if let Some(v) = cli.feature_value {
    ddc.set_vcp_feature(cli.feature_code, v).unwrap();
    println!("{}", v);
  } else {
    let value = ddc.get_vcp_feature(cli.feature_code).unwrap();
    println!("{} {}", value.value(), value.maximum());
  }
}
