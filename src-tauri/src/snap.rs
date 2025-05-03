use super::types::*;

pub fn snap() -> ImageBin {
    let img_loadeded = std::fs::read("icons/128x128.png").unwrap_or_default();
    ImageBin::new(img_loadeded)
}
