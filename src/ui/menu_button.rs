use bevy::prelude::*;

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_button);
    }
}

#[derive(Component)]
pub struct MenuButton;

#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub button: ButtonBundle,
    pub colors: MenuButtonColors,
    menu_button: MenuButton,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        let button_colors = MenuButtonColors::default();
        MenuButtonBundle {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(170.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: button_colors.normal.into(),
                ..default()
            },
            colors: button_colors,
            menu_button: MenuButton,
        }
    }
}

impl MenuButtonBundle {
    pub fn transparent() -> Self {
        let mut bundle = Self::default();
        bundle.button.background_color = Color::NONE.into();
        bundle.colors.normal = Color::NONE;
        bundle
    }

    pub fn with_width(mut self, width: Val) -> Self {
        self.button.style.width = width;
        self
    }
}

#[derive(Component)]
pub struct MenuButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

impl Default for MenuButtonColors {
    fn default() -> Self {
        MenuButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            pressed: Color::rgb(0.35, 0.35, 0.35),
        }
    }
}

#[derive(Bundle)]
pub struct MenuButtonLabelBundle {
    text: TextBundle,
}

impl Default for MenuButtonLabelBundle {
    fn default() -> Self {
        MenuButtonLabelBundle {
            text: TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 40.0,
                    color: Self::COLOR,
                    ..default()
                },
            ),
        }
    }
}

impl MenuButtonLabelBundle {
    pub const COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

    pub fn from_text(text: &str) -> Self {
        let mut bundle = Self::default();
        bundle.text.text.sections[0].value = text.to_string();
        bundle
    }

    pub fn with_small_font(mut self) -> Self {
        for section in &mut self.text.text.sections {
            section.style.font_size = 15.0;
        }
        self
    }

    pub fn with_alpha(mut self, alpha: f32) -> Self {
        for section in &mut self.text.text.sections {
            section.style.color.set_a(alpha);
        }
        self
    }
}

fn hover_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonColors),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::Pressed => {
                *color = button_colors.pressed.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}
