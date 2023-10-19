pub mod cache;
pub mod data;
mod monitor;


pub fn init() {
    cache::init();
    monitor::init();
}
