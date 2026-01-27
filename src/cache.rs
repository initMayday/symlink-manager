use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use std::{
    env, fs,
    path::PathBuf,
    process::exit, sync::Arc,
};

use crate::utils::{get_confirmation, write_err};

#[derive(Default, Serialize, Deserialize)]
pub struct Cache {
    files: Vec<String>,
    symlinks: Vec<String>,
}

async fn read_cache() -> Cache {
    let lock = Arc::new(Semaphore::new(1));

    let cache_dir = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .ok_or_else(|| {
            write_err("XDG_CACHE_HOME wasn't set, nor was HOME!");
            exit(1);
        })
        .unwrap();

    let cache_path = cache_dir.join("desym.cache");

    if !cache_path.exists() {
        return Cache::default();
    }

    let text = fs::read_to_string(&cache_path).expect("Unable to read cache file");
    match serde_json::from_str(&text) {
        Ok(cache) => cache,
        Err(_) => {
            write_err("Unable to read cache file! This may be due to a format update, or incorrect modification of the cache file");
            let confirmation = get_confirmation(
                "Would you like to continue? The cache file will be overwritten, and updated as needed",
                lock,
            )
            .await;
            if confirmation {
                Cache::default()
            } else {
                write_err("Unable to continue! Please rectify the cache file issue");
                exit(1);
            }
        }
    }
}
