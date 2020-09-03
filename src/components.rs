use crate::types::*;
use ggez::graphics;
use ncollide2d as nc;

use specs::*;
use specs_derive::*;

// ///////////////////////////////////////////////////////////////////////
// Components
// ///////////////////////////////////////////////////////////////////////

/// A position in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub point: Point2,
    pub rotation: f32
}

/// A sprite
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Sprite {
	pub clip: graphics::Rect,
	pub scale: graphics::mint::Vector2<f32>
}

/// Motion in the game world.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub is_blocked: bool,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub orientation: f32
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub handle: nc::pipeline::object::CollisionObjectSlabHandle
}

/// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Clone, Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Shot {
    pub damage: u32,
}

pub fn register_components(specs_world: &mut World) {
    specs_world.register::<Position>();
    specs_world.register::<Motion>();
    specs_world.register::<Collider>();
    // specs_world.register::<Shot>();
    specs_world.register::<Player>();
    specs_world.register::<Sprite>();
}
