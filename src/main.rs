#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog};
use relm4::prelude::*;
use relm4_macros::*;
use gtkrs::{header::{Header, HeaderOutput}, dialog::{Dialog, DialogOutput, DialogInput}};

struct App {
    mode: AppMode,
    header: Controller<Header>,
    dialog: Controller<Dialog>,
    // stack: ??
}

#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppInput {
    ChangeView(AppMode),
    CloseRequest,
    CloseWindow,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = AppMode;
    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        Window {
            set_default_width: 500,
            set_default_height: 300,

            Box {
                set_orientation: gtk::Orientation::Vertical,
                model.header.widget(),

                Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_valign: gtk::Align::Center,

                    Label {
                        #[watch]
                        set_label: &format!("Placeholder for {:?}", model.mode),
                    }
                }
            },
            
            connect_close_request[sender] => move |_| {
                sender.input(AppInput::CloseRequest);
                gtk::Inhibit(true)
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let header: Controller<Header> = Header::builder()
                                            .launch(())
                                            .forward(sender.input_sender(), |message| match message {
                                                HeaderOutput::View => AppInput::ChangeView(AppMode::View),
                                                HeaderOutput::Edit => AppInput::ChangeView(AppMode::Edit),
                                                HeaderOutput::Export => AppInput::ChangeView(AppMode::Export),
                                            });
        let dialog: Controller<Dialog> = Dialog::builder()
                                            .transient_for(root)
                                            .launch(false)
                                            .forward(sender.input_sender(), |message| match message{
                                                DialogOutput::CloseWindow => AppInput::CloseWindow,
                                            });
        let model = App { mode: init, header: header, dialog: dialog };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::ChangeView(mode) => self.mode = mode,
            AppInput::CloseRequest => self.dialog.sender().send(DialogInput::Show).unwrap(),
            AppInput::CloseWindow => relm4::main_application().quit(),
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Dialog");
    app.run::<App>(AppMode::View);
}