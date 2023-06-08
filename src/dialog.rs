use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk::{self, traits::WidgetExt, prelude::*}, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::adw::{Application, ApplicationWindow, Window, ToastOverlay, Toast, HeaderBar, MessageDialog, ResponseAppearance};
use relm4_macros::view;

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

#[relm4::component]
impl SimpleComponent for Dialog {
    type Init = bool;
    type Input = DialogInput;
    type Output = DialogOutput;

    view! {
        #[root]
        MessageDialog::new(None, 
                            Some("Discard changes and close?"), 
                            Some("All unsaved changes will be lost")
        ) {
                set_modal: true,
                #[watch]
                set_visible: model.shown,
                add_response: ("cancel", "Cancel"),
                add_response: ("discard", "Discard"),
                set_response_appearance: ("discard", ResponseAppearance::Destructive),
                connect_response[sender] => move |_, response| {
                    sender.input(if response=="cancel" {
                        DialogInput::Cancel
                    } else {
                        DialogInput::Discard
                    })
                }
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