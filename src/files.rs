use std::{io, path::Path, sync::Arc};

use tokio::{fs, process::Command, sync::Semaphore, task::JoinSet};

use crate::{
    Config,
    utils::{get_confirmation, write_err, write_suc},
};

async fn write_to_file(path: &Path, content: String, lock: Arc<Semaphore>) {
    let settings = crate::utils::settings();
    match fs::write(path, content.as_bytes()).await {
        Ok(_) => {
            let _ = lock.acquire().await;
            write_suc(format!("Wrote to, Path: {}", path.display()).as_str());
        }
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            let output = Command::new(&settings.superuser_command)
                .arg("bash")
                .arg("-c")
                .arg("printf '%s' \"$1\" > \"$2\"")
                .arg("--")
                .arg(&content)
                .arg(path)
                .output()
                .await;

            match output {
                Ok(output) if output.status.success() => {
                    let _ = lock.acquire().await;
                    write_suc(
                        format!("Wrote to (superuser), Path: {}", path.display()).as_str(),
                    );
                }
                Ok(output) => {
                    let _ = lock.acquire().await;
                    write_err(
                        format!(
                            "Failed to write to file (superuser), Path: {}, Exit: {}, Stderr: {}",
                            path.display(),
                            output.status,
                            String::from_utf8_lossy(&output.stderr),
                        )
                        .as_str(),
                    );
                }
                Err(err) => {
                    let _ = lock.acquire().await;
                    write_err(
                        format!(
                            "Failed to write to file (superuser), Path: {}, Error: {}",
                            path.display(),
                            err
                        )
                        .as_str(),
                    );
                }
            }
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

pub async fn process(config: &Config) {
    let mut set = JoinSet::new();
    let lock = Arc::new(Semaphore::new(1));

    for (path, new_content) in config.files.clone() {
        let lock = Arc::clone(&lock);
        set.spawn(async move {
            let file_path = Path::new(&path);
            if fs::try_exists(file_path).await.unwrap() {
                if file_path.is_dir() {
                    let _ = lock.acquire().await;
                    write_err(
                        format!(
                            "Failed to write to, Path: {}, as this is a directory!",
                            file_path.display()
                        )
                        .as_str(),
                    );
                    return
                }

                let old_content = fs::read_to_string(file_path).await.unwrap();
                if new_content != old_content {
                    // Update content
                    write_to_file(&file_path, new_content, lock).await;
                }
            } else {
                
                // Ensure the parent directories exist
                if let Some(parent) = file_path.parent() {
                    if !fs::try_exists(parent).await.unwrap() {
                        let confirmation = get_confirmation(
                            format!(
                                "The path {}, does not exist. It must exist in order to create {}. Would you like to do this?",
                                parent.display(),
                                file_path.display(),
                            ).as_str()
                        );

                        let mut created_path = confirmation;

                        if confirmation {
                            if let Err(err) = fs::create_dir_all(parent).await {
                                write_err(format!("Failed to create path: {}, error: {}", parent.display(), err).as_str());
                                created_path = false;
                            }
                        }

                        if created_path {
                            write_suc(format!("Created path: {}", parent.display()).as_str());
                        } else {
                            write_err(format!("Aborting, could not create path: {}", parent.display()).as_str());
                            return
                        }
                    }
                }

                // Create the file, and write to it
                write_to_file(file_path, new_content, lock).await;


            }
        });
    }

    while let Some(_res) = set.join_next().await {
    }
}
