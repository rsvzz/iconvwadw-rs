mod model;
use model::{IconItem, LoadPath, LoadSvg, PathIconAdw, PathModel};

use adw::{
    AboutDialog, Application, ApplicationWindow, ViewStack, ViewStackPage, Window, prelude::*,
};
use gtk::gdk::{Display, Texture};
use gtk::gio;
use gtk::glib::{Bytes, Propagation, MainContext};
use gtk::{
    Align, Box, Builder, Button, GestureClick, GridView, Image, Label, ListView, MenuButton,
    Orientation, Picture, SignalListItemFactory,
};

use std::path::Path;
use std;
use std::{env, thread};

fn main() {
    let app = Application::builder()
        .application_id("io.github.rsvzz.iconvwadw")
        .build();
    let path = env::current_exe().expect("No path exe");

    app.connect_activate({
        let dir = path.clone();
        move |app| {
            //let r_ui = "../../data/ui/main.ui"; //devmode
            //let view_ui = "../../data/ui/view_data.ui"; //devmode

            let r_ui = "../share/iconvwadw/ui/main.ui"; //release
            let view_ui = "../share/iconvwadw/ui/view_data.ui"; //release

            let build = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(r_ui)
                    .to_string_lossy()
                    .to_string(),
            );

            let build_view = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(view_ui)
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

            let win_icon: Window = build_view.object("win_icon_view").unwrap();
            win_icon.set_transient_for(Some(&window));
            win_icon.set_modal(true);
            win_icon.set_resizable(false);

            let btn_copy: Button = build_view.object("btn_copy").unwrap();
            let lbl_icon: Label = build_view.object("lbl_icon_name").unwrap();
            btn_copy.connect_clicked({
                let lbl_icon = lbl_icon.clone();
                let win_ic = win_icon.clone();
                move |_| {
                    if let Some(display) = Display::default() {
                        let clipboard = display.clipboard();
                        clipboard.set_text(&lbl_icon.label());
                        win_ic.set_visible(false);
                    }
                }
            });

            win_icon.connect_close_request(|win| {
                win.set_visible(false);
                Propagation::Stop
            });
            let svg_view = LoadSvg::new(150, 150); //view click

            let (tx, rx) = std::sync::mpsc::channel::<cairo::glib::Bytes>();

            factory_grid.connect_bind({

                let build_icon = build_view.clone();
                let _win_icon = win_icon.clone();
                let svg_v = svg_view.clone();
                move |_, obj| {
                    let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                    let image: Picture = list_item.child().and_downcast::<Picture>().unwrap();
                    let item = list_item.item().and_downcast::<IconItem>().unwrap();
                    let path_icon = item.path().to_string();
                    let tx_cp = tx.clone();
                    let svg_cp = svg_v.clone();

                     _ = thread::spawn(move || {
                        let bytes = svg_cp.get_texture_for_png(path_icon);
                        tx_cp.send(bytes).unwrap();
                       
                    }).join();

                    let img = image.clone();
                    let rx_cp = rx.try_recv().clone();
                    let item_cp = item.clone();
                   MainContext::default().spawn_local(async move {
                        if let Ok(bytes) = rx_cp.clone() {
                            if let Ok(texture_byte) =
                                Texture::from_bytes(&Bytes::from_owned(bytes))
                            {
                                item_cp.set_texture(&texture_byte);
                                img.set_paintable(Some(&texture_byte));
                            }
                        }
                    });
                    //image.set_paintable(Some(&item.texture().unwrap()));

                    let gesture = GestureClick::new();
                    gesture.connect_pressed({
                        let item_c = item.clone();
                        let view_win = build_icon.clone();
                        let icon_win = _win_icon.clone();
                        move |_, _, _, _| {
                            let icon: Picture = view_win.object("pic_icon").unwrap();
                            //glib from cairo old version
                            let svg_dg = LoadSvg::new(150, 150);
                            let _texture_bytes: cairo::glib::Bytes =
                                svg_dg.get_texture_for_png(item_c.path().to_string());

                            if let Ok(texture) =
                                Texture::from_bytes(&gtk::glib::Bytes::from_owned(_texture_bytes))
                            {
                                icon.set_paintable(Some(&texture));
                            }

                            let lbl_name: Label = view_win.object("lbl_icon_name").unwrap();
                            lbl_name.set_label(&item_c.name());
                            icon_win.set_visible(true);
                            icon_win.present();
                        }
                    });
                    image.add_controller(gesture);
                }
            });

            view_grid.set_factory(Some(&factory_grid));
            //read path
            let path_symb: String;
            let path_scalable: String;
            //exist SNAP
            match env::var("SNAP") {
                Ok(snap_path) => {
                    let symb_path = Path::new(&snap_path).join("usr/share/icons/Adwaita/symbolic");
                    let scalable_path =
                        Path::new(&snap_path).join("usr/share/icons/Adwaita/scalable");

                    path_symb = symb_path.display().to_string();
                    path_scalable = scalable_path.display().to_string();
                }
                Err(_) => {
                    path_symb = String::from("/usr/share/icons/Adwaita/symbolic");
                    path_scalable = String::from("/usr/share/icons/Adwaita/scalable");
                }
            }

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

            //add menu

            let menu = gio::Menu::new();
            menu.append(Some("About"), Some("app.about"));

            let about_opt = gio::SimpleAction::new("about", None);

            about_opt.connect_activate({
                let _win = window.clone();
                let _dir = dir.clone();
                move |_, _| {
                    //let about_ui = "../../data/ui/about.ui"; //devmode
                    let about_ui = "../share/iconvwadw/ui/about.ui"; //release

                    let about_build = Builder::from_file(
                        _dir.parent()
                            .unwrap()
                            .join(about_ui)
                            .to_string_lossy()
                            .to_string(),
                    );

                    let _dialog: AboutDialog = about_build.object("about_dialog").unwrap();

                    _dialog.present(Some(&_win));
                }
            });

            app.add_action(&about_opt);

            let button_menu: MenuButton = build.object("menu_option").unwrap();
            button_menu.set_menu_model(Some(&menu));

            list_view.set_factory(Some(&factory));
            window.present();
        }
    });

    app.run();
}
