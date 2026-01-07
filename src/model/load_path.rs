use gtk::{ListView, SingleSelection,};
use gtk::gio::{prelude::*, ListStore, ListModel};
use crate::model::PathModel;
use std::fs;
use std::path::Path;
#[derive(Debug, PartialEq, Eq)]
pub enum PathIconAdw {
    SYMBOL,
    SCALABLE
}
#[derive(Clone)]
pub struct LoadPath{
    list_view: ListView,
    path_sym: String,
    path_sca: String,
}

impl LoadPath{
    pub fn new(view: &ListView, symb: String, scal: String) -> Self{
        LoadPath{
            list_view: view.clone(),
            path_sym: symb,
            path_sca: scal,
        }
    }
    pub fn set_files_path(&self, mtype: PathIconAdw){
         let mut path = Path::new(&self.path_sym);
        if mtype == PathIconAdw::SCALABLE{
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
                  store.append(&PathModel::new(dir.to_string_lossy().to_string(), nombre.to_string_lossy().to_string()));
                }
            }

            let selectmode = SingleSelection::new(Some(store.upcast::<ListModel>()));

            self.list_view.set_model(Some(&selectmode));
          }
    }
}