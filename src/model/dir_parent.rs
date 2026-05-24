use crate::model::{DirList, PathIconAdw};

#[derive(Clone)]
pub struct DirParent{
    type_parent : PathIconAdw,
    parent: String,
    list_item : Vec<DirList>
}

impl DirParent {
    pub fn new(t_parent: PathIconAdw, dir_parent: String, items: &Vec<DirList>) -> Self{
        DirParent{
            type_parent: t_parent,
            parent: dir_parent,
            list_item: items.to_vec(),
        }
    }

    pub fn get_parent(&self) -> String{
        self.parent.to_string()
    }

    pub fn get_item_for_parent(&self) -> &[DirList]{
        &self.list_item
    }

    pub  fn get_type_parent(&self) -> &PathIconAdw{
        &self.type_parent
    }
}