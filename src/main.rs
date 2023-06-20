#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video, Entry, InputHints, InputPurpose, EntryBuffer};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::{prelude::{*, FactoryComponent}, factory::FactoryVecDeque};
use relm4_macros::*;
use webkit6::WebView;
use gtkrs::webwindow::{WebWindow, self};

type WebWindowControlBarInit = (u32, String, WebWindow);

struct WebWindowControlBar {
    id: u32,
    url: String,
    webwindow: WebWindow
}

#[derive(Debug)]
enum WebWindowControlBarInput {
    Back,
    Forward,
    Close,
}

#[derive(Debug)]
enum WebWindowControlBarOutput {
    RemoveMePlease(u32), // pass the id
}

#[relm4::factory]
impl FactoryComponent for WebWindowControlBar {
    type Init = WebWindowControlBarInit;
    type Input = WebWindowControlBarInput;
    type Output = WebWindowControlBarOutput;
    type CommandOutput = ();
    type Widgets = WebWindowControlBarWidgets;
    type ParentInput = AppInput;
    type ParentWidget = Box;

    view! {
        Box {
            set_orientation: Orientation::Horizontal,
            set_spacing:10,
            set_margin_all: 20,
            
            #[name(back_btn)]
            Button {
                add_css_class: "circular",
                add_css_class: "toolbar-button",
                set_icon_name: "left",
                #[watch]
                set_sensitive: model.web_view.can_go_back(),
                connect_clicked => WebWindowControlBarInput::Back,
            },

            #[name(forward_btn)]
            Button {
                add_css_class: "circular",
                add_css_class: "toolbar-button",
                set_icon_name: "right",
                #[watch]
                set_sensitive: model.web_view.can_go_forward(),
                connect_clicked => WebWindowControlBarInput::Forward,
            },

            Label {
                set_hexpand: true,
                set_label: &self.url,
            },

            Button {
                add_css_class: "circular",
                add_css_class: "toolbar-button",
                set_icon_name: "cross",
                connect_clicked => WebWindowControlBarInput::Close,
            }
        }
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            WebWindowControlBarInput::Close => {
                self.webwindow.widgets().web_window.destroy();
                sender.output(WebWindowControlBarOutput::RemoveMePlease(self.id));
            },
            WebWindowControlBarInput::Back => {
                self.webwindow.widgets().web_window.go_back();
            },
            WebWindowControlBarInput::Forward => {
                self.webwindow.widgets().web_window.go_forward();
            },
        }
    }

    fn init_model(init: Self::Init, index: &Self::Index, sender: FactorySender<Self>) -> Self {
        Self { id: init.0, url: init.1, webwindow: init.2 }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        Some( match _output {
            WebWindowControlBarOutput::RemoveMePlease(id) => AppInput::RemoveWebWindowControlBar(id)
        })
    }
}

struct App {
    url_entry_buffer: EntryBuffer,
    used_ids: Vec<u32>,
    webwindowcontrolbars: FactoryVecDeque<WebWindowControlBar>,
}

#[derive(Debug)]
enum AppInput {
    NewWebWindow, // Also handles adding a WebWindowControlBar
    RemoveWebWindowControlBar(u32),
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
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_hexpand: true,
                        set_margin_all: 5,
                        set_halign: Align::Center,

                        Entry {
                            set_hexpand: true,
                            set_halign: Align::Fill,
                            #[watch]
                            set_buffer: &model.url_entry_buffer,
                            set_input_purpose: InputPurpose::Url,
                            set_input_hints: InputHints::NO_SPELLCHECK,
                        },
        
                        Button {
                            set_halign: Align::End,
                            set_icon_name: "plus",
                            connect_clicked => AppInput::NewWebWindow,
                        }
                    },

                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_hexpand: true,
                        set_halign: Align::Center,

                        Box {
                            set_orientation: Orientation::Vertical,

                            #[local_ref]
                            webwindowcontrolbar_box -> Box {
                                set_orientation: Orientation::Vertical,
                                set_spacing: 5,
                            }
                        }

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
        let webwindowcontrolbars = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = App { webwindowcontrolbars: webwindowcontrolbars, url_entry_buffer: EntryBuffer::default(), used_ids: vec![] };
        let webwindowcontrolbar_box = model.webwindowcontrolbars.widget();
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::NewWebWindow => {
                let url = String::from(self.url_entry_buffer.text());
                let new_webwindow = WebWindow::builder().launch(url).detach();
                let id: u32 = 0;
                for proposed_id in 0.. {
                    if !self.used_ids.iter().any(|&i| i==proposed_id) {
                        id = proposed_id;
                        break
                    }
                }
                self.webwindowcontrolbars.guard().push_back((id, url, new_webwindow.clone())); // TODO
                self.url_entry_buffer = EntryBuffer::default();
            },

            AppInput::RemoveWebWindowControlBar(id) => {
                for index in 0..self.webwindowcontrolbars.len() {
                    if self.webwindowcontrolbars[index].id == id {
                        self.webwindowcontrolbars.remove(index);
                    }
                }
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Web");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}