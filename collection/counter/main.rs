use relm4::prelude::*;
use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align};
use relm4::adw::{prelude::*, Window, HeaderBar};

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: isize,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    /// The type of the messages that this component can receive.
    type Input = AppInput;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = isize;

    view! {
        Window {
            set_title: Some("Counter"),
            set_default_height: 200,
            set_default_width: 200,

            Box {
                set_orientation: Orientation::Vertical,

                HeaderBar {
                    add_css_class: "flat",
                    set_decoration_layout: Some(":close"),
                },

                Box {
                    set_orientation: Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 20,

                    Label {
                        #[watch]
                        set_label: &format!("{}", model.counter)
                    },
    
                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_halign: Align::Center,
                        set_spacing: 5,
                        set_margin_all: 5,
    
                        Button {
                            add_css_class: "circular",
                            set_icon_name: "list-add-symbolic",
                            connect_clicked => AppInput::Increment
                        },
                        Button {
                            add_css_class: "circular",
                            set_icon_name: "list-remove-symbolic",
                            connect_clicked => AppInput::Decrement
                        }
                    }
                }
            }

        }
    }

    fn init(
            counter: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = AppModel{counter};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => {
                self.counter += 1;
            }
            AppInput::Decrement => {
                self.counter -= 1;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("Counter");
    app.run::<AppModel>(0);
}
