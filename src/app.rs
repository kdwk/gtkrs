use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Image, Video};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use libshumate::{prelude::*, SimpleMap};

struct App {

}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        Window {
            set_default_size: (500, 500),

            HeaderBar {
                add_css_class: "flat",
                set_decoration_layout: Some("close:")
            },

            Box {
                set_orientation: Orientation::Vertical,

                SimpleMap {},

                Box {
                    set_orientation: Orientation::Horizontal,
                    set_halign: Align::Center,

                    Image {
                        from_file: "data/turtlerock.jpg"
                    }
                },

                Label {
                    add_css_class: "title-1",
                    set_label: "Turtle Rock"
                },

                Box {
                    set_orientation: Orientation::Horizontal,

                    Label {
                        set_label: "Joshua Tree National Park",
                    },

                    Label {
                        set_label: "",
                        set_hexpand: true,
                    },

                    Label {
                        set_label: "California",
                    }
                }
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
            let model = App {};
            let widgets = view_output!();
            ComponentParts { model: model, widgets: widgets }
    }
}