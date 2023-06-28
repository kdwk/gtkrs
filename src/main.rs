#![allow(unused_imports)]
#![allow(unused_variables)]
use gtkrs::app::App;
use relm4::prelude::*;

fn main () {
    let app = RelmApp::new("com.github.kdwk.Compare");
    app.run::<App>(());
}