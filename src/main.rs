use adw::{Application, ApplicationWindow, prelude::*};
use gtk::{Builder, prelude::*};

use std::env;


fn main() {
     let app = Application::builder()
        .application_id("io.github.rsvzz.iconvwadw")
        .build();
    let path = env::current_exe().expect("No path exe");

    app.connect_activate({ 
        let dir = path.clone();
        move |app|{

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


         window.present();


        }
    });

    app.run();
}
