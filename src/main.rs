use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use rand::prelude::IteratorRandom;
use relm4::factory::FactoryVecDeque;
use relm4::gtk::traits::WidgetExt;
use relm4::prelude::{DynamicIndex, FactoryComponent};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};
use relm4::adw::{Application, ApplicationWindow, Window, ToastOverlay, Toast, HeaderBar};
use relm4_macros::view;
use relm4_icons::icon_name;
use relm4::gtk::prelude::GtkWindowExt;

#[derive(Debug)]
struct Counter {
    counter: i32,
}

#[derive(Debug)]
enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum CounterOutput {
    SendFront(DynamicIndex),
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for Counter {
    type Init = i32;           // Initial counter value
    type Input = CounterMsg;       // Everything you can do to a counter
    type Output = CounterOutput;   // Everything a counter can do
    type CommandOutput = ();
    type Widgets = CounterWidgets; // Auto-created by macro, don't worry about them
    type ParentInput = AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        // UI code for just one counter
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,
            set_margin_all: 20,

            #[name(label)]
            gtk::Label {
                #[watch]
                set_label: &self.counter.to_string(),
            },

            #[name(add_btn)]
            gtk::Button {
                add_css_class: "circular",
                set_icon_name: icon_name::PLUS,
                connect_clicked => CounterMsg::Increment
            },

            #[name(subtract_btn)]
            gtk::Button {
                add_css_class: "circular",
                set_icon_name: icon_name::MINUS,
                connect_clicked => CounterMsg::Decrement
            },

            #[name(move_up_btn)]
            gtk::Button {
                add_css_class: "circular",
                set_icon_name: icon_name::UP,
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveUp(index.clone()))
                }
            },

            #[name(move_down_btn)]
            gtk::Button {
                add_css_class: "circular",
                set_icon_name: icon_name::DOWN,
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::MoveDown(index.clone()))
                }
            },

            #[name(send_front_btn)]
            gtk::Button {
                add_css_class: "circular",
                set_icon_name: icon_name::TOP,
                connect_clicked[sender, index] => move |_| {
                    sender.output(CounterOutput::SendFront(index.clone()))
                }
            }
        }
    }

    fn init_model(init: Self::Init, index: &Self::Index, sender: relm4::FactorySender<Self>) -> Self {
        Self {counter: init }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        Some(match _output {
            CounterOutput::MoveUp(index) => AppMsg::MoveUp(index),
            CounterOutput::MoveDown(index) => AppMsg::MoveDown(index),
            CounterOutput::SendFront(index) => AppMsg::SendFront(index),
        })
    }
}

struct App {
    created_widgets: u32,
    counters: FactoryVecDeque<Counter>,
}

#[derive(Debug)]
enum AppMsg {
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    SendFront(DynamicIndex),
    AddCounter,
    RemoveCounter,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppMsg;

    type Output = ();

    type Init = u32; // corresponding to widgets_created

    view! {
        ApplicationWindow {
            set_title: Some("Counters"),
            set_default_width: 279,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                
                HeaderBar {
                    add_css_class: "flat",
                    set_decoration_layout: Some(":close"),
                },
            
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 5,
                    set_margin_all: 5,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_spacing: 20,
                        set_margin_all: 20,

                        gtk::Button {
                            set_label: "Add",
                            connect_clicked => AppMsg::AddCounter,
                        },
        
                        gtk::Button {
                            set_label: "Remove",
                            connect_clicked => AppMsg::RemoveCounter,
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,

                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            #[local_ref]
                            counter_box -> gtk::Box {
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 5,
                            }
                        }
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
        let counters = FactoryVecDeque::new(gtk::Box::default(), sender.input_sender());
        let model = App {
            created_widgets: init,
            counters: counters,
        };
        let counter_box = model.counters.widget();
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::AddCounter => {
                self.counters.guard().push_back(self.created_widgets as i32);
                self.created_widgets += 1;
            }
            AppMsg::RemoveCounter => {
                self.counters.guard().pop_back();
            }
            AppMsg::SendFront(index) => {
                self.counters.guard().move_front(index.current_index());
            }
            AppMsg::MoveDown(index) => {
                let new_index = index.current_index() + 1;
                if new_index < self.counters.len() {
                    self.counters.guard().move_to(index.current_index(), new_index);
                }
            }
            AppMsg::MoveUp(index) => {
                if index.current_index() > 0 {
                    let new_index = index.current_index() - 1;
                    self.counters.guard().move_to(index.current_index(), new_index);
                }
            }
        }
    }

}

fn main () {
    let app = RelmApp::new("com.github.kdwk.gtkrs");
    relm4_icons::initialize_icons();
    app.run::<App>(0);
}