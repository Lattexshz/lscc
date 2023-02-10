use std::path::Path;

pub enum WildCardType {
    AllFile,
    SpecificExt,
    SpecificFileName,
    Normal,
}

pub fn wild_card(base: &Path) -> WildCardType {
    let file_name = base.file_stem();
    let file_ext = base.extension();
    if file_name.unwrap().to_str().unwrap() == "*" && file_ext.is_none() {
        return WildCardType::AllFile;
    } else if file_name.unwrap().to_str().unwrap() != "*"
        && file_ext.unwrap().to_str().unwrap() == "*"
    {
        return WildCardType::SpecificExt;
    } else if file_name.unwrap().to_str().unwrap() == "*" && file_ext.is_some() {
        return WildCardType::SpecificFileName;
    }
    WildCardType::Normal
}
