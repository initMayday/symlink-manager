
use mlua::{Lua, LuaSerdeExt};
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Config {
    files: HashMap<String, String>,
    symlinks: HashMap<String, String>,
    settings: Settings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Settings {
    add_symlink_confirmation: bool,
    add_path_confirmation: bool,
    remove_path_confirmation: bool,
    cache_path: String,
    superuser_command: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lua_source = fs::read_to_string("example.lua")?;
    let lua = Lua::new();

    let chunk = lua.load(&lua_source).set_name("example.lua");
    let value = chunk.eval::<mlua::Value>()?;
    let config: Config = lua.from_value(value)?;

    println!("{:#?}", config);

    Ok(())
}
