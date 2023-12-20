use thiserror::Error;

#[tauri::command]
pub fn set_config_dir(path: String) -> Result<(), Error> {
    // TODO do stuff

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
