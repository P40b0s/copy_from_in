use crate::DirectoriesSpy;





#[cfg(test)]
mod tests
{
    use crate::{DirectoriesSpy, AppState, app::STATE};

    #[test]
    fn test_directory_spy()
    {
        AppState::initialize();
        let settings = &STATE.get().unwrap().lock().unwrap().settings;

        let t = settings.tasks.iter().nth(0).unwrap();

        DirectoriesSpy::check_for_new_packets(t, |found|
        {
            
        });
    }
}