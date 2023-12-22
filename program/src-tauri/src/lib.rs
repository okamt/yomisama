pub mod commands;
pub mod config;
pub mod state;
pub mod tray;
pub mod windows;

#[cfg(test)]
mod tests {
    use super::*;
    use typescript_type_def::{write_definition_file, DefinitionFileOptions};

    type TSTypes = (windows::Payload, config::Settings);

    // This would be better left in `build.rs`, but the crate types are not available before building.
    // Not ideal but it works for now.
    #[test]
    fn generate_typescript_bindings() {
        std::fs::write("../src/bindings.ts", {
            let mut buf = Vec::new();
            write_definition_file::<_, TSTypes>(
                &mut buf,
                DefinitionFileOptions {
                    root_namespace: None,
                    ..Default::default()
                },
            )
            .unwrap();
            String::from_utf8(buf).unwrap()
        })
        .unwrap();
    }
}
