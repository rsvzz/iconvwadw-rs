use crate::model::{IconItem, LoadSvg};
use cairo::glib::Bytes;
use gtk::gdk::Texture;
use gtk::gio::{ListModel, ListStore, prelude::*};
use gtk::glib::MainContext;
use gtk::{GridView, NoSelection};
use std::fs;
use std::path::Path;
use std::thread;

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

        let selectmode = NoSelection::new(Some(store.clone().upcast::<ListModel>()));
        self.view_gv.set_model(Some(&selectmode));

        let ic_source = Path::new(&self.path);
        let (tx, rx) = std::sync::mpsc::channel::<Bytes>();
        let svg = LoadSvg::new(60, 60); //view click

        if ic_source.is_dir() {

            for entrada in fs::read_dir(ic_source).unwrap() {
                let entrada = entrada.unwrap();
                let dir = entrada.path();

                if dir.is_file() {
                    let item = IconItem::new(
                        dir.to_string_lossy().to_string(),
                        dir.file_stem().unwrap().to_string_lossy().to_string(),
                    );
                    let path_icon = item.path().to_string();
                    let svg_cp = svg.clone();
                    let tx_cp = tx.clone();
                    _ = thread::spawn(move || {
                        let bytes = svg_cp.get_texture_for_png(path_icon);
                        tx_cp.send(bytes).unwrap();
                    });

                    let item_cp = item.clone();
                    let store_cp = store.clone();
                    let rx_cp = rx.recv().clone();

                    MainContext::default().spawn_local(async move {
                        if let Ok(bytes) = rx_cp {
                            if let Ok(texture) =
                                Texture::from_bytes(&gtk::glib::Bytes::from_owned(bytes))
                            {
                                item_cp.set_texture(&texture);
                                store_cp.append(&item_cp);
                            }
                        }
                    });
                }
            }
        }
    }
}
