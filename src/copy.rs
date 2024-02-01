#[cfg(test)]
mod tests
{
    use crate::{DirectoriesSpy, AppState};
    #[test]
    fn test_directory_spy()
    {
        AppState::initialize();
        DirectoriesSpy::process_tasks();
    }
}