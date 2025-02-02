use bevy::app::Startup;
use bevy::prelude::{App, BuildChildren, ChildBuild, Commands, Trigger};
use n_tetris::ncurses::{ClickEvent, Color, NcursesPlugin};
use n_tetris::ui::{ButtonBundle, LabelBundle, VBoxBundle};

fn main() {
    App::new()
        .add_plugins(NcursesPlugin)
        .add_systems(Startup, create_label)
        .run();
}

fn create_label(mut commands: Commands) {
    commands
        .spawn(VBoxBundle::new((2, 1)))
        .with_children(|parent| {
            parent
                .spawn(VBoxBundle::default().with_local_position((10, 10)))
                .with_children(|parent| {
                    parent.spawn(LabelBundle::new_text("Hello world"));
                    parent
                        .spawn(ButtonBundle::new_text("Hello world clickable"))
                        .observe(|_trigger: Trigger<ClickEvent>| panic!("clicked"));
                    parent.spawn(
                        LabelBundle::new_text("Hello world colored")
                            .with_color((Color::Red, Color::White)),
                    );
                });
        });
}
