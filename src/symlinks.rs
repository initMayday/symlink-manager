use std::{io, path::Path, sync::Arc};
use std::os::unix::fs as unix_fs;

use owo_colors::OwoColorize;
use tokio::{fs, process::Command, sync::Semaphore, task::JoinSet};

use crate::{
    Config,
    utils::{self, write_err, write_suc},
};

async fn remove_path(path: &Path, lock: Arc<Semaphore>) -> bool {
    let confirmation = utils::get_confirmation(
        format!(
            "The path {}, exists. Would you like to remove it?",
            path.display(),
        )
        .as_str(),
        lock.clone()
    ).await;

    if confirmation {
        let result = match fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == io::ErrorKind::IsADirectory => fs::remove_dir_all(path).await,
            Err(err) => Err(err),
        };

        match result {
            Ok(()) => {
                let _ = lock.acquire().await;
                write_suc(format!("Removed path: {}", path.display()).as_str());
                return true;
            }
            Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
                let settings = utils::settings();
                let permit = lock.acquire().await;
                let output = Command::new(&settings.superuser_command)
                    .arg("rm")
                    .arg("-rf")
                    .arg(path)
                    .output()
                    .await
                    .unwrap();
                drop(permit);

                if output.status.success() {
                    let _ = lock.acquire().await;
                    write_suc(format!("Removed path (superuser): {}", path.display()).as_str());
                    return true;
                } else {
                    let _ = lock.acquire().await;
                    write_err(
                        format!(
                            "Failed, could not remove path: {}, Err: {}",
                            path.display(),
                            String::from_utf8_lossy(&output.stderr)
                        )
                        .as_str(),
                    );
                    return false;
                }
            }
            Err(err) => {
                let _ = lock.acquire().await;
                write_err(
                    format!(
                        "Failed, could not remove path: {}, Err: {}",
                        path.display(),
                        err
                    )
                    .as_str(),
                );
                return false;
            }
        }
    } else {
        let _ = lock.acquire().await;
        write_err(format!("Aborting, could not remove path: {}", path.display()).as_str());
        return false;
    }
}

async fn create_symlink(base_path: &Path, symlink_path: &Path, lock: Arc<Semaphore>) -> bool {
    match unix_fs::symlink(base_path, symlink_path) {
        Ok(()) => {
            let _ = lock.acquire().await;
            write_suc(
                format!(
                    "Created symlink: {} -> {}",
                    symlink_path.display(),
                    base_path.display()
                )
                .as_str(),
            );
            return true;
        }
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            let settings = utils::settings();
            let permit = lock.acquire().await;
            let output = Command::new(&settings.superuser_command)
                .arg("ln")
                .arg("-s")
                .arg(base_path)
                .arg(symlink_path)
                .output()
                .await
                .unwrap();
            drop(permit);

            if output.status.success() {
                let _ = lock.acquire().await;
                write_suc(
                    format!(
                        "Created symlink (superuser): {} -> {}",
                        base_path.display(),
                        symlink_path.display()
                    )
                    .as_str(),
                );
                return true;
            } else {
                let _ = lock.acquire().await;
                write_err(
                    format!(
                        "Failed, could not create symlink (superuser): {} -> {}, Err: {}",
                        base_path.display(),
                        symlink_path.display(),
                        String::from_utf8_lossy(&output.stderr)
                    )
                    .as_str(),
                );
                return false;
            }
        }
        Err(err) => {
            let _ = lock.acquire().await;
            write_err(
                format!(
                    "Failed, could not create symlink: {} -> {}, Err: {}",
                    base_path.display(),
                    symlink_path.display(),
                    err
                )
                .as_str(),
            );
            return false;
        }
    }
}

pub async fn process(config: &Config) {
    let mut set = JoinSet::new();
    let lock = Arc::new(Semaphore::new(1));

    for (base_path, symlink_path) in config.symlinks.clone() {
        let lock = Arc::clone(&lock);
        set.spawn(async move {
            let file_path = Path::new(&base_path);
            let symlink_path = Path::new(&symlink_path);
            if fs::try_exists(symlink_path).await.unwrap() {
                // Check it points to the right place
                let metadata = fs::symlink_metadata(symlink_path).await.unwrap();
                if metadata.file_type().is_symlink() {
                    // Check it points to the right file, else, remove it
                    if symlink_path == fs::read_link(&symlink_path).await.unwrap() {
                        return
                    } else {
                        remove_path(&symlink_path, lock.clone()).await;
                    }
                } else {
                    remove_path(&symlink_path, lock.clone()).await;
                }
            }

            // Try and now create the symlink - it was invalid before, or didn't exist
            // Ensure the parent directories exist
            if let Some(parent) = file_path.parent() {
                if !fs::try_exists(parent).await.unwrap() {
                    if !utils::create_path(parent, lock.clone()).await {
                        return;
                    }
                }
            }

            println!("creating {}", file_path.display());
            create_symlink(file_path, symlink_path, lock).await;
            
        });
    }

    while let Some(_res) = set.join_next().await {} // We don't actually do anything as of now
}
