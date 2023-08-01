use crate::thread_manager::ThreadManager;

mod thread_manager;
mod gui;
mod configuration;
mod image_formatter;
mod screenshots;
mod draw_window;
mod image_combiner;
mod windows;

fn main() { ThreadManager::new().join(); }
