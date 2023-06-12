use relm4::gtk::{prelude::*, Box, Label, TextView, TextBuffer, TextTagTable};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, ViewStackPage, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use relm4_icons::icon_name::*;
use crate::{header::{Header}, dialog::{Dialog, DialogOutput, DialogInput}};

pub struct Stack;

#[relm4::component(pub)]
impl SimpleComponent for Stack {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        ViewStack {
            set_vexpand: true,

            add = &StatusPage {
                set_title: "View",
                set_icon_name: Some("rich-text"),
                set_description: Some("Placeholder for View")
            } -> {
                set_name: Some("page1"),
                set_title: Some("View"),
                set_icon_name: Some("rich-text"),
                set_use_underline: true,
            },

            add = &StatusPage {
                set_title: "Edit",
                set_icon_name: Some("edit"),
                set_description: Some("Placeholder for Edit")
            } -> {
                set_name: Some("page2"),
                set_title: Some("Edit"),
                set_icon_name: Some("edit"),
                set_use_underline: true,
            },

            add = &StatusPage {
                set_title: "Export",
                set_icon_name: Some("share"),
                set_description: Some("Placeholder for Export")
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