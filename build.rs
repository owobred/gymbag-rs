use std::{env, io::Result, path::PathBuf};

fn main() -> Result<()> {
    //     let mut prost_config = prost_build::Config::new();

    //     prost_config
    //         .type_attribute(".gymbag.proto", "#[derive(serde::Serialize)]")
    //         .type_attribute(".gymbag.proto", "#[serde(rename_all = \"snake_case\")]");

    //     prost_reflect_build::Builder::new()
    //         .descriptor_pool("crate::DescriptorPool")
    //         .compile_protos_with_config(prost_config, &["src/Recording/Frame.proto"], &["src/"])

    // prost_build::compile_protos(&["src/Recording/Frame.proto"], &["src/"])?;

    prost_build::Config::new()
        .type_attribute(".gymbag.proto", "#[derive(serde::Serialize)]")
        .type_attribute(".gymbag.proto", "#[serde(rename_all = \"snake_case\")]")
        .file_descriptor_set_path(
            &PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin"),
        )
        .compile_protos(&["src/Recording/Frame.proto"], &["src/"])

    // let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");
    // prost_build::Config::new()
    //     .file_descriptor_set_path(&descriptor_path)
    //     .compile_protos(&["src/Recording/Frame.proto"], &["src/"])?;
    //
    // let descriptor_set = std::fs::read(descriptor_path)?;
    // pbjson_build::Builder::new()
    //     .register_descriptors(&descriptor_set)?
    //     .build(&[".gymbag.proto"])?;
    // Ok(())
}
