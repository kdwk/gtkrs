#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video, Entry, InputHints, InputPurpose, EntryBuffer};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use gtkrs::webwindow::WebWindow;

struct App {
    url_entry_buffer: EntryBuffer,
}

#[derive(Debug)]
enum AppInput {
    NewWebWindow(String),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        Window {
            set_default_height: 300,
            set_default_width: 300,
            set_title: Some(""),
            add_css_class: "devel",

            Box {
                set_orientation: Orientation::Vertical,
    
                HeaderBar {
                    set_decoration_layout: Some(":close"),
                    add_css_class: "flat",
                },
    
                Box {
                    set_orientation: Orientation::Vertical,
                    set_vexpand: true,
                    set_margin_all: 20,
                    set_spacing: 50,
                    set_valign: Align::Center,

                    Entry {
                        #[watch]
                        set_buffer: &model.url_entry_buffer,
                        set_input_purpose: InputPurpose::Url,
                        set_input_hints: InputHints::NO_SPELLCHECK,
                    },
    
                    Button {
                        set_halign: Align::Center,
                        add_css_class: "pill",
                        set_label: "New WebWindow",
                        connect_clicked => AppInput::NewWebWindow(String::from("https://apple.com"))
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
        let model = App { url_entry_buffer: EntryBuffer::default() };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::NewWebWindow(url) => {
                let url = String::from(self.url_entry_buffer.text());
                let new_webwindow = WebWindow::builder().launch(url).detach();
                self.url_entry_buffer = EntryBuffer::default();
            },
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Web");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}