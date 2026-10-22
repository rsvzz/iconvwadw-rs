#![windows_subsystem = "windows"]
mod model;
use gtk4 as gtk;
use libadwaita as adw;
use model::{IconItem, LoadPath, LoadSvg, PathIconAdw, PathModel};

use adw::{
    AboutDialog, Application, ApplicationWindow, ColorScheme, EntryRow, PreferencesGroup,
    StyleManager, ViewStack, ViewStackPage, Window, prelude::*,
};
use gtk::gdk::{Display, Texture, gdk_pixbuf::PixbufLoader};
use gtk::glib::{Bytes, Propagation};
use gtk::{
    Align, Box, Builder, Button, GestureClick, GridView, Image, Label, ListView, MenuButton,
    Orientation, Picture, SignalListItemFactory,
};
use gtk::{gio, glib};

use std::cell::RefCell;
use std::rc::Rc;
use std::{env, path::Path};

fn main() {
    let app = Application::builder()
        .application_id("io.github.rsvzz.iconvwadw")
        .build();
    let path = env::current_exe().expect("No path exe");

    app.connect_activate({
        let dir = path.clone();
        move |app| {
            let r_ui: &str;
            let view_ui: &str;

            if cfg!(debug_assertions) {
                r_ui = "../../data/ui/main.ui"; //devmode
                view_ui = "../../data/ui/view_data.ui"; //devmode
            } else {
                r_ui = "../share/iconvwadw/ui/main.ui"; //release
                view_ui = "../share/iconvwadw/ui/view_data.ui"; //release
            }

            let build = Builder::from_file(
                dir.parent()
                    .unwrap()
                    .join(r_ui)
                    .to_string_lossy()
                    .to_string(),
            );

            if cfg!(debug_assertions) {
                let icon_main: Image = build.object("icon_main").unwrap();
                icon_main.set_icon_name(Some(""));
            }

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
            let btn_clear_mode: Button = build.object("btn_clear_mode").unwrap();
            let btn_find: Button = build.object("btn_find").unwrap();
            let adw_pref_search: PreferencesGroup = build.object("adw_pref_search").unwrap();
            let et_search_icon: EntryRow = build.object("et_search_icon").unwrap();

            btn_find.connect_clicked(glib::clone!(
                #[weak]
                adw_pref_search,
                #[weak]
                et_search_icon,
                move |_| {
                    adw_pref_search.set_visible(!adw_pref_search.is_visible());
                    if !adw_pref_search.is_visible() {
                        et_search_icon.set_text("");
                    }
                }
            ));

            let style_manager = StyleManager::default();

            btn_clear_mode.connect_clicked(move |_| {
                if style_manager.is_dark() {
                    style_manager.set_color_scheme(ColorScheme::ForceLight);
                } else {
                    style_manager.set_color_scheme(ColorScheme::ForceDark);
                }
            });

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
            let lbl_icon: Label = build_view.object("lbl_title_icon").unwrap();
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

            factory_grid.connect_bind({
                let build_icon = build_view.clone();
                let _win_icon = win_icon.clone();
                move |_, obj| {
                    let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                    let image: Picture = list_item.child().and_downcast::<Picture>().unwrap();
                    let item = list_item.item().and_downcast::<IconItem>().unwrap();

                    let gesture = GestureClick::new();
                    gesture.connect_pressed({
                        let item_c = item.clone();
                        let view_win = build_icon.clone();
                        let icon_win = _win_icon.clone();
                        let svg_dg = svg_view.clone();
                        move |_, _, _, _| {
                            let icon: Picture = view_win.object("pic_icon").unwrap();
                            //glib from cairo old version ubuntu 24.04 ltc
                            let bytes = svg_dg.get_texture_for_png(item_c.path().to_string());


                            let loader = PixbufLoader::new();
                            if loader.write(&bytes).is_ok() && loader.close().is_ok() {
                                if let Some(pixbuf) = loader.pixbuf() {
                                    // 2. Crear la textura a partir del Pixbuf
                                    let texture = Texture::for_pixbuf(&pixbuf);
                                    icon.set_paintable(Some(&texture));
                                }
                            }
            
                            let lbl_name: Label = view_win.object("lbl_title_icon").unwrap();
                            lbl_name.set_label(&item_c.name());
                            icon_win.set_visible(true);
                            icon_win.present();
                        }
                    });
                    image.add_controller(gesture);

                    item.bind_property("texture", &image, "paintable")
                        .flags(gtk::glib::BindingFlags::SYNC_CREATE)
                        .transform_to(|_, value: Bytes| {
                            // Crear textura desde los bytes
                               let loader = PixbufLoader::new();
                            if loader.write(&value).is_ok() && loader.close().is_ok() {
                                if let Some(pixbuf) = loader.pixbuf() {
                                    // 2. Crear la textura a partir del Pixbuf
                                    let texture = Texture::for_pixbuf(&pixbuf);
                                    Some(texture)
                                
                                }
                                else{
                                    None
                                }
                            }
                            else{
                                None
                            }
                        
                        })
                        .build();
                }
            });

            view_grid.set_factory(Some(&factory_grid));
            //read path
            let path_symb: String;
            let path_scalable: String;

            if cfg!(target_os = "windows") {
                if let Some(root) = dir.parent() {
                    let base = root.parent().unwrap(); //top root of /bin
                    path_symb = base
                        .join("share")
                        .join("icons")
                        .join("Adwaita")
                        .join("symbolic")
                        .display()
                        .to_string();
                    path_scalable = base
                        .join("share")
                        .join("icons")
                        .join("Adwaita")
                        .join("scalable")
                        .display()
                        .to_string();
                } else {
                    path_symb = String::from("/usr/share/icons/Adwaita/symbolic");
                    path_scalable = String::from("/usr/share/icons/Adwaita/scalable");
                }
            } else {
                match env::var("SNAP") {
                    Ok(snap_path) => {
                        let symb_path =
                            Path::new(&snap_path).join("usr/share/icons/Adwaita/symbolic");
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
            }

            let load_view = Rc::new(RefCell::new(LoadPath::new(
                &list_view,
                path_symb,
                path_scalable,
                &view_grid,
            )));
            load_view.borrow_mut().load_files();

            load_view.borrow_mut().set_files_path(PathIconAdw::SYMBOL);
            load_view
                .borrow_mut()
                .set_filter_loaded_files(PathIconAdw::SYMBOL);

            let load_v = load_view.clone();
            let et_search_cp = et_search_icon.clone();
            let pref_group_cp = adw_pref_search.clone();
            stack.connect_visible_child_notify(move |stack| {
                if let Some(child) = stack.visible_child() {
                    let page: ViewStackPage = stack.page(&child);
                    et_search_cp.set_text("");
                    pref_group_cp.set_visible(false);
                    if page.name().unwrap_or_default() == "symbol" {
                        load_v.borrow().set_files_path(PathIconAdw::SYMBOL);
                        load_v
                            .borrow_mut()
                            .set_filter_loaded_files(PathIconAdw::SYMBOL);
                    } else {
                        load_v.borrow().set_files_path(PathIconAdw::SCALABLE);
                        load_v
                            .borrow_mut()
                            .set_filter_loaded_files(PathIconAdw::SCALABLE);
                    }
                }
            });
            let load_view_cp = load_view.clone();
            let stack_cp = stack.clone();

            et_search_icon.connect_changed(move |row| {
                //println!("{}", row.text());
                if row.text().len() > 0 {
                    load_view_cp
                        .borrow()
                        .set_filter_items_show(row.text().to_string());
                } else {
                    if let Some(child) = stack_cp.visible_child() {
                        let page: ViewStackPage = stack.page(&child);
                        if page.name().unwrap_or_default() == "symbol" {
                            load_view_cp
                                .borrow_mut()
                                .set_files_path(PathIconAdw::SYMBOL);
                        } else {
                            load_view_cp
                                .borrow_mut()
                                .set_files_path(PathIconAdw::SCALABLE);
                        }
                    }
                }
            });

            let factory = SignalListItemFactory::new();

            factory.connect_setup(move |_, obj| {
                let list_item = obj.downcast_ref::<gtk::ListItem>().unwrap();
                let label = Label::builder().margin_top(5).margin_bottom(5).build();

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
                    let about_ui: &str;

                    if cfg!(debug_assertions) {
                        about_ui = "../../data/ui/about.ui"; //devmode
                    } else {
                        about_ui = "../share/iconvwadw/ui/about.ui"; //release
                    }

                    let about_build = Builder::from_file(
                        _dir.parent()
                            .unwrap()
                            .join(about_ui)
                            .to_string_lossy()
                            .to_string(),
                    );

                    let _dialog: AboutDialog = about_build.object("about_dialog").unwrap();

                    if cfg!(debug_assertions) {
                        _dialog.set_application_icon("");
                    }

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
