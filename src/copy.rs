use crate::DirectoriesSpy;





#[cfg(test)]
mod tests
{
    use logger::debug;

    use crate::{DirectoriesSpy, AppState, app::STATE};

    #[test]
    fn test_directory_spy()
    {
        AppState::initialize();
        DirectoriesSpy::check_for_new_packets(|thread, found|
        {
            debug!("Сообщение от потока: {} был найден пакет {}", thread.thread_name, found);
        });
       
        logger::info!("Потоки выполняются параллельно, пока основной едет дальше");
        let delay = std::time::Duration::from_secs(3);

        loop
        {
            println!("sleeping for 3  sec ");
            std::thread::sleep(delay);
        }
    }
}