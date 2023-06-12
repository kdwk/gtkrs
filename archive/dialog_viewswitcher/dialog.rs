use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ResponseAppearance};
use relm4::prelude::*;
use relm4_macros::*;

#[derive(Debug)]
pub struct Dialog;

#[derive(Debug)]
pub enum DialogInput {
    Discard,
    Cancel,
}

#[derive(Debug)]
pub enum DialogOutput {
    CloseDialog,
    CloseWindow,
}

#[relm4::component(pub)]
impl SimpleComponent for Dialog {
    type Init = ();
    type Input = DialogInput;
    type Output = DialogOutput;

    view! {
        dialog = MessageDialog::new(gtk::Window::NONE, 
                            Some("Discard changes and close?"), 
                            Some("All unsaved changes will be lost")
        ) {
                set_modal: true,
                set_visible: true,
                add_response: ("cancel", "Cancel"),
                add_response: ("discard", "Discard"),
                set_response_appearance: ("discard", ResponseAppearance::Destructive),
                connect_response: (None, move |_, response| {
                    sender.input(if response=="cancel" {
                        DialogInput::Cancel
                    } else {
                        DialogInput::Discard
                    })
                }),
                connect_close_request => move |_| {
                    gtk::Inhibit(true)
                }
            }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = Dialog{};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DialogInput::Cancel => {
                sender.output(DialogOutput::CloseDialog).unwrap()
            },
            DialogInput::Discard => {
                sender.output(DialogOutput::CloseWindow).unwrap()
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, output: relm4::Sender<Self::Output>) {
        widgets.dialog.destroy();
    }
}