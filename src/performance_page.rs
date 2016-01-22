extern crate gtk;

use gtk::traits::*;
use gtk::widgets::*;
use graphic::{Graphic, BasicGraphic};

pub struct PerformancePage {
    main_box: gtk::Box,
    performance_switcher: gtk::ListBox,
    performance_stack: gtk::Stack
}

impl PerformancePage {
    pub fn new() -> PerformancePage {
        let glade_src = include_str!("../data/performance_page.ui");
        let builder = Builder::new_from_string(glade_src).unwrap();

        unsafe {
            let mut instance = PerformancePage {
                main_box: builder.get_object("performance_page").unwrap(),
                performance_switcher: builder.get_object("performance_switcher").unwrap(),
                performance_stack: builder.get_object("performance_stack").unwrap(),

            };

            instance.create_memory_page();
            instance.performance_switcher.connect_row_activated(|switcher, row| {
                println!("Activated!");
            });

            return instance;
        }

    }

    fn create_memory_page (&mut self) {
        let switcher_row = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
        let switcher_graphic = BasicGraphic::new();
        let label = gtk::Label::new("Memory").unwrap();

        switcher_row.add(switcher_graphic.get_widget());
        switcher_row.add(&label);
        switcher_row.set_size_request(-1, 200);

        self.performance_switcher.add(&switcher_row);

        let mut graphic = Graphic::new();
        graphic.set_title("Memory");
        self.performance_stack.add(graphic.get_widget());
    }

    pub fn get_widget (&self) -> &gtk::Box {
       return  &self.main_box;
    }
}

