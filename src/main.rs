use bevy::app::Startup;
use bevy::prelude::{App, Commands, Trigger};
use n_tetris::ncurses::{ButtonBundle, ClickEvent, Color, LabelBundle, NcursesPlugin};

fn main() {
    App::new()
        .add_plugins(NcursesPlugin)
        .add_systems(Startup, create_label)
        .run();
}

fn create_label(mut commands: Commands) {
    commands.spawn(LabelBundle::new("Hello world", (10, 10)));
    commands
        .spawn(ButtonBundle::new("Hello world clickable", (10, 11)))
        .observe(|_trigger: Trigger<ClickEvent>| panic!("clicked"));
    commands.spawn(
        LabelBundle::new("Hello world colored", (10, 12)).with_color((Color::Red, Color::White)),
    );
}
