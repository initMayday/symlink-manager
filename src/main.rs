mod cache;
mod files;
mod symlinks;
mod utils;

use mlua::{Lua, LuaOptions, LuaSerdeExt, StdLib};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::sync::Semaphore;

use clap::Parser;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Config {
    files: HashMap<String, String>,
    symlinks: HashMap<String, String>,
    settings: Settings,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Settings {
    add_path_confirmation: bool,
    remove_path_confirmation: bool,
    cache_path: String,
    superuser_command: String,
}

#[derive(Debug, Parser)]
struct Args {
    lua_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let lua_source = fs::read_to_string(&args.lua_file)?;
    // Debug is an unsafe lib so funny stuff can occur
    let lua =
        unsafe { Lua::unsafe_new_with(StdLib::ALL_SAFE | StdLib::DEBUG, LuaOptions::default()) };

    let chunk_name = format!("@{}", args.lua_file.display());
    let chunk = lua.load(&lua_source).set_name(&chunk_name);
    let value = chunk.eval::<mlua::Value>()?;
    let config: Config = lua.from_value(value)?;
    utils::init_settings(config.settings.clone());

    let lock = Arc::new(Semaphore::new(1));

    let ((), ()) = tokio::join!(
        files::process(&config, lock.clone()),
        symlinks::process(&config, lock.clone()),
    );

    Ok(())
}
