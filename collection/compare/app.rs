use relm4::gtk::{prelude::*, Box, Label, Button, Orientation, Align, Image, Video, IconSize, Picture};
use relm4::adw::{prelude::*, Window, HeaderBar, MessageDialog, ViewStack, StatusPage};
use relm4::prelude::*;
use relm4_macros::*;
use libshumate::{prelude::*, SimpleMap, RasterRenderer, MapSource, TileDownloader};

pub struct App {
    map_source: Option<MapSource>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        Window {
            set_default_size: (500, 500),
            set_title: Some("Compare"),

            Box {
                set_orientation: Orientation::Vertical,

                HeaderBar {
                    add_css_class: "flat",
                    set_decoration_layout: Some("close:")
                },
    
                Box {
                    set_orientation: Orientation::Vertical,
    
                    SimpleMap {
                        set_map_source: model.map_source.as_ref(),
                    },
    
                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_halign: Align::Center,
    
                        Picture {
                            set_file: Some("data/turtlerock.jpg")
                        }
                    },
    
                    Label {
                        set_halign: Align::Start,
                        add_css_class: "title-1",
                        set_label: "Turtle Rock"
                    },
    
                    Box {
                        set_orientation: Orientation::Horizontal,
                        set_margin_all: 10,
    
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
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
            let model = App {map_source: Some(RasterRenderer::from_url("https://tile.openstreetmap.org/#Z#/#X#/#Y#.png"))};
            let widgets = view_output!();
            ComponentParts { model: model, widgets: widgets }
    }
}