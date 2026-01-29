use crate::model::{IconItem, LoadSvg};
use glib::{Bytes, ControlFlow};
use gtk::gio::{ListModel, ListStore, prelude::*};
use gtk::glib;
use gtk::{GridView, NoSelection};
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[derive(Clone)]
pub struct LoadIcon {
    view_gv: GridView,
    path: String,
}

pub struct ItemIdex {
    pub id: u32,
    pub bytes: Vec<u8>,
}

impl ItemIdex {
    pub fn new(id: u32, bytes: Vec<u8>) -> Self {
        ItemIdex {
            id: id,
            bytes: bytes,
        }
    }
}

impl LoadIcon {
    pub fn new(source: String, grid_view: &GridView) -> Self {
        LoadIcon {
            view_gv: grid_view.clone(),
            path: source,
        }
    }
    pub fn set_texture_store(&mut self) {
        if let Some(store) = self.view_gv.model() {
            let svg = LoadSvg::new(60, 60); //view click
            let (tx, rx) = mpsc::channel::<ItemIdex>();
            let rx = Arc::new(Mutex::new(rx));
            for i in 0..store.n_items() {
                if let Some(item) = store.item(i).and_downcast::<IconItem>() {
                    let path_icon = item.path().to_string();
                    let svg_cp = svg.clone();
                    let tx_cp = tx.clone();
                    let idex = i.clone();
                    thread::spawn(move || {
                        let _bytes = svg_cp.get_texture_for_png(path_icon);
                        let obj = ItemIdex::new(idex, _bytes);
                        tx_cp.send(obj).unwrap();
                    });

                    glib::idle_add_local({
                        let rx_cp = rx.clone();
                        let store_cp = store.clone();
                        move || {
                            if let Ok(recive) = rx_cp.lock().unwrap().recv() {
                                if let Some(item) = store_cp.item(recive.id) {
                                    // downcast al tipo concreto
                                    let obj = item.downcast::<super::IconItem>().unwrap();
                                    obj.set_texture(Bytes::from_owned(recive.bytes));
                                    ControlFlow::Continue
                                } else {
                                    ControlFlow::Break
                                }
                            } else {
                                ControlFlow::Break
                            }
                        }
                    });
                }
            }
            drop(tx);
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
            self.set_texture_store();
        }
    }
}
