use super::types::*;

pub fn snap() -> (TimeStamp, ImageBin) {
    (TimeStamp::new(0), ImageBin::new(vec![0, 0, 0, 0]))
}
