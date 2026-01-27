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
                write_suc(format!("Removed path: {}", path.display()).as_str());
                return true;
            }
            Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
                let settings = utils::settings();
                let _permit = lock.acquire().await;
                let output = Command::new(&settings.superuser_command)
                    .arg("rm")
                    .arg("-rf")
                    .arg(path)
                    .output()
                    .await
                    .unwrap();
                drop(_permit);

                if output.status.success() {
                    write_suc(format!("Removed path (superuser): {}", path.display()).as_str());
                    return true;
                } else {
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
        write_err(format!("Aborting, could not remove path: {}", path.display()).as_str());
        return false;
    }
}

async fn create_symlink(base_path: &Path, symlink_path: &Path, lock: Arc<Semaphore>) -> bool {
    match unix_fs::symlink(base_path, symlink_path) {
        Ok(()) => {
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
            let _permit = lock.acquire().await;
            let output = Command::new(&settings.superuser_command)
                .arg("ln")
                .arg("-s")
                .arg(base_path)
                .arg(symlink_path)
                .output()
                .await
                .unwrap();
            drop(_permit);

            if output.status.success() {
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

pub async fn process(config: &Config, lock: Arc<Semaphore>) {
    let mut set = JoinSet::new();

    for (symlink_path, base_path) in config.symlinks.clone() {
        let lock = Arc::clone(&lock);
        set.spawn(async move {
            let base_path = Path::new(&base_path);
            let symlink_path = Path::new(&symlink_path);
            match fs::symlink_metadata(symlink_path).await {
                Ok(metadata) => {
                    if metadata.file_type().is_symlink() {
                        let link_target = match fs::read_link(symlink_path).await {
                            Ok(target) => target,
                            Err(err) => {
                                write_err(
                                    format!(
                                        "Failed, could not read symlink target: {}, Err: {}",
                                        symlink_path.display(),
                                        err
                                    )
                                    .as_str(),
                                );
                                return;
                            }
                        };

                        let resolved_target = if link_target.is_absolute() {
                            link_target
                        } else {
                            symlink_path
                                .parent()
                                .unwrap_or(Path::new(""))
                                .join(&link_target)
                        };

                        if resolved_target == base_path {
                            return;
                        }

                        remove_path(symlink_path, lock.clone()).await;
                    } else {
                        remove_path(symlink_path, lock.clone()).await;
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::NotFound => {}
                Err(err) => {
                    write_err(
                        format!(
                            "Failed, could not stat path: {}, Err: {}",
                            symlink_path.display(),
                            err
                        )
                        .as_str(),
                    );
                    return;
                }
            }

            // Try and now create the symlink - it was invalid before, or didn't exist
            // Ensure the parent directories exist
            if let Some(parent) = symlink_path.parent() {
                if !fs::try_exists(parent).await.unwrap() {
                    if !utils::create_path(parent, lock.clone()).await {
                        return;
                    }
                }
            }

            create_symlink(base_path, symlink_path, lock).await;
            
        });
    }

    while let Some(_res) = set.join_next().await {} // We don't actually do anything as of now
}
