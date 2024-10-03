use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./resources"]
pub struct Resource;

impl Resource {}
