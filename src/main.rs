use relm4::{prelude::*, gtk::{traits::{GtkWindowExt, BoxExt, ButtonExt}, glib::clone, subclass::text_view}, adw::prelude::*};

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
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    // type Widgets = AppWidgets;
    // fn init_root() -> Self::Root {
    //     adw::Window::builder()
    //         .title("Counter")
    //         .default_height(700)
    //         .default_width(650)
    //         .build()
    // }

    // fn init(
    //         counter: Self::Init,
    //         window: &Self::Root,
    //         sender: ComponentSender<Self>,
    //     ) -> ComponentParts<Self> {
    //     let model = AppModel {counter};
    //     let vbox = gtk::Box::builder()
    //                     .orientation(gtk::Orientation::Vertical)
    //                     .spacing(5)
    //                     .build();
    //     let inc_button = gtk::Button::builder()
    //                         .label("Increment")
    //                         .build();
    //     let dec_button = gtk::Button::builder()
    //                         .label("Decrement")
    //                         .build();
    //     let label = gtk::Label::new(Some(&format!("{}", model.counter)));

    //     vbox.append(&label);
    //     vbox.append(&inc_button);
    //     vbox.append(&dec_button);
    //     window.set_child(Some(&vbox));
        
    //     inc_button.connect_clicked(clone!(@strong sender => move |_| {
    //         sender.input(AppInput::Increment);
    //     }));
    //     dec_button.connect_clicked(clone!(@strong sender => move |_| {
    //         sender.input(AppInput::Decrement);
    //     }));

    //     let widgets = AppWidgets {label};

    //     ComponentParts { model: model, widgets: widgets }
    // }

    view! {
        gtk::Window {
            set_title: Some("Counter"),
            set_default_height: 300,
            set_default_width: 250,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    #[watch]
                    set_label: &format!("{}", model.counter)
                },
                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => Increment
                },
                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => Decrement
                }
            }

        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = AppModel{counter};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets };
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

    fn update_view(&self, widgets: &mut Self::Widgets, sender: ComponentSender<Self>) {
        let counter_string = self.counter.to_string();
        let counter_str: &str = &counter_string;
        widgets.label.set_label(counter_str);
    }
}

fn main() {
    let app = RelmApp::new("Counter");
    app.run::<AppModel>(0);
}
