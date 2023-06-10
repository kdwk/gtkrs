use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, ViewStackPage};
use relm4::prelude::*;
use relm4_macros::*;
use relm4_icons::icon_name::*;
use crate::{header::{Header, HeaderOutput}, dialog::{Dialog, DialogOutput, DialogInput}};

pub struct Stack;

#[relm4::component(pub)]
impl SimpleComponent for Stack {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        #[root]
        ViewStack {
            set_vexpand: true,

            add = &Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for View"
                    }
            } -> {
                set_name: Some("page1"),
                set_title: Some("View"),
                set_icon_name: Some("rich-text"),
                set_use_underline: true,
            },

            add = &Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for Edit"
                    }
            } -> {
                set_name: Some("page2"),
                set_title: Some("Edit"),
                set_icon_name: Some("edit"),
                set_use_underline: true,
            },

            add = &Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for Export"
                    }

            } -> {
                set_name: Some("page3"),
                set_title: Some("Export"),
                set_icon_name: Some("share"),
                set_use_underline: true,
            },
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = Stack {};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }
}