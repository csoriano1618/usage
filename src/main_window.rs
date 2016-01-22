extern crate gtk;

use std;
use performance_page::PerformancePage;

pub struct MainWindow {
    window: gtk::Window,
    pages_stack: gtk::Stack,
    stack_switcher: gtk::Stack
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let glade_src = include_str!("../data/main_window.ui");
        let builder = gtk::widgets::Builder::new_from_string(glade_src).unwrap();

        unsafe {
            let instance = MainWindow {
                window: builder.get_object("window").unwrap(),
                pages_stack: builder.get_object("pages_stack").unwrap(),
                stack_switcher: builder.get_object("stack_switcher").unwrap(),
            };

            let performance_page = PerformancePage::new();
            let performance_page_widget = performance_page.get_widget();
            instance.pages_stack.add_titled(performance_page_widget, "performance", "Performance");

            return instance;
        }
    }

    pub fn get_widget (&self) -> &gtk::Window {
       return  &self.window;
    }
}
