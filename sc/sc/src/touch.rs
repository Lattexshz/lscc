use std::path::Path;
use std::time::SystemTime;

pub fn touch(p:Vec<&Path>) {
    for path in p {
        match path.exists() {
            true => {
                match std::fs::File::open(path) {
                    Ok(f) => {
                        let time = SystemTime::now();
                        match f.set_modified(time) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("{}",e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}",e);
                    }
                }
            }
            false => {
                match std::fs::File::create(path) {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("{}",err);
                    }
                }
            }
        }
    }
}