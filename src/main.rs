use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use rand::prelude::IteratorRandom;
use relm4::gtk::traits::WidgetExt;
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::adw::{Application, ApplicationWindow, Window, ToastOverlay, Toast};

const ICON_LIST: &[&str] = &[
    "bookmark-new-symbolic",
    "edit-copy-symbolic",
    "edit-cut-symbolic",
    "edit-find-symbolic",
    "starred-symbolic",
    "system-run-symbolic",
    "emoji-objects-symbolic",
    "emoji-nature-symbolic",
    "display-brightness-symbolic",
];

fn random_icon() -> &'static str {
    ICON_LIST.iter().choose(&mut rand::thread_rng()).expect("Could not choose a random icon")
}

#[derive(Debug)]
enum AppInput {
    Change1,
    Change2,
}

#[tracker::track]
struct AppModel {
    icon1: &'static str,
    icon2: &'static str,
    icons_identical: bool,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = ();

    view! {
        Window {
            #[track = "model.changed(AppModel::icons_identical())"]
            set_class_active: ("identical", model.icons_identical),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 20,
                set_margin_all: 20,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_margin_all: 20,

                    gtk::Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::icon1())"]
                        set_icon_name: Some(model.icon1),
                    },
                    gtk::Button {
                        add_css_class: "circular",
                        set_icon_name: "list-add-symbolic",
                        connect_clicked => AppInput::Change1
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 20,
                    set_margin_all: 20,

                    gtk::Image {
                        set_pixel_size: 50,
                        #[track = "model.changed(AppModel::icon2())"]
                        set_icon_name: Some(model.icon2)
                    },
                    gtk::Button {
                        add_css_class: "circular",
                        set_icon_name: "update-symbolic",
                        connect_clicked => AppInput::Change2
                    }
                }

            }
        }
    }

    fn init(
            _params: Self::Init, // Not used so add _
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = AppModel{icon1: random_icon(), icon2: random_icon(), icons_identical: false, tracker: 0};
        let widgets = view_output!();
        relm4::set_global_css(".identical {background: #8ff0a4}");
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppInput::Change1 => {
                self.set_icon1(random_icon());
            }
            AppInput::Change2 => {
                self.set_icon2(random_icon());
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("com.github.Kdwk.Icons");
    app.run::<AppModel>(());
}