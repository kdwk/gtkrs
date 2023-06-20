#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Video, Entry, InputHints, InputPurpose, EntryBuffer, ScrolledWindow};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::{prelude::{*, FactoryComponent}, factory::FactoryVecDeque};
use relm4_macros::*;
use webkit6::{WebView, prelude::*};
use gtkrs::webwindow::{WebWindow};
use std::collections::HashSet;

struct WebWindowControlBar {
    id: DynamicIndex,
    url: String,
    webwindow: Controller<WebWindow>
}

type WebWindowControlBarInit = (String, Controller<WebWindow>);

#[derive(Debug)]
enum WebWindowControlBarInput {
    Back,
    Forward,
    Close,
    Refresh,
}

#[derive(Debug)]
enum WebWindowControlBarOutput {
    RemoveMePlease(DynamicIndex), // pass the id
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
            set_spacing: 5,
            set_margin_all: 5,
            
            #[name(back_btn)]
            Button {
                add_css_class: "circular",
                set_icon_name: "left",
                connect_clicked => WebWindowControlBarInput::Back,
            },

            #[name(forward_btn)]
            Button {
                add_css_class: "circular",
                set_icon_name: "right",
                connect_clicked => WebWindowControlBarInput::Forward,
            },

            #[name(refresh_btn)]
            Button {
                add_css_class: "circular",
                set_icon_name: "refresh",
                connect_clicked => WebWindowControlBarInput::Refresh,
            },

            Label {
                set_hexpand: true,
                set_halign: Align::Start,
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
                sender.output(WebWindowControlBarOutput::RemoveMePlease(self.id.clone()));
            },
            WebWindowControlBarInput::Back => self.webwindow.widgets().web_view.go_back(),
            WebWindowControlBarInput::Forward => self.webwindow.widgets().web_view.go_forward(),
            WebWindowControlBarInput::Refresh => self.webwindow.widgets().web_view.reload(),
        }
    }

    fn init_model(init: Self::Init, index: &Self::Index, sender: FactorySender<Self>) -> Self {
        Self { id: index.clone(), url: init.0, webwindow: init.1 }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        Some( match _output {
            WebWindowControlBarOutput::RemoveMePlease(id) => AppInput::RemoveWebWindowControlBar(id)
        })
    }
}





struct App {
    url_entry_buffer: EntryBuffer,
    webwindowcontrolbars: FactoryVecDeque<WebWindowControlBar>,
}

#[derive(Debug)]
enum AppInput {
    NewWebWindow, // Also handles adding a WebWindowControlBar
    RemoveWebWindowControlBar(DynamicIndex),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        Window {
            set_default_height: 500,
            set_default_width: 350,
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
                    set_spacing: 3,
                    set_margin_all: 5,

                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_hexpand: true,
                        set_margin_all: 5,
                        set_halign: Align::Fill,

                        Entry {
                            set_hexpand: true,
                            set_halign: Align::Fill,
                            set_margin_all: 5,
                            #[watch]
                            set_buffer: &model.url_entry_buffer,
                            set_placeholder_text: Some("https://gnome.org"),
                            set_input_purpose: InputPurpose::Url,
                            set_input_hints: InputHints::NO_SPELLCHECK,
                        },
        
                        Button {
                            set_margin_all: 5,
                            set_halign: Align::End,
                            set_icon_name: "plus",
                            connect_clicked => AppInput::NewWebWindow,
                        }
                    },
                    
                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_hexpand: true,
                        set_halign: Align::Fill,

                        Box {
                            set_orientation: Orientation::Vertical,

                            #[local_ref]
                            webwindowcontrolbar_box -> Box {
                                set_orientation: Orientation::Vertical,
                                set_spacing: 0,
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
        let model = App { webwindowcontrolbars: webwindowcontrolbars, url_entry_buffer: EntryBuffer::default() };
        let webwindowcontrolbar_box = model.webwindowcontrolbars.widget();
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::NewWebWindow => {
                let url = String::from(self.url_entry_buffer.text());
                let new_webwindow = WebWindow::builder().launch(url.clone()).detach();
                self.webwindowcontrolbars.guard().push_back((url, new_webwindow));
                self.url_entry_buffer = EntryBuffer::default();
            },

            AppInput::RemoveWebWindowControlBar(id) => {
                self.webwindowcontrolbars.guard().remove(id.current_index());
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Spidey");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}