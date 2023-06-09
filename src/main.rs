#![allow(unused_imports)]
#![allow(unused_variables)]
use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog};
use relm4::prelude::*;
use relm4_macros::*;
use gtkrs::{header::{Header, HeaderOutput}, dialog::{Dialog, DialogOutput, DialogInput}, stack::{Stack}};

struct App {
    mode: AppMode,
    header: Controller<Header>,
    dialog: Option<Controller<Dialog>>,
    stack: Option<&'static ViewStack>,
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
        let dialog: Option<Controller<Dialog>> = None;
        // let dialog: Controller<Dialog> = Dialog::builder()
        //                                     .transient_for(root)
        //                                     .launch(false)
        //                                     .forward(sender.input_sender(), |message| match message{
        //                                         DialogOutput::CloseWindow => AppInput::CloseWindow,
        //                                     });
        let stack: Option<&'static ViewStack> = Some(&Stack::builder().launch(()));
        let header: Controller<Header> = Header::builder()
                                                    .launch(&stack)
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
            // AppInput::CloseRequest => self.dialog.sender().send(DialogInput::Show).unwrap(),
            AppInput::CloseRequest => {
                // let stream = Dialog::builder()
                //                         .transient_for(root)
                //                         .launch(())
                                        // .forward(sender.input_sender(), |message| match message {
                                        //     DialogOutput::CloseWindow => AppInput::CloseWindow,
                                        // }).into_stream();
                // sender.oneshot_command(async move {
                //     let result = stream.recv_one().await;
                // })
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

    fn update_cmd(
            &mut self,
            message: Self::CommandOutput,
            sender: ComponentSender<Self>,
            root: &Self::Root,
        ) {
        
    }
}

fn main() {
    let app = RelmApp::new("com.github.kdwk.Dialog");
    app.run::<App>(AppMode::View);
}