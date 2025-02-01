use bevy::app::App;
use bevy::prelude::{
    AppExit, Bundle, Changed, Commands, Component, Entity, EntityCommands, Event, EventReader,
    IntoSystemConfigs, Last, Plugin, PostUpdate, PreUpdate, Query, Res, Resource, Update,
};
use pancurses::{
    chtype, curs_set, endwin, getmouse, has_colors, init_pair, initscr, mousemask, noecho,
    resize_term, start_color, COLOR_PAIR,
};
use std::ops::{Deref, DerefMut};

pub use pancurses::Input;

pub trait TryApply<T> {
    fn try_apply(&self, f: impl FnOnce(&T));
}

impl<T> TryApply<T> for Option<T> {
    fn try_apply(&self, f: impl FnOnce(&T)) {
        match self {
            None => {}
            Some(value) => {
                f(value);
            }
        }
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,
        }
    }
}

impl Into<Color> for u8 {
    fn into(self) -> Color {
        match self {
            0 => Color::Black,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            6 => Color::Cyan,
            7 => Color::White,
            _ => panic!("Color out of range"),
        }
    }
}

#[derive(Resource)]
pub struct Window {
    pub window: pancurses::Window,
}

impl Deref for Window {
    type Target = pancurses::Window;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        endwin();
    }
}

unsafe impl Sync for Window {}
unsafe impl Send for Window {}

#[derive(Event)]
pub struct ClickEvent;

#[derive(Event)]
pub struct InputEvent {
    pub event: Input,
}

#[derive(Component, Default)]
pub struct NPosition {
    pub x: u16,
    pub y: u16,
}

impl Into<NPosition> for (u16, u16) {
    #[inline]
    fn into(self) -> NPosition {
        NPosition {
            x: self.0,
            y: self.1,
        }
    }
}

#[derive(Component)]
pub struct NSize {
    pub x: u16,
    pub y: u16,
}

impl Into<NSize> for (u16, u16) {
    #[inline]
    fn into(self) -> NSize {
        NSize {
            x: self.0,
            y: self.1,
        }
    }
}

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Label {
    pub text: String,
}

#[derive(Bundle)]
pub struct ColoredLabelBundle {
    pub label: Label,
    pub position: NPosition,
    pub size: NSize,
    pub color: NColor,
}

#[derive(Bundle)]
pub struct NormalLabelBundle {
    pub label: Label,
    pub position: NPosition,
    pub size: NSize,
}

#[derive(Default)]
pub struct LabelBundle {
    text: Option<String>,
    position: Option<NPosition>,
    color: Option<NColor>,
}

impl LabelBundle {
    #[inline]
    pub fn new(text: impl Into<String>, position: impl Into<NPosition>) -> Self {
        Self::default()
            .with_text(text.into())
            .with_position(position.into())
    }

    #[inline]
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    #[inline]
    pub fn with_position(mut self, position: NPosition) -> Self {
        self.position = Some(position);
        self
    }

    #[inline]
    pub fn with_color(mut self, color: NColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn spawn<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let text = self.text.unwrap_or_default();
        let len = text.len() as u16;
        match self.color {
            None => commands.spawn(NormalLabelBundle {
                label: Label { text },
                position: self.position.unwrap_or_default(),
                size: (len, 1).into(),
            }),
            Some(color) => commands.spawn(ColoredLabelBundle {
                label: Label { text },
                position: self.position.unwrap_or_default(),
                size: (len, 1).into(),
                color,
            }),
        }
    }

    pub fn spawn_with<'a, T: Bundle>(
        self,
        commands: &'a mut Commands,
        bundle: T,
    ) -> EntityCommands<'a> {
        let text = self.text.unwrap_or_default();
        let len = text.len() as u16;
        match self.color {
            None => commands.spawn((
                NormalLabelBundle {
                    label: Label { text },
                    position: self.position.unwrap_or_default(),
                    size: (len, 1).into(),
                },
                bundle,
            )),
            Some(color) => commands.spawn((
                ColoredLabelBundle {
                    label: Label { text },
                    position: self.position.unwrap_or_default(),
                    size: (len, 1).into(),
                    color,
                },
                bundle,
            )),
        }
    }

    pub fn spawn_as_button<'a>(self, commands: &'a mut Commands) -> EntityCommands<'a> {
        self.spawn_with(commands, Clickable)
    }

    pub fn spawn_as_button_with<'a, T: Bundle>(
        self,
        commands: &'a mut Commands,
        bundle: T,
    ) -> EntityCommands<'a> {
        self.spawn_with(commands, (Clickable, bundle))
    }
}

#[derive(Component, Copy, Clone)]
pub struct NColor {
    pub color: u8,
}

impl NColor {
    #[inline]
    pub fn new(foreground: Color, background: Color) -> Self {
        Self {
            color: color_num(foreground.into(), background.into()),
        }
    }
}

impl Into<(Color, Color)> for NColor {
    fn into(self) -> (Color, Color) {
        let fg = (self.color & 7).into();
        let bg = ((self.color >> 4) & 7).into();
        (fg, bg)
    }
}

#[inline]
fn color_num(foreground: u8, background: u8) -> u8 {
    1 << 7 | (7 & background) << 4 | 7 & foreground
}

fn is_bold(color: u8) -> bool {
    (1 << 3) & color == 1
}

fn register_colors() {
    for b in 0..8 {
        for f in 0..8 {
            init_pair(color_num(f, b) as i16, f as i16, b as i16);
        }
    }
}

pub struct NcursesPlugin;

impl Plugin for NcursesPlugin {
    fn build(&self, app: &mut App) {
        let window = initscr();
        noecho();
        window.nodelay(true);
        window.keypad(true);
        curs_set(0);
        mousemask(pancurses::BUTTON1_CLICKED as pancurses::mmask_t, None);
        if !has_colors() {
            panic!("You're terminal doesn't support colors");
        }
        start_color();
        register_colors();
        window.clear();
        window.refresh();
        app.set_runner(ncurses_runner);
        app.add_event::<ClickEvent>();
        app.add_event::<InputEvent>();
        app.insert_resource(Window { window });
        app.add_systems(
            PreUpdate,
            (
                clear_window,
                input_window,
                click_event_trigger.after(input_window),
            ),
        );
        app.add_systems(Update, update_label_size);
        app.add_systems(Last, refresh_window);
        app.add_systems(PostUpdate, draw_label);
    }
}

fn ncurses_runner(mut app: App) -> AppExit {
    // Finalize plugin building, including running any necessary clean-up.
    // This is normally completed by the default runner.
    app.finish();
    app.cleanup();

    let mut exit = app.should_exit();

    while exit.is_none() {
        app.update();
        exit = app.should_exit();
    }

    // If exit is `None` then it should still be in the loop
    exit.unwrap()
}

fn clear_window(window: Res<Window>) {
    window.clear();
}

fn input_window(window: Res<Window>, mut commands: Commands) {
    if let Some(input) = window.getch() {
        if let Input::KeyResize = input {
            resize_term(0, 0);
        } else {
            commands.send_event(InputEvent { event: input });
        }
    }
}

fn check_bounds(mouse_pos: (i32, i32), pos: &NPosition, size: &NSize) -> bool {
    mouse_pos.0 >= pos.x as i32
        && mouse_pos.0 < (pos.x + size.x) as i32
        && mouse_pos.1 >= pos.y as i32
        && mouse_pos.1 < (pos.y + size.y) as i32
}

fn click_event_trigger(
    query: Query<(Entity, &NPosition, &NSize, &Clickable)>,
    mut events: EventReader<InputEvent>,
    mut commands: Commands,
) {
    'events: for event in events.read() {
        if let Input::KeyMouse = event.event {
            if let Ok(mouse_event) = getmouse() {
                for (entity, pos, size, _) in query.iter() {
                    if check_bounds((mouse_event.x, mouse_event.y), pos, size) {
                        commands.get_entity(entity).unwrap().trigger(ClickEvent);
                        continue 'events;
                    }
                }
            }
        }
    }
}

fn refresh_window(window: Res<Window>) {
    window.refresh();
}

fn update_label_size(mut query: Query<(&Label, &mut NSize), Changed<Label>>) {
    for (label, mut size) in query.iter_mut() {
        size.x = label.text.len() as u16;
    }
}

fn draw_label(query: Query<(&Label, &NPosition, Option<&NColor>)>, window: Res<Window>) {
    for (label, position, maybe_color) in query.iter() {
        maybe_color.try_apply(|c| {
            window.attron(COLOR_PAIR(c.color as chtype));
            if is_bold(c.color) {
                window.attron(pancurses::A_BOLD);
            }
        });
        window.mvprintw(position.y as i32, position.x as i32, &label.text);
        maybe_color.try_apply(|c| {
            window.attroff(COLOR_PAIR(c.color as chtype));
            if is_bold(c.color) {
                window.attroff(pancurses::A_BOLD);
            }
        });
    }
}
