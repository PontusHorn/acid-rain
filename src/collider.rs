use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub center: Vec2,
    pub solid: bool,
}

impl Collider {
    pub fn from_size(size: Vec2) -> Self {
        Self { size, ..default() }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            size,
            center,
            ..default()
        }
    }

    pub fn rect(&self, translation: &Vec3) -> Rect {
        Rect::from_center_size(translation.truncate() + self.center, self.size)
    }

    pub fn with_solid(mut self, solid: bool) -> Self {
        self.solid = solid;
        self
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            size: Vec2::ZERO,
            center: Vec2::ZERO,
            solid: true,
        }
    }
}
