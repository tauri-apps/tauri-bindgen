#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use std::path::Path;

use napi::bindgen_prelude::*;
use tauri_bindgen_core::{Files, WorldGenerator};
use wit_parser::World;

#[napi]
pub enum Kind {
  Javascript,
  Typescript,
}

#[napi]
pub fn process(kind: Kind, s: String) -> Result<String> {
  let path = Path::new(&s);

  if !path.is_file() {
    return Err(napi::Error::from_reason(format!(
      "wit file `{}` does not exist",
      path.display()
    )));
  }

  let world = World::parse_file(&path).map_err(|_| {
    napi::Error::from_reason(format!("failed to parse wit file `{}`", path.display()))
  })?;

  let mut files = Files::default();
  match kind {
    Kind::Javascript => {
      gen_world(
        tauri_bindgen_gen_guest_js::Opts::default().build(),
        world,
        &mut files,
      )?;
    }
    Kind::Typescript => {
      gen_world(
        tauri_bindgen_gen_guest_ts::Opts::default().build(),
        world,
        &mut files,
      )?;
    }
  }

  if files.iter().count() != 1 {
    return Err(napi::Error::from_reason(
      "Expected codegen to generate exactly one file",
    ));
  }

  let (_, file) = files.iter().next().unwrap();
  Ok(unsafe { std::str::from_utf8_unchecked(file).to_string() })
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
