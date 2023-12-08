use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub center: Vec2,
}

impl Collider {
    pub fn from_size(size: Vec2) -> Self {
        Self { size, ..default() }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self { size, center }
    }

    pub fn rect(&self, transform: &Transform) -> Rect {
        Rect::from_center_size(transform.translation.truncate() + self.center, self.size)
    }
}

impl Default for Collider {
    fn default() -> Self {
        Self::from_center_size(Vec2::ZERO, Vec2::ZERO)
    }
}
