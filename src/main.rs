extern crate gtk;

mod main_window;
mod performance_page;
mod graphic;

use gtk::traits::*;
use gtk::signal::Inhibit;
use gtk::widgets::*;
use main_window::MainWindow;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let main_window: MainWindow = MainWindow::new();
    let window = main_window.get_widget();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
