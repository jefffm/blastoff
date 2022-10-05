use assets_manager::{
    loader::{LoadFrom, StringLoader},
    Asset,
};

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

impl Asset for MarkovSeed {
    const EXTENSION: &'static str = "txt";

    const EXTENSIONS: &'static [&'static str] = &[Self::EXTENSION];

    type Loader = LoadFrom<String, StringLoader>;

    const HOT_RELOADED: bool = true;
}
