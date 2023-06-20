use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage, ToastOverlay, Toast};
use relm4::prelude::*;
use relm4_macros::*;
use webkit6::{prelude::*, WebView};

pub struct WebWindow {
    pub url: String,
}

#[derive(Debug)]
enum WebWindowInput {
    NewSmallWindow,
}

#[relm4::component(pub)]
impl SimpleComponent for WebWindow {
    type Init = String;
    type Input = ();
    type Output = ();

    view! {
        #[name(web_window)]
        Window {
            set_default_height: 1000,
            set_default_width: 1000,

            ToastOverlay {
                Box {
                    set_orientation: Orientation::Vertical,
                    
                    #[name(web_view)]
                    WebView {
                        set_vexpand: true,
                        load_uri: model.url.as_str(),
                        // connect_create => WebWindowInput::NewSmallWindow,
                    }
                }
            },

            present: (),
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = WebWindow { url: init };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    // fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
    //     match message {
    //         WebWindowInput::NewSmallWindow => println!("New small window requested."),
    //     }
    // }

    // fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {
    //     widgets.web_window.destroy();
    // }
}