#![allow(unused_imports)]
#![allow(unused_variables)]
// use ashpd::zbus::export::futures_core::future;
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
// use ashpd::desktop::file_chooser::{Choice, FileFilter, SelectedFiles};

struct App {
    label: i32,
}

#[derive(Debug)]
enum AppInput {
    OpenFile,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = i32;
    type Input = AppInput;
    type Output = ();

    view! {
        Window {
            set_default_height: 400,
            set_default_width: 300,
            set_resizable: false,
            set_title: Some(""),

            Box {
                set_orientation: Orientation::Vertical,

                HeaderBar {
                    set_decoration_layout: Some(":close"),
                    add_css_class: "flat",
                },

            //     Box {
            //         set_orientation: Orientation::Vertical,
            //         set_valign: Align::Center,
            //         set_vexpand: true,

            //         Label {
            //             add_css_class: "title-1",
            //             #[watch]
            //             set_label: &format!("{}", model.label),
            //         },
    
            //         Box {
            //             set_orientation: Orientation::Horizontal,
            //             set_halign: Align::Center,
            //             set_margin_all: 20,
            //             set_spacing: 10,
    
            //             Button {
            //                 add_css_class: "circular",
            //                 set_icon_name: "plus",
            //                 connect_clicked => AppInput::Increment
            //             },
    
            //             Button {
            //                 add_css_class: "circular",
            //                 set_icon_name: "minus",
            //                 connect_clicked => AppInput::Decrement
            //             }
            //         }
            //     }

                Box {
                    set_orientation: Orientation::Vertical,
                    set_valign: Align::Center,
                    set_vexpand: true,

                    StatusPage {
                        set_icon_name: Some("call-missed"),
                        set_title: "1 Missed Call",
                        set_description: Some("09:38 Missed call from Ted\n11:20 Missed call from Rebecca"),
                    }
                }

                // Video {
                //     set_filename: Some("data/Nextcloud intro.mp4"),
                // }
            }

        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = App { label: init };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::OpenFile => (),
        }
    }
}

// WIP: does not work
// async fn open_file() -> ashpd::Result<()> {
//     let files = SelectedFiles::open_file()
//     .title("open a video")
//     .accept_label("Select")
//     .modal(true)
//     .multiple(false)
//     .send()
//     .await?
//     .response()?;

//     println!("{:#?}", files);

//     Ok(())
// }

fn main () {
    let app = RelmApp::new("com.github.kdwk.Practice");
    relm4_icons::initialize_icons();
    app.run::<App>(0);
}