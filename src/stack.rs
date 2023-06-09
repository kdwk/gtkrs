use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, ViewStackPage};
use relm4::prelude::*;
use relm4_macros::*;
use relm4_icons::icon_name;
use crate::{header::{Header, HeaderOutput}, dialog::{Dialog, DialogOutput, DialogInput}};

pub struct Stack;

#[relm4::component(pub)]
impl SimpleComponent for Stack {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        ViewStack {
            set_vexpand: true,

            ViewStackPage {
                set_name: "page1",
                set_title: "View",
                set_icon_name: relm4_icons::RICH_TEXT,
                set_use_underline: true,

                Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for View"
                    }
                }
            },

            ViewStackPage {
                set_name: "page2",
                set_title: "Edit",
                set_icon_name: relm4_icons::EDIT,
                set_use_underline: true,

                Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for Edit"
                    }
                }
            },

            ViewStackPage {
                set_name: "page3",
                set_title: "Export",
                set_icon_name: relm4_icons::SHARE,
                set_use_underline: true,

                Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,

                    Label {
                        set_label: "Placeholder for Export"
                    }
                }
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