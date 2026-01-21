use crate::model::{IconItem};
use gtk::gio::{ListModel, ListStore, prelude::*};
use gtk::{GridView, NoSelection};
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct LoadIcon {
    view_gv: GridView,
    path: String,
}

impl LoadIcon {
    pub fn new(source: String, grid_view: &GridView) -> Self {
        LoadIcon {
            view_gv: grid_view.clone(),
            path: source,
        }
    }

    pub fn set_data_source(&mut self, source: String) {
        self.path = source;

        let store = ListStore::builder()
            .item_type(IconItem::static_type())
            .build();

        let ic_source = Path::new(&self.path);
        if ic_source.is_dir() {

            for entrada in fs::read_dir(ic_source).unwrap() {
                
                let entrada = entrada.unwrap();
                let dir = entrada.path();

                if dir.is_file() {
                    let item = IconItem::new(
                        dir.to_string_lossy().to_string(),
                        dir.file_stem().unwrap().to_string_lossy().to_string(),
                    );

                    store.append(&item);
                    
                }
            }
        }

        let selectmode = NoSelection::new(Some(store.upcast::<ListModel>()));

        self.view_gv.set_model(Some(&selectmode));
    }
}
