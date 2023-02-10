use std::path::Path;

pub enum WildCardType {
    AllFile,
    SpecificExt,
    Normal
}

pub fn wild_card(base: &Path) -> WildCardType {
    let file_name = base.file_name().unwrap().to_str().unwrap();
    let file_ext = base.extension();
    if file_name == "*" && file_ext.is_none() {
        return WildCardType::AllFile;
    }
    WildCardType::Normal
}