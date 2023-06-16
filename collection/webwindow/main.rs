#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use crate::webview::WebWindow;

struct Entry {
    url: String
}

#[derive(Debug)]
enum EntryInput {
    CloseWebWindow,
}

#[relm4::factory]
impl FactoryComponent for Entry {
    todo!();
}

struct App;

#[derive(Debug)]
enum AppInput {
    PlaceHolder,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        Window {
            set_default_height: 1000,
            set_default_width: 1000,
            set_title: Some(""),

        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = App {  };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::PlaceHolder => {},
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Web");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}