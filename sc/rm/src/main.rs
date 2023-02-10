use std::path::Path;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        std::process::exit(0);
    }
    let path = Path::new(args[1].as_str());
    rm(path);
}

pub fn rm(p: &Path) {
    match std::fs::remove_file(p) {
        Ok(_) => {

        }

        Err(err) => {
            eprintln!("{}",err);
        }
    }
}