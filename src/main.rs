use gtk::prelude::*;
use relm4::{gtk::{self, traits::WidgetExt}, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, set_global_css};
use relm4::adw::{Application, ApplicationWindow, Window, ToastOverlay, Toast};
use tracker;

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon() -> &'static str {
    ICON_LIST.iter().choose().expect("Could not choose a random icon")
}

#[derive(Debug)]
enum AppInput {
    Change_1,
    Change_2,
}

#[tracker::track]
struct AppModel {
    icon1: &str,
    icon2: &str,
    identical: bool,
}

impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = &str;

    view! {
        ApplicationWindow {
            set_title: "Icons",
            set_default_height: 300,
            set_default_width: 300,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical;
                set_spacing: 20,
                set_margin_all: 20,

                gtk::
            }
        }
    }

    fn init(
            icon1: Self::Init,
            icon2: Self::Init,
            identical: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = AppModel{icon1: random_icon(), icon2: random_icon(), identical: false, tracker: 0};
        let widgets = view_output!();
        set_global_css(".identical {background: #8ff0a4}");
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::Change_1 => {
                
            }
        }
    }
}