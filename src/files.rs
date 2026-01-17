use std::{path::Path, sync::Arc};

use tokio::{fs, sync::Semaphore, task::JoinSet};

use crate::{
    Config,
    utils::{write_err, write_suc},
};

async fn write_to_file(path: &Path, content: String, lock: Arc<Semaphore>) {
    match fs::write(path, content).await {
        Ok(_) => {
            let _ = lock.acquire().await;
            write_suc(format!("Wrote to, Path: {}", path.display()).as_str());
        }
        Err(err) => {
            let _ = lock.acquire().await;
            write_err(
                format!(
                    "Failed to write to file, Path: {}, Error: {}",
                    path.display(),
                    err
                )
                .as_str(),
            );
        }
    }
}

async fn process(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut set = JoinSet::new();
    let lock = Arc::new(Semaphore::new(1));

    for (path, new_content) in config.files.clone() {
        let lock = Arc::clone(&lock);
        set.spawn(async move {
            let file_path = Path::new(&path);
            if file_path.exists() {
                if file_path.is_dir() {
                    let _ = lock.acquire();
                    write_err(
                        format!(
                            "Failed to write to, Path: {}, as this is a directory!",
                            file_path.display()
                        )
                        .as_str(),
                    );
                }

                let old_content = fs::read_to_string(file_path).await.unwrap();
                if new_content != old_content {
                    // Update content
                    write_to_file(&file_path, new_content, lock).await;
                }
            } else {
                // Check if the path exists - create it if it doesn't
                // Create the file
                // Retry either of these as sudo if they fail due to perms
            }
        });
    }

    while let Some(res) = set.join_next().await {
        res?;
    }

    Ok(())
}
