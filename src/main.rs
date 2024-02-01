mod copy;
mod settings;
mod app;
pub use app::AppState;
pub use io::DirectoriesSpy;
mod io;
extern crate chrono;


const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";
// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
fn main()
{
    AppState::initialize();
    DirectoriesSpy::process_tasks();
}