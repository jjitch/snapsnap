#[derive(Debug, Clone, PartialEq)]
pub struct ImageBin(std::vec::Vec<u8>);

impl ImageBin {
    pub fn new(v: std::vec::Vec<u8>) -> Self {
        ImageBin(v)
    }
}

impl Into<tauri::ipc::InvokeResponseBody> for ImageBin {
    fn into(self) -> tauri::ipc::InvokeResponseBody {
        tauri::ipc::InvokeResponseBody::Raw(self.0)
    }
}
