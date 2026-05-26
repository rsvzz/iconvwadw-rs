use crate::model::{DirList, DirParent, LoadIcon, PathModel};
use gtk::gio::{ListModel, ListStore, prelude::*};
use gtk::{GridView, ListView, SingleSelection};
use std::{cell::RefCell, fs, path::Path, rc::Rc};
#[derive(Debug, PartialEq, Eq, Clone)]
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
    list_files: Option<Vec<DirParent>>,
    list_filter: Option<ListStore>,
}

impl LoadPath {
    pub fn new(view: &ListView, symb: String, scal: String, grid_v: &GridView) -> Self {
        LoadPath {
            list_view: view.clone(),
            path_sym: symb,
            path_sca: scal,
            view_grid: grid_v.clone(),
            list_files: None,
            list_filter: None,
        }
    }

    pub fn get_items_filter(&self) -> bool {
        if self.list_filter != None {
            true
        } else {
            false
        }
    }
    ///load files for .svg theme
    fn get_dir_for_path(parent: &Path) -> Vec<DirList> {
        // Vec<DirList> move live obj
        let mut list: Vec<DirList> = Vec::new();

        match fs::read_dir(parent) {
            Ok(_path) => {
                for dir in _path {
                    if let Ok(_dir) = dir {
                        let _n_dir = _dir.path();

                        if _n_dir.is_dir() {
                            let store = ListStore::builder()
                                .item_type(PathModel::static_type())
                                .build();

                            match fs::read_dir(_n_dir.clone()) {
                                Ok(sub_path) => {
                                    for _file in sub_path {
                                        if let Ok(file) = _file {
                                            let _file_path = file.path();

                                            if _file_path.is_file() {
                                                if let Some(file_name) = _file_path.file_name() {
                                                    store.append(&PathModel::new(
                                                        _file_path.to_string_lossy().to_string(),
                                                        file_name.to_string_lossy().to_string(),
                                                        None,
                                                    ));
                                                }
                                            }
                                        }
                                    }

                                    if let Some(name_dir) = _n_dir.file_name() {
                                        list.push(DirList::new(
                                            _n_dir.to_string_lossy().to_string(),
                                            name_dir.to_string_lossy().to_string(),
                                            &store,
                                        ));
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }

        list
    }

    ///load all files .svg theme Adwaita
    pub fn load_files(&mut self) {
        let mut list: Vec<DirParent> = Vec::new();

        let path_scalable = Path::new(&self.path_sca);
        let path_symbole = Path::new(&self.path_sym);
        let dir_ic_symbole = LoadPath::get_dir_for_path(path_symbole);
        let dir_ic_scalable = LoadPath::get_dir_for_path(path_scalable);

        list.push(DirParent::new(
            PathIconAdw::SYMBOL,
            path_symbole.to_string_lossy().to_string(),
            &dir_ic_symbole,
        ));
        list.push(DirParent::new(
            PathIconAdw::SCALABLE,
            path_scalable.to_string_lossy().to_string(),
            &dir_ic_scalable,
        ));

        self.list_files = Some(list);
    }

    pub fn set_filter_items_show(&self, filter: String) {
        if filter.len() > 0 {
            if let Some(items) = &self.list_filter {
                let store = ListStore::builder()
                    .item_type(PathModel::static_type())
                    .build();

                for i in 0..items.n_items() {
                    if let Some(obj) = items.item(i).and_downcast_ref::<PathModel>() {
                        if obj.name().contains(&filter) {
                            store.append(obj);
                        }
                    }
                }

                let mut load_icon = LoadIcon::new(String::from(""), &self.view_grid.clone());

                load_icon.set_data_items(&store);
            }
        }
    }

    pub fn set_filter_loaded_files(&mut self, mtype: PathIconAdw) {
        let mut dir: Option<&DirParent> = None;
        if let Some(list) = &self.list_files {
            for item in list {
                if mtype == *item.get_type_parent() {
                    dir = Some(&item);
                    break;
                }
            }
        }

        let store = ListStore::builder()
            .item_type(PathModel::static_type())
            .build();

        if let Some(list) = dir {
            for ic_item in list.get_item_for_parent() {
                if let Some(items) = ic_item.icon_names() {
                    for i in 0..items.n_items() {
                        if let Some(obj) = items.item(i).and_downcast::<PathModel>() {
                            store.append(&obj);
                        }
                    }
                }
            }
        }

        self.list_filter.replace(store);
    }

    pub fn set_files_path(&self, mtype: PathIconAdw) {
        let mut dir: Option<&DirParent> = None;
        if let Some(list) = &self.list_files {
            for item in list {
                if mtype == *item.get_type_parent() {
                    dir = Some(&item);
                    break;
                }
            }
        }

        let store = ListStore::builder()
            .item_type(PathModel::static_type())
            .build();

        match dir {
            Some(item) => {
                for ic_item in item.get_item_for_parent() {
                    store.append(&PathModel::new(
                        ic_item.dir(),
                        ic_item.dir_name(),
                        ic_item.icon_names().as_ref(),
                    ));
                }
            }
            None => {}
        }

        let load_icon = Rc::new(RefCell::new(LoadIcon::new(
            String::from(""),
            &self.view_grid.clone(),
        )));

        let selectmode: SingleSelection = SingleSelection::new(Some(store.upcast::<ListModel>()));
        if selectmode.n_items() > 0 {
            let item = selectmode
                .selected_item()
                .and_downcast::<PathModel>()
                .unwrap();
            //load_icon.borrow_mut().set_data_source(item.path());
            if let Some(icons) = item.list_icons() {
                load_icon.borrow_mut().set_data_items(&icons);
            }
        }

        selectmode.connect_selected_notify({
            let load_ic = load_icon.clone();
            //let list = self.list_files.clone();
            move |sel| {
                let pos = sel.selected();
                if pos != gtk::INVALID_LIST_POSITION {
                    let item = sel.selected_item().and_downcast::<PathModel>().unwrap();

                    if let Some(icons) = item.list_icons() {
                        load_ic.borrow_mut().set_data_items(&icons);
                    }
                }
            }
        });
        self.list_view.set_model(Some(&selectmode));
        // }
    }
}
