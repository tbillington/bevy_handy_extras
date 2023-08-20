use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;

pub trait MoveTowards<T> {
    fn move_towards(current: T, target: T, max_move: f32) -> T;
    fn move_towards_unclamped(current: T, target: T) -> T;
}

impl MoveTowards<Self> for Vec3 {
    #[inline]
    fn move_towards(current: Self, target: Self, max_move: f32) -> Self {
        current + (target - current).clamp_length_max(max_move)
    }

    #[inline]
    fn move_towards_unclamped(current: Self, target: Self) -> Self {
        current + (target - current)
    }
}

impl MoveTowards<Self> for Vec2 {
    #[inline]
    fn move_towards(current: Self, target: Self, max_move: f32) -> Self {
        current + (target - current).clamp_length_max(max_move)
    }

    #[inline]
    fn move_towards_unclamped(current: Self, target: Self) -> Self {
        current + (target - current)
    }
}

pub trait MoveTowardsSelf<T> {
    fn move_towards(&mut self, target: T, max_move: f32);
    fn move_towards_unclamped(&mut self, target: T);
}

impl<'a> MoveTowardsSelf<Vec3> for Mut<'a, Transform> {
    #[inline]
    fn move_towards(&mut self, target: Vec3, max_move: f32) {
        self.translation = Vec3::move_towards(self.translation, target, max_move);
    }

    #[inline]
    fn move_towards_unclamped(&mut self, target: Vec3) {
        self.translation = Vec3::move_towards_unclamped(self.translation, target);
    }
}
