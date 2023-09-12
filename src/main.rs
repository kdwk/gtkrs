use relm4::prelude::*;
use relm4::gtk::{prelude::*, Box, Orientation, Label, Button, Orientation::*, Align::*, GridView, SelectionModel, ListItemFactory, glib};
use relm4::adw::{prelude::*, Window, HeaderBar};

struct App {
    grid_view_model: Option<SelectionModel>,
    grid_view_factory: Option<ListItemFactory>,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    view!{
        Window {
            Box {
                set_orientation: Orientation::Vertical,
                HeaderBar {},
                GridView::new(model.grid_view_model, model.grid_view_factory) {
                    set_min_columns: 2,
                }
            }
        }
    }

    fn init(
            init: Self::Init,
            root: &Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
        let model = App {grid_view_model: Some(glib::Object::builder::<SelectionModel>().build()), grid_view_factory: Some(ListItemFactory)};
        let widgets = view_output!();
        ComponentParts { model: model, widgets: widgets }
    }
}

fn main () {
    let app = RelmApp::new("Grids");
    relm4_icons::initialize_icons();
    app.run::<App>(());
}