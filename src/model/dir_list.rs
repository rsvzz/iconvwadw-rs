use gtk4 as gtk;
use gtk::glib;
use glib::prelude::*;
use glib::{Object, Properties, subclass::prelude::*};
use gtk::gio::ListStore;
use std::cell::RefCell;

glib::wrapper! {
    pub struct DirList(ObjectSubclass<imp::DirList>);
}
//Icon
impl DirList {
    pub fn new(dir_path: String, dir_name: String, ic_names: &ListStore) -> Self{

        Object::builder()
            .property("dir", dir_path)
            .property("dir_name", dir_name)
            .property("icon_names", ic_names)
            .build()
    }
}

impl Default for DirList {
    fn default() -> Self {
        // Empty
        Object::builder()
            .property("dir", String::new())
            .property("dir_name", String::new())
            .property("icon_names", None::<ListStore>)
            .build()
    }
}

mod imp {
    

    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::DirList)]
    pub struct DirList {
        #[property(get, set)]
        dir: RefCell<String>,
        #[property(get, set)]
        dir_name: RefCell<String>,
        #[property(get, set)]
        icon_names: RefCell<Option<ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DirList {
        const NAME: &'static str = "DirList";
        type Type = super::DirList;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DirList {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}