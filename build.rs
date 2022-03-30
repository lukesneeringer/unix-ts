use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CRATE_NAME: &str = "plutus";

fn main() -> Result<(), Box<dyn Error>> {
  println!("cargo:rerun-if-changed=README.md");
  fs::write(
    PathBuf::from(env::var("OUT_DIR")?).join("README-rustdocified.md"),
    readme_rustdocifier::rustdocify(
      &fs::read_to_string("README.md")?,
      &env::var("CARGO_PKG_NAME")?,
      Some(&env::var("CARGO_PKG_VERSION")?),
      Some(CRATE_NAME),
    )?,
  )?;
  Ok(())
}
