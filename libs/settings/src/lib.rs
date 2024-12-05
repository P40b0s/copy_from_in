//mod io;
mod task;
mod validation_error;
mod file_methods;
mod settings;
pub use file_methods::FileMethods;
pub use utilites::{Serializer, serialize_to_file};
pub use settings::Settings;
pub use task::{Task, Filter, CopyModifier};
pub use validation_error::ValidationError;

#[cfg(test)]
mod test
{
    use std::path::{Path, PathBuf};

    use serde::Deserialize;
    use crate::{file_methods::FileMethods, CopyModifier, Settings, Task};
    use utilites::{Serializer};
    #[test]
    fn test_serialize_medo()
    {
        let mut medo: Settings = Settings::default();
        let mut task = Task::default();
        task.target_dirs.push(PathBuf::new().join("d1"));
        task.target_dirs.push(PathBuf::new().join("d2"));
        task.target_dirs.push(PathBuf::new().join("d3"));
        medo.tasks.push(task);
        let r = medo.save(Serializer::Toml);
        assert!(r.is_ok())
    }
    #[test]
    fn test_from_toml_to_json()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(Serializer::Toml).unwrap();
        let r = settings.save(Serializer::Json);
        assert!(r.is_ok())
    }
    #[test]
    fn test_from_json_to_toml()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(Serializer::Json).unwrap();
        let r = settings.save(Serializer::Toml);
        assert!(r.is_ok())
    }
    #[test]
    fn test_serialize_settings_json()
    {
        let settings: Settings = Settings::default();
        let r = settings.save(Serializer::Json);
        assert!(r.is_ok())
    }

    //TODO сделать заполненый таск! сейчас тесты не проходят
    #[test]
    fn test_deserialize_settings_json()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(Serializer::Json).unwrap();
        assert_eq!(settings.tasks[0].copy_modifier, CopyModifier::CopyAll);
        assert_eq!(settings.tasks[1].copy_modifier, CopyModifier::CopyOnly);
    }
    #[test]
    fn test_deserialize_settings_toml()
    {
        logger::StructLogger::new_default();
        let settings = Settings::load(Serializer::Toml).unwrap();
        assert_eq!(settings.tasks[0].copy_modifier, CopyModifier::CopyAll);
        assert_eq!(settings.tasks[1].copy_modifier, CopyModifier::CopyOnly);
    }
}