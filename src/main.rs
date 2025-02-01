use bevy::app::Startup;
use bevy::prelude::{App, Commands, Trigger};
use n_tetris::ncurses::{ClickEvent, Color, LabelBundle, NColor, NcursesPlugin};

fn main() {
    App::new()
        .add_plugins(NcursesPlugin)
        .add_systems(Startup, create_label)
        .run();
}

fn create_label(mut commands: Commands) {
    LabelBundle::new("Hello world", (10, 10)).spawn(&mut commands);
    LabelBundle::new("Hello world clickable", (10, 11))
        .spawn_as_button(&mut commands)
        .observe(|_trigger: Trigger<ClickEvent>| panic!("clicked"));
    LabelBundle::new("Hello world colored", (10, 12))
        .with_color(NColor::new(Color::Red, Color::White))
        .spawn(&mut commands);
}
