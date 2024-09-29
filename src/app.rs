use bevy::prelude::*;


#[derive(Resource)]
pub struct MyResource {
    my_value: i32,
}

pub fn create_default_app() -> App {
    create_app(0)
}

pub fn create_app(initial_my_value: i32) -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        //app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }

    app.insert_resource(MyResource { my_value: initial_my_value} );


    // NO! Do not update!
    // text will be invisible in main
    //app.update();

    app
}


#[cfg(test)]
fn get_my_value(app: &mut App) -> i32 {
    app.world().resource::<MyResource>().my_value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_default_app();
    }


    #[test]
    fn test_app_starts_with_no_my_value() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(get_my_value(&mut app), 0);
    }
    #[test]
    fn test_app_can_have_an_initial_my_value() {
        let my_value = 314;
        let mut app = create_app(my_value);
        app.update();
        assert_eq!(get_my_value(&mut app), my_value);
    }
}
