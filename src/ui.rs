use crate::ncurses::{NColor, NPosition, NSize};
use bevy::prelude::{Bundle, Component};

#[derive(Component)]
pub struct Clickable;

#[derive(Component, Default)]
pub struct Label {
    pub text: String,
}

impl Label {
    #[inline]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Into<Label> for String {
    #[inline]
    fn into(self) -> Label {
        Label { text: self }
    }
}

#[derive(Default, Bundle)]
pub struct LabelBundle {
    label: Label,
    position: NPosition,
    color: NColor,
    size: NSize,
}

impl LabelBundle {
    #[inline]
    pub fn new(text: impl Into<String>, position: impl Into<NPosition>) -> Self {
        Self::default().with_text(text).with_position(position)
    }

    #[inline]
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.label = text.into().into();
        self.size = (self.label.text.len() as u16, 1).into();
        self
    }

    #[inline]
    pub fn with_position(mut self, position: impl Into<NPosition>) -> Self {
        self.position = position.into();
        self
    }

    #[inline]
    pub fn with_color(mut self, color: impl Into<NColor>) -> Self {
        self.color = color.into();
        self
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    pub label: LabelBundle,
    clickable: Clickable,
}

impl ButtonBundle {
    #[inline]
    pub fn new(text: impl Into<String>, position: impl Into<NPosition>) -> Self {
        Self {
            label: LabelBundle::new(text, position),
            clickable: Clickable,
        }
    }

    #[inline]
    pub fn new_with(label: LabelBundle) -> Self {
        Self {
            label,
            clickable: Clickable,
        }
    }

    #[inline]
    pub fn with_color(mut self, color: impl Into<NColor>) -> Self {
        self.label.color = color.into();
        self
    }
}

#[derive(Component, Default)]
pub struct Padding(u16);

impl Into<Padding> for u16 {
    fn into(self) -> Padding {
        Padding(self)
    }
}

impl Into<u16> for Padding {
    fn into(self) -> u16 {
        self.0
    }
}

#[derive(Component, Default)]
pub struct Spacing(u16);

impl Into<Spacing> for u16 {
    fn into(self) -> Spacing {
        Spacing(self)
    }
}

impl Into<u16> for Spacing {
    fn into(self) -> u16 {
        self.0
    }
}
