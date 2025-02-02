use crate::ncurses::{NColor, NLocalPosition, NPosition, NSize};
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
    local_position: NLocalPosition,
    color: NColor,
    size: NSize,
}

impl LabelBundle {
    #[inline]
    pub fn new(text: impl Into<String>, position: impl Into<NPosition>) -> Self {
        Self::default().with_text(text).with_position(position)
    }

    pub fn new_text(text: impl Into<String>) -> Self {
        Self::default().with_text(text)
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

    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            label: LabelBundle::new_text(text),
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
pub struct Padding(pub u16);

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
pub struct Spacing(pub u16);

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

#[derive(Component, Default)]
pub struct VBox;

#[derive(Bundle, Default)]
pub struct VBoxBundle {
    vbox: VBox,
    pub size: NSize,
    pub spacing: Spacing,
    pub padding: Padding,
    pub position: NPosition,
    pub local_position: NLocalPosition,
}

impl VBoxBundle {
    pub fn new(position: impl Into<NPosition>) -> Self {
        Self::default().with_position(position)
    }

    pub fn with_position(mut self, position: impl Into<NPosition>) -> Self {
        self.position = position.into();
        self
    }

    pub fn with_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn with_spacing(mut self, spacing: impl Into<Spacing>) -> Self {
        self.spacing = spacing.into();
        self
    }

    pub fn with_local_position(mut self, position: impl Into<NLocalPosition>) -> Self {
        self.local_position = position.into();
        self
    }
}
