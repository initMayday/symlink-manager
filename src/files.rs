use std::{io, path::Path, sync::Arc};

use tokio::{fs, process::Command, sync::Semaphore, task::JoinSet};

use crate::{
    Config,
    utils::{self, write_err, write_suc},
};

async fn write_to_file(path: &Path, content: String, lock: Arc<Semaphore>) {
    let settings = crate::utils::settings();
    match fs::write(path, content.as_bytes()).await {
        Ok(_) => {
            write_suc(format!("Wrote to, Path: {}", path.display()).as_str());
        }
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            let _permit = lock.acquire().await;
            let output = Command::new(&settings.superuser_command)
                .arg("bash")
                .arg("-c")
                .arg("printf '%s' \"$1\" > \"$2\"")
                .arg("--")
                .arg(&content)
                .arg(path)
                .output()
                .await;
            drop(_permit);

            match output {
                Ok(output) if output.status.success() => {
                    write_suc(
                        format!("Wrote to (superuser), Path: {}", path.display()).as_str(),
                    );
                }
                Ok(output) => {
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
                        if !utils::create_path(parent, lock.clone()).await {
                            return
            }
                    }
                }

                // Create the file, and write to it
                write_to_file(file_path, new_content, lock).await;
            }
        });
    }

    while let Some(_res) = set.join_next().await {} // We don't actually do anything as of now
}
