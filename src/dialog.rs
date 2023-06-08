use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ResponseAppearance};
use relm4::prelude::*;
use relm4_macros::*;

#[derive(Debug)]
pub struct Dialog {
    shown: bool
}

#[derive(Debug)]
pub enum DialogInput {
    Show,
    Discard,
    Cancel,
}

#[derive(Debug)]
pub enum DialogOutput {
    CloseWindow,
}

#[relm4::component(pub)]
impl SimpleComponent for Dialog {
    type Init = bool;
    type Input = DialogInput;
    type Output = DialogOutput;

    view! {
        #[root]
        MessageDialog::new(gtk::Window::NONE, 
                            Some("Discard changes and close?"), 
                            Some("All unsaved changes will be lost")
        ) {
                set_modal: true,
                #[watch]
                set_visible: model.shown,
                add_response: ("cancel", "Cancel"),
                add_response: ("discard", "Discard"),
                set_response_appearance: ("discard", ResponseAppearance::Destructive),
                connect_response: (None, move |_, response| {
                    sender.input(if response=="cancel" {
                        DialogInput::Cancel
                    } else {
                        DialogInput::Discard
                    })
                })
            }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = Dialog{shown: init};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            DialogInput::Show => self.shown = true,
            DialogInput::Cancel => self.shown = false,
            DialogInput::Discard => {
                self.shown = false;
                sender.output(DialogOutput::CloseWindow).unwrap()
            }
        }
    }
}