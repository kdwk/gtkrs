use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{gtk::{self, traits::WidgetExt}, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: isize,
}

struct AppWidgets {
    label: gtk::Label,
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
        gtk::Window {
            set_title: Some("Counter"),
            set_default_height: 200,
            set_default_width: 200,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,
                set_margin_all: 20,

                gtk::Label {
                    #[watch]
                    set_label: &format!("{}", model.counter)
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::Center,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Button {
                        add_css_class: "circular",
                        set_icon_name: "list-add-symbolic",
                        connect_clicked => AppInput::Increment
                    },
                    gtk::Button {
                        add_css_class: "circular",
                        set_icon_name: "list-remove-symbolic",
                        connect_clicked => AppInput::Decrement
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

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
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
