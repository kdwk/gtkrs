use relm4::gtk::{prelude::*, Box, Label};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewSwitcherTitle, ViewStack};
use relm4::prelude::*;
use relm4_macros::*;
use crate::stack::Stack;

pub struct Header {
    stack: Option<ViewStack>
}

#[derive(Debug)]
pub enum HeaderOutput {
    Edit,
    View,
    Export,
}

#[relm4::component(pub)]
impl SimpleComponent for Header {
    type Init = Option<ViewStack>;
    type Input = ();
    type Output = HeaderOutput;

    view! {
        #[root]
        HeaderBar {
            add_css_class: "flat",
            // set_title_widget = &gtk::Box {
            //     add_css_class: "linked",
                
            //     #[name = "group"]
            //     gtk::ToggleButton {
            //         set_label: "View",
            //         set_active: true,
            //         connect_toggled[sender] => move |btn| {
            //             if btn.is_active() {
            //                 sender.output(HeaderOutput::View).unwrap()
            //             }
            //         }
            //     },
            //     gtk::ToggleButton {
            //         set_label: "Edit",
            //         set_group: Some(&group),
            //         set_active: true,
            //         connect_toggled[sender] => move |btn| {
            //             if btn.is_active() {
            //                 sender.output(HeaderOutput::Edit).unwrap()
            //             }
            //         }
            //     },
            //     gtk::ToggleButton {
            //         set_label: "Export",
            //         set_group: Some(&group),                    
            //         set_active: true,
            //         connect_toggled[sender] => move |btn| {
            //             if btn.is_active() {
            //                 sender.output(HeaderOutput::Export).unwrap()
            //             }
            //         }
            //     },
            // }
            #[wrap(Some)]
            set_title_widget = &ViewSwitcherTitle {
                set_stack: model.stack.as_ref(),
                set_title: "Try View Switcher",
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let stack: Option<ViewStack> = init;
        let model = Header {stack: stack};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

}