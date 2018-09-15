extern crate pallium;
extern crate yew;

use pallium::*;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Context, Model>::new(Context::new()).mount_to_body();
    yew::run_loop();
}
