use std::error::Error;

use glsl_to_spirv::ShaderType;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=shaders");

    std::fs::create_dir_all("assets/shaders").unwrap();

    for entry in std::fs::read_dir("assets/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            let shader_type = in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(ShaderType::Vertex),
                    "frag" => Some(ShaderType::Fragment),
                    _ => None,
                }
            });

            if let Some(t) = shader_type {
                use std::io::Read;

                let source = std::fs::read_to_string(&in_path)?;
                let mut compiled_file = glsl_to_spirv::compile(&source, t)?;

                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!("assets/shaders/{}.spv", in_path.file_name().unwrap().to_string_lossy());

                std::fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }

    Ok(())
}