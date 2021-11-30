use std::{env, error::Error, path::Path};

use spirv_builder::SpirvBuilder;

pub struct ShaderBuilder {
}

impl ShaderBuilder {
    pub fn new() -> ShaderBuilder {
        ShaderBuilder {  }
    }

    pub fn build_shader<'a>(
        &'a mut self,
        path_to_crate: &str,
    ) -> Result<&'a mut Self, Box<dyn Error>> {
        let builder_dir = &Path::new(env!("CARGO_MANIFEST_DIR"));
        let path_to_crate = builder_dir.join(path_to_crate);
        let builder = SpirvBuilder::new(path_to_crate, "spirv-unknown-vulkan1.2");

        let _result = builder
            .print_metadata(spirv_builder::MetadataPrintout::None)
            .spirv_metadata(spirv_builder::SpirvMetadata::Full)
            .build()?;

        Ok(self)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_OS");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
    // While OUT_DIR is set for both build.rs and compiling the crate, PROFILE is only set in
    // build.rs. So, export it to crate compilation as well.
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rustc-env=PROFILE={}", profile);

    let mut shader_builder = ShaderBuilder::new();

    shader_builder
        .build_shader(
            "../buggy-shader",
        )?;

    Ok(())
}
