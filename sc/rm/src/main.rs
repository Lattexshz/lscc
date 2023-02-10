use std::path::Path;
use libsc::{wild_card, WildCardType};

fn main() {
    let mut args:Vec<String> = std::env::args().collect();
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
                    delete_file(&entry.as_ref().unwrap().path(),&mut success,&mut failed)
                }
            }
            WildCardType::SpecificExt => {}
            WildCardType::Normal => {
                delete_file(p,&mut success,&mut failed)
            }
        }
    }

    println!("Operation ended with {} completed {} failed",success,failed);
}

fn delete_file(p:&Path, success: &mut i32, failed: &mut i32) {
    match std::fs::remove_file(p) {
        Ok(_) => {
            *success += 1;
        }

        Err(err) => {
            eprintln!("{}",err);
            *failed += 1;
        }
    }
}