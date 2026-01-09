mod model;
use model::{IconItem, LoadPath, LoadSvg, PathIconAdw, PathModel};

use adw::{Application, ApplicationWindow, ViewStack, ViewStackPage, prelude::*};
use gtk::{
    Align, Box, Builder, GridView, Label, ListView, Orientation, Picture, SignalListItemFactory,
    Image,
};
use gtk::gdk::{Texture};

use std::env;

fn main() {
    let app = Application::builder()
        .application_id("io.github.rsvzz.iconvwadw")
        .build();
    let path = env::current_exe().expect("No path exe");

    app.connect_activate({
        let dir = path.clone();
        move |app| {
            let r_ui = "../../data/ui/main.ui";

            let build = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(r_ui)
                    .to_string_lossy()
                    .to_string(),
            );

            let window: ApplicationWindow = build.object("window").unwrap();
            window.set_application(Some(app));

            let stack: ViewStack = build.object("stack_view").unwrap();
            let list_view: ListView = build.object("list_folder_icon").unwrap();
            let view_grid: GridView = build.object("grid_icon").unwrap();

            let factory_grid = SignalListItemFactory::new();

            factory_grid.connect_setup(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let image = Picture::builder()
                    .width_request(60)
                    .height_request(60)
                    .margin_top(5)
                    .build();

                list_item.set_child(Some(&image));
            });

            let load_svg = LoadSvg::new(60, 60);

            factory_grid.connect_bind({
                let svg = load_svg.clone();
                move |_, obj| {
                    let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                    let image: Picture = list_item.child().and_downcast::<Picture>().unwrap();
                    let item = list_item.item().and_downcast::<IconItem>().unwrap();
                    let texture: Texture = svg.get_texture_for_png(item.path().to_string());
                    image.set_paintable(Some(&texture));
                }
            });

            view_grid.set_factory(Some(&factory_grid));
            //read path
            let path_symb = String::from("/usr/share/icons/Adwaita/symbolic");
            let path_scalable = String::from("/usr/share/icons/Adwaita/scalable");

            let load_view = LoadPath::new(&list_view, path_symb, path_scalable, &view_grid);
            load_view.set_files_path(PathIconAdw::SYMBOL);
            stack.connect_visible_child_notify({
                let load_v = load_view.clone();
                move |stack| {
                    if let Some(child) = stack.visible_child() {
                        let page: ViewStackPage = stack.page(&child);
                        if page.name().unwrap_or_default() == "symbol" {
                            load_v.set_files_path(PathIconAdw::SYMBOL);
                        } else {
                            load_v.set_files_path(PathIconAdw::SCALABLE);
                        }
                    }
                }
            });

            let factory = SignalListItemFactory::new();

            factory.connect_setup(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let label = Label::new(Some(""));
                let box_content: Box = Box::builder()
                    .orientation(Orientation::Horizontal)
                    .spacing(5)
                    .margin_top(5)
                    .margin_bottom(5)
                    .build();

                let icon = Image::from_icon_name("go-next-symbolic");
                label.set_halign(Align::Start);
                label.set_margin_start(10);
                icon.set_halign(Align::End);
                icon.set_hexpand(true);
                box_content.append(&label);
                box_content.append(&icon);
                list_item.set_child(Some(&box_content));
            });

            factory.connect_bind(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let box_content: Box = list_item.child().and_downcast::<Box>().unwrap();
                let item = list_item.item().and_downcast::<PathModel>().unwrap();
                if let Some(child) = box_content.first_child() {
                    if let Ok(label) = child.downcast::<Label>() {
                        label.set_text(&item.name());
                    }
                }
            });

            list_view.set_factory(Some(&factory));
            window.present();
        }
    });

    app.run();
}
