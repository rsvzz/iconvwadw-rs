use crate::model::{LoadIcon, PathModel};
use gtk::gio::{ListModel, ListStore, prelude::*};
use gtk::{GridView, ListView, SingleSelection};
use std::{cell::RefCell, fs, path::Path, rc::Rc};
#[derive(Debug, PartialEq, Eq)]
pub enum PathIconAdw {
    SYMBOL,
    SCALABLE,
}
#[derive(Clone)]
pub struct LoadPath {
    list_view: ListView,
    //path symbolic
    path_sym: String,
    //path scalable
    path_sca: String,
    view_grid: GridView,
}

impl LoadPath {
    pub fn new(view: &ListView, symb: String, scal: String, grid_v: &GridView) -> Self {
        LoadPath {
            list_view: view.clone(),
            path_sym: symb,
            path_sca: scal,
            view_grid: grid_v.clone(),
        }
    }
    pub fn set_files_path(&self, mtype: PathIconAdw) {
        let mut path = Path::new(&self.path_sym);
        if mtype == PathIconAdw::SCALABLE {
            path = Path::new(&self.path_sca);
        }

        let store = ListStore::builder()
            .item_type(PathModel::static_type())
            .build();

        if path.is_dir() {
            for entrada in fs::read_dir(path).unwrap() {
                let entrada = entrada.unwrap();
                let dir = entrada.path();

                if let Some(nombre) = dir.file_name() {
                    store.append(&PathModel::new(
                        dir.to_string_lossy().to_string(),
                        nombre.to_string_lossy().to_string(),
                    ));
                }
            }
            let load_icon = Rc::new(RefCell::new(LoadIcon::new(
                String::from(""),
                &self.view_grid.clone(),
            )));

            let selectmode: SingleSelection =
                SingleSelection::new(Some(store.upcast::<ListModel>()));
            if selectmode.n_items() > 0 {
                let item = selectmode
                    .selected_item()
                    .and_downcast::<PathModel>()
                    .unwrap();
                load_icon.borrow_mut().set_data_source(item.path());
            }

            selectmode.connect_selected_notify({
                let load_ic = load_icon.clone();
                move |sel| {
                    let pos = sel.selected();
                    if pos != gtk::INVALID_LIST_POSITION {
                            let item = sel.selected_item().and_downcast::<PathModel>().unwrap();
                            load_ic.borrow_mut().set_data_source(item.path());
                    }
                }
            });
            self.list_view.set_model(Some(&selectmode));
        }
    }
}
