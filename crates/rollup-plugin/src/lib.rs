#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use std::path::Path;

use napi::bindgen_prelude::*;
use tauri_bindgen_core::{Files, WorldGenerator};
use wit_parser::World;

#[napi]
pub fn bindgen(path: String) -> Result<String> {
  let path = Path::new(&path);

  let world = World::parse_file(&path).map_err(|e| {
    napi::Error::from_reason(format!(
      "failed to parse wit file `{}` with error {}",
      path.display(),
      e
    ))
  })?;

  let mut files = Files::default();

  gen_world(
    tauri_bindgen_gen_guest_js::Opts::default().build(),
    world,
    &mut files,
  )?;

  if files.iter().count() != 1 {
    return Err(napi::Error::from_reason(format!(
      "Expected codegen to produce exactly one output file! got {}",
      files.iter().count()
    )));
  }

  let (_, str) = files.iter().next().unwrap();
  Ok(unsafe { std::str::from_utf8_unchecked(str).to_string() })
}

fn gen_world(
  mut generator: Box<dyn WorldGenerator>,
  world: World,
  files: &mut Files,
) -> Result<()> {
  let name = world.name.clone();
  generator.generate(&name, &world, files);
  Ok(())
}
