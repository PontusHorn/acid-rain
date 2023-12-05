use bevy::prelude::*;

#[derive(Component)]
pub struct Collider(pub Rect);

impl Collider {
    pub fn new(rect: Rect) -> Self {
        Self(rect)
    }

    pub fn translate(&self, translation: Vec2) -> Self {
        Self::new(Rect::from_center_size(
            self.0.center() + translation,
            self.0.size(),
        ))
    }

    pub fn contains(&self, point: Vec2) -> bool {
        self.0.contains(point)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !self.0.intersect(other.0).is_empty()
    }
}
