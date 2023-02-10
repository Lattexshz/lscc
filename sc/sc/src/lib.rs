use std::path::Path;

pub fn wild_card_file_ext(base: &Path) -> Vec<String> {
    let mut available_ext:Vec<&str> = vec![];
    let entries = std::fs::read_dir(".").unwrap();

    for entry in entries {
        println!("{:?}", entry.unwrap().file_name());
    }
    // for i in std::fs::read_dir("./").unwrap() {
    //
    //     let path = Path::new(i.unwrap().file_name().to_str().unwrap());
    //     match path.exists() {
    //         true => {}
    //         false => {
    //             continue;
    //         }
    //     }
    //     available_ext.push(path.to_str().unwrap());
    // }
    let mut files:Vec<String> = vec![];
    // let file_name = base.file_name().unwrap().to_str().unwrap();
    // for i in &available_ext {
    //     println!("{}.{}",file_name,i);
    //     files.push(format!("{}.{}",file_name,i))
    // }
    files
}