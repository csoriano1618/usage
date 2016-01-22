extern crate cairo;
extern crate gtk;

use gtk::traits::*;
use gtk::widgets::*;

pub struct BasicGraphic {
    historic_area: gtk::DrawingArea,
    current_value: f32,
}

pub struct Graphic {
    graphic_grid: gtk::Grid,
    title_label: gtk::Label,
    current_value_label: gtk::Label,
    divisions_values_box: gtk::Box,
    historic_graphic: BasicGraphic,
    current_value_area: gtk::DrawingArea,
    description_label: gtk::Label,
    current_value: f32,
}

trait HasCurrentValue {
    fn set_current_value(&mut self, value: f32);
}

impl Graphic {
    pub fn new() -> Graphic {
        let glade_src = include_str!("../data/graphic.ui");
        let builder = Builder::new_from_string(glade_src).unwrap();

        unsafe {
            let instance = Graphic {
                graphic_grid: builder.get_object("graphic_grid").unwrap(),
                title_label: builder.get_object("title_label").unwrap(),
                current_value_label: builder.get_object("current_value_label").unwrap(),
                divisions_values_box: builder.get_object("divisions_values_box").unwrap(),
                current_value_area: builder.get_object("current_value_area").unwrap(),
                description_label: builder.get_object("description_label").unwrap(),
                historic_graphic: BasicGraphic::new(),
                current_value: 0f32,
            };

            instance.graphic_grid.attach(instance.historic_graphic.get_widget(),
                                         1, 1, 1, 1);
            instance
        }
    }

    pub fn get_widget (&self) -> &gtk::Grid {
       return  &self.graphic_grid;
    }

    pub fn set_title (&mut self, title: &str) {
        self.title_label.set_text(title);
    }
}

impl HasCurrentValue for Graphic {
    fn set_current_value(&mut self, value: f32) {
        self.current_value = value;
        self.historic_graphic.set_current_value(value);
    }
}

impl BasicGraphic {
    pub fn new() -> BasicGraphic {
        let glade_src = include_str!("../data/graphic.ui");
        let builder = Builder::new_from_string(glade_src).unwrap();

        unsafe {
            BasicGraphic {
                historic_area: builder.get_object("historic_area").unwrap(),
                current_value: 0f32,
            }
        }
    }

    pub fn get_widget (&self) -> &gtk::DrawingArea {
       return  &self.historic_area;
    }
}

impl HasCurrentValue for BasicGraphic {
    fn set_current_value(&mut self, value: f32) {
        self.current_value = value;
    }
}

struct TestWid {
    value: f32,
}


