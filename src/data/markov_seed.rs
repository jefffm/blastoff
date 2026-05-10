use std::borrow::Cow;

use assets_manager::{BoxedError, FileAsset};

pub struct MarkovSeed(String);
impl MarkovSeed {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for MarkovSeed {
    fn from(txt: String) -> Self {
        MarkovSeed(txt)
    }
}

impl FileAsset for MarkovSeed {
    const EXTENSION: &'static str = "txt";

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, BoxedError> {
        Ok(String::from_utf8(bytes.into_owned())?.into())
    }
}
