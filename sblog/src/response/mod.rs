mod monitor;

pub mod cache;
pub mod data;
pub mod template;

pub fn init() {
    cache::init();
    monitor::init();
}
