use std::path::Path;
use libsc::rm::rm;

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

#[cfg(test)]
mod tests {
    use std::path::Path;
    use libsc::rm::rm;

    #[test]
    fn delete() {
        std::fs::File::create("test.txt").unwrap();
        rm(vec![Path::new("test.txt")]);
    }

    #[test]
    fn wild_card() {
        std::fs::File::create("test.txt").unwrap();
        rm(vec![Path::new("*")]);
    }

    #[test]
    fn wild_card_stem() {
        std::fs::File::create("test.txt").unwrap();
        rm(vec![Path::new("*.txt")]);
    }

    #[test]
    fn wild_card_ext() {
        std::fs::File::create("test.txt").unwrap();
        rm(vec![Path::new("test.*")]);
    }
}
