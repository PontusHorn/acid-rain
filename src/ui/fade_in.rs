use bevy::prelude::*;

pub struct FadeInPlugin;

impl Plugin for FadeInPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fade_in_background, fade_in_text));
    }
}

#[derive(Component)]
pub struct FadeIn {
    timer: Timer,
    text_alpha: f32,
    background_alpha: f32,
}

impl Default for FadeIn {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1., TimerMode::Once),
            text_alpha: 1.,
            background_alpha: 1.,
        }
    }
}

impl FadeIn {
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
            ..default()
        }
    }

    pub fn with_background_alpha(mut self, alpha: f32) -> Self {
        self.background_alpha = alpha;
        self
    }
}

fn fade_in_background(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &mut FadeIn, Entity)>,
) {
    for (mut color, mut fade, entity) in query.iter_mut() {
        fade.timer.tick(time.delta());
        let alpha = fade.timer.percent() * fade.background_alpha;
        color.0.set_a(alpha);
        if fade.timer.finished() {
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_in_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut FadeIn, Entity)>,
) {
    for (mut text, mut fade_in, entity) in query.iter_mut() {
        fade_in.timer.tick(time.delta());
        let alpha = fade_in.timer.percent() * fade_in.text_alpha;
        for section in text.sections.iter_mut() {
            section.style.color.set_a(alpha);
        }

        if fade_in.timer.finished() {
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}
