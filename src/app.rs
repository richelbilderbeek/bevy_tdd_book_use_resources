use bevy::prelude::*;


#[derive(Resource)]
pub struct MyResource {
    my_value: i32,
}

pub fn create_default_app() -> App {
    create_app(MyResource { my_value:0 } )
}

pub fn create_app(initial_my_resource: MyResource) -> App {
    let mut app = App::new();

    app.insert_resource(initial_my_resource);

    // NO! Do not update!
    //app.update();
    app
}


pub fn get_my_resource(app: &mut App) -> &MyResource {
    app.world().resource::<MyResource>()
}

pub fn get_my_value(app: &mut App) -> i32 {
    get_my_resource(app).my_value
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_default_app();
    }


    #[test]
    fn test_app_starts_with_default_value() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(get_my_value(&mut app), 0);
    }
    #[test]
    fn test_app_can_have_an_initial_value() {
        let value = 314;
        let my_resource = MyResource { my_value: value};
        let mut app = create_app(my_resource);
        app.update();
        assert_eq!(get_my_resource(&mut app).my_value, value);
        assert_eq!(get_my_value(&mut app), value);
    }
}
