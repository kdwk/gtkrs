#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack};
use relm4::prelude::*;
use relm4_macros::*;
use gtkrs::{header::{Header, HeaderOutput}, dialog::{Dialog, DialogOutput, DialogInput}, stack::{Stack}};

struct App {
    mode: AppMode,
    header: Controller<Header>,
    dialog: Option<Controller<Dialog>>,
    stack: Controller<Stack>,
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
    CloseDialog,
    CloseRequest,
    CloseWindow,
}

#[relm4::component]
impl Component for App {
    type Init = AppMode;
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        Window {
            set_default_width: 500,
            set_default_height: 300,

            Box {
                set_orientation: gtk::Orientation::Vertical,
                model.header.widget(),
                model.stack.widget(),
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
        let dialog: Option<Controller<Dialog>> = None;
        let stack: Controller<Stack> = Stack::builder().launch(()).detach();
        let stack_ref: Option<ViewStack> = Some(stack.widget().clone());
        let header: Controller<Header> = Header::builder()
                                                    .launch(stack_ref)
                                                    .forward(sender.input_sender(), |message| match message {
                                                        HeaderOutput::View => AppInput::ChangeView(AppMode::View),
                                                        HeaderOutput::Edit => AppInput::ChangeView(AppMode::Edit),
                                                        HeaderOutput::Export => AppInput::ChangeView(AppMode::Export),
                                                    });
        let model = App { mode: init, header: header, dialog: dialog, stack: stack };
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            AppInput::ChangeView(mode) => self.mode = mode,
            AppInput::CloseRequest => {
                self.dialog = Some(Dialog::builder()
                                                .transient_for(root)
                                                .launch(())
                                                .forward(sender.input_sender(), |message| match message {
                                                    DialogOutput::CloseDialog => AppInput::CloseDialog,
                                                    DialogOutput::CloseWindow => AppInput::CloseWindow,
                                                }));
            },
            AppInput::CloseWindow => {
                self.dialog = None;
                relm4::main_application().quit()
            },
            AppInput::CloseDialog => self.dialog = None,
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Dialog");
    relm4_icons::initialize_icons();
    app.run::<App>(AppMode::View);
}