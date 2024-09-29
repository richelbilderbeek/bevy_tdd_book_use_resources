use bevy::prelude::*;

use bevy::prelude::States;

#[derive(Resource)]
pub struct GoalsReached {
    bonus: u32,
}

// custom implementation for unusual values
impl Default for GoalsReached {
    fn default() -> Self {
        GoalsReached {
            bonus: 0
        }
    }
}

pub fn create_default_app() -> App {
    create_app(0)
}

pub fn create_app(initial_bonus: u32) -> App {
    let mut app = App::new();

    // Only add these plugin in testing.
    // The main app will assume it to be absent.
    // Adding DefaultPlugins will cause tests to crash
    if cfg!(test) {
        //app.add_plugins(bevy::state::app::StatesPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.init_resource::<GoalsReached>();


    // NO! Do not update!
    // text will be invisible in main
    //app.update();

    app
}


#[cfg(test)]
fn get_bonus(app: &mut App) -> u32 {
    app.world().resource::<GoalsReached>().get().clone().bonus
}


#[cfg(test)]
mod tests {
    use bevy::input::keyboard::Key;
    use super::*;

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_app_has_text() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_texts(&mut app), 1);
    }

    #[test]
    fn test_app_starts_with_no_bonus() {
        let mut app = create_default_app();
        app.update();
        assert_eq!(get_bonus(&mut app), 0);
    }
    #[test]
    fn test_app_can_have_an_initial_bonus() {
        let bonus = 42;
        let mut app = create_app(bonus);
        app.update();
        assert_eq!(get_bonus(&mut app), bonus);
    }
}
