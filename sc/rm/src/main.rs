use libsc::{wild_card, WildCardType};
use std::path::Path;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        std::process::exit(0);
    }
    args.remove(0);
    let mut path = vec![];
    for i in &args {
        path.push(Path::new(i.as_str()));
    }

    rm(path);
}

pub fn rm(p: Vec<&Path>) {
    let mut success = 0;
    let mut failed = 0;
    for p in p {
        match wild_card(p) {
            WildCardType::AllFile => {
                let entries = std::fs::read_dir(".").unwrap();
                for entry in entries {
                    delete_file(&entry.as_ref().unwrap().path(), &mut success, &mut failed)
                }
            }

            WildCardType::SpecificExt => {
                // wcefew.*

                let entries = std::fs::read_dir(".").unwrap();
                let mut file = p.to_path_buf();
                for entry in entries {
                    let cp = match &mut entry.as_ref() {
                        Ok(entry) => entry.path(),
                        Err(_) => {
                            continue;
                        }
                    };

                    let ext = match cp.extension() {
                        None => {
                            continue;
                        }
                        Some(ext) => ext.to_str().unwrap(),
                    };

                    file.set_extension(ext);

                    match file.exists() {
                        true => match file.is_dir() {
                            true => {
                                continue;
                            }
                            false => {}
                        },
                        false => {
                            continue;
                        }
                    }

                    delete_file(&file, &mut success, &mut failed)
                }
            }

            WildCardType::SpecificFileName => {
                //*.py
                let entries = std::fs::read_dir(".").unwrap();
                let ext = p.extension().unwrap().to_str().unwrap();
                for entry in entries {
                    let p = &mut entry.as_ref().unwrap().path();
                    p.set_extension(ext);
                    match p.exists() {
                        true => match p.is_dir() {
                            true => {
                                continue;
                            }
                            false => {}
                        },
                        false => {
                            continue;
                        }
                    }
                    println!("{}", p.display());
                    delete_file(p, &mut success, &mut failed)
                }
            }

            WildCardType::Normal => delete_file(p, &mut success, &mut failed),
        }
    }

    println!(
        "Operation ended with {} completed {} failed",
        success, failed
    );
}

fn delete_file(p: &Path, success: &mut i32, failed: &mut i32) {
    match std::fs::remove_file(p) {
        Ok(_) => {
            *success += 1;
        }

        Err(err) => {
            eprintln!("{}", err);
            *failed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn delete() {
        std::fs::File::create("test.txt").unwrap();
        crate::rm(vec![Path::new("test.txt")]);
    }

    #[test]
    fn wild_card() {
        std::fs::File::create("test.txt").unwrap();
        crate::rm(vec![Path::new("*")]);
    }

    #[test]
    fn wild_card_stem() {
        std::fs::File::create("test.txt").unwrap();
        crate::rm(vec![Path::new("*.txt")]);
    }

    #[test]
    fn wild_card_ext() {
        std::fs::File::create("test.txt").unwrap();
        crate::rm(vec![Path::new("test.*")]);
    }
}
