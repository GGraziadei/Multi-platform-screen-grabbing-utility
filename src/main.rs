use crate::thread_manager::ThreadManager;

mod thread_manager;
mod gui;
mod configuration;
mod image_formatter;
mod screenshots;
mod main_window;
mod image_combiner;

fn main() { ThreadManager::new().join(); }
