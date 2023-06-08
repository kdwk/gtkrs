use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk::{self, traits::WidgetExt, prelude::*}, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::adw::{Application, ApplicationWindow, Window, ToastOverlay, Toast, HeaderBar};
use relm4_macros::view;

pub struct Header;

#[derive(Debug)]
pub enum HeaderOutput {
    Edit,
    View,
    Export,
}

#[relm4::component]
impl SimpleComponent for Header {
    type Init = ();
    type Input = ();
    type Output = HeaderOutput;

    view! {
        #[root]
        HeaderBar {
            #[wrap(Some)]
            set_title_widget = &gtk::Box {
                add_css_class: "linked",
                
                #[name = "group"]
                gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::View).unwrap()
                        }
                    }
                },
                gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Edit).unwrap()
                        }
                    }
                },
                gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),                    
                    set_active: true,
                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() {
                            sender.output(HeaderOutput::Export).unwrap()
                        }
                    }
                },
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = Header;
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

}