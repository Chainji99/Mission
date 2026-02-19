use anyhow::Result;
#[derive(Debug, Clone)]
pub struct Base64Image(String)

impl Base64Image {
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn new(data: String) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow::anyhow!(" data cannot be empty !!"));
        }
        let bytes: Vec<u8> = match general_purpose::STANDARD.decode(&data) {
            
        };
    }
}
