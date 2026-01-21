use gtk::glib;
use glib::{prelude::*};
use glib::{Object, Properties, subclass::prelude::*};
use gtk::gdk::{Texture};
use std::cell::RefCell;

glib::wrapper! {
    pub struct IconItem(ObjectSubclass<imp::IconItem>);
}
//Icon
impl IconItem {

    pub fn new(path: String, name: String) -> Self {
        Object::builder()
            .property("name", name)
            .property("path", path)
            .property("texture", None::<Texture>)
            .build()
    }
}

impl Default for IconItem {
      fn default() -> Self {
        // Empty
       Object::builder()
            .property("name", "")
            .property("path", "")
            .property("texture", None::<Texture>)
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::IconItem)]
    pub struct IconItem {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        path: RefCell<String>,
        #[property(get, set)]
        texture: RefCell<Option<Texture>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IconItem {
        const NAME: &'static str = "IconItem";
        type Type = super::IconItem;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for IconItem {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}
