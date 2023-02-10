use std::path::Path;
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
    let mut count = 1;
    let len = p.len();

    let mut success = 0;
    let mut failed = 0;
    for p in p {
        println!("Removing files... {} of {} ",count,len);
        match std::fs::remove_file(p) {
            Ok(_) => {
                success += 1;
            }

            Err(err) => {
                eprintln!("{}",err);
                failed += 1;
            }
        }
        count += 1;
    }

    println!("Operation ended with {} completed {} failed",success,failed);
}