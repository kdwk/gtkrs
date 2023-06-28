use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;

pub struct App {
    label: String,
    button_visible: bool,
}

#[derive(Debug)]
pub enum AppInput {
    ButtonClicked,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        Window {
            set_default_size: (500, 500),
            set_title: Some("Compare"),

            Box {
                set_orientation: Orientation::Vertical,

                HeaderBar {
                    add_css_class: "flat",
                    set_decoration_layout: Some(":close"),
                },

                Box {
                    set_orientation: Orientation::Vertical,
                    set_vexpand: true,
                    set_valign: Align::Center,
    
                    Label {
                        add_css_class: "title-1",
                        set_label: "+"
                    },
    
                    Label {
                        #[watch]
                        set_label: &format!("{}", model.label),
                    },

                    Button {
                        set_label: "Show Label",
                        set_hexpand: false,
                        #[watch]
                        set_visible: model.button_visible,
                        connect_clicked => AppInput::ButtonClicked,
                    }
                }               
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = App { label: String::from(""), button_visible: true };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::ButtonClicked => {
                self.button_visible = false;
                self.label = String::from("Success");
            }
        }
    }
}