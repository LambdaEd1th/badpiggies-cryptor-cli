use std::borrow::Cow;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./resources"]
pub struct Resource;

impl Resource {
    pub fn get_example() -> Option<Cow<'static, [u8]>> {
        match Self::get("Example.xml") {
            Some(file) => Some(file.data),
            None => None,
        }
    }
}
