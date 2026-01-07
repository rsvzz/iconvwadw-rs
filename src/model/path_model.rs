
use gtk::glib;
use glib::prelude::*;
use glib::{Object, Properties, subclass::prelude::*};
use std::cell::RefCell;

glib::wrapper! {
    pub struct PathModel(ObjectSubclass<imp::PathModel>);
}

impl PathModel {

    pub fn new(path: String, name: String) -> Self {
        Object::builder()
            .property("name", name)
            .property("path", path)
            .build()
    }
}

impl Default for PathModel {
      fn default() -> Self {
        // Empty
       Object::builder()
            .property("name", "")
            .property("path", "")
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::PathModel)]
    pub struct PathModel {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        path: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PathModel {
        const NAME: &'static str = "PathModel";
        type Type = super::PathModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PathModel {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
