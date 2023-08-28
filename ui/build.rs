use slint_build::{CompilerConfiguration, EmbedResourcesKind};

fn main() {
    let conf = CompilerConfiguration::new().embed_resources(EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("components/main.slint", conf).unwrap();
}
