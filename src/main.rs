use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let mut app = create_default_app();
    app.add_plugins(DefaultPlugins);

    // To silence clippy for not reading the value
    assert_eq!(get_my_value(&mut app), 0);

    app.run();
}
