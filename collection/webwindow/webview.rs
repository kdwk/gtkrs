use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use webkit6::{prelude::*, WebView};

pub struct WebWindow {
    url: String,
}

// #[derive(Debug)]
// enum WebWindowInput {
//     PlaceHolder,
// }

#[relm4::component(pub)]
impl SimpleComponent for WebWindow {
    type Init = String;
    type Input = WebWindowInput;
    type Output = ();

    view! {
        Window {
            set_default_height: 1000,
            set_default_width: 1000,

            Box {
                set_orientation: Orientation::Vertical,
                
                WebView {
                    set_vexpand: true,
                    load_uri: model.url.as_str(),
                }
            }
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
}