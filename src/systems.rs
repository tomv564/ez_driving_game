//! specs systems.
use crate::components::*;
// use crate::util;
use specs::{self, Join};
use ncollide2d as nc;
use ggez::nalgebra as na;

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
    type SystemData = (
        specs::WriteStorage<'a, Position>,
        specs::ReadStorage<'a, Motion>,
    );

    fn run(&mut self, (mut pos, motion): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        for (pos, motion) in (&mut pos, &motion).join() {
            pos.point += motion.velocity;
            pos.rotation = motion.orientation;
        }
    }
}

pub struct CollisionSystem;

// impl CollisionSystem {
//      fn collides(pos: Position, positions: specs::ReadStorage<Position>) -> bool {
//         return true;
//     }
// }

impl<'a> specs::System<'a> for CollisionSystem {
    type SystemData = (
        specs::WriteExpect<'a, nc::world::CollisionWorld<f32, specs::Entity>>,
        specs::ReadStorage<'a, Collider>,
        specs::ReadStorage<'a, Position>
    );

    fn run(&mut self, (mut collision_world, colliders, positions): Self::SystemData) {

        for (collider, pos) in (&colliders, &positions).join() {
            let collision_obj = collision_world.get_mut(collider.handle).expect("yo no collision object?");

            let new_position = na::Isometry2::new(na::Vector2::new(pos.point.x, pos.point.y), na::zero());
            // collision_obj.position().clone();
            // new_position.
            collision_obj.set_position(new_position);
        }
    }


}
// pub struct RenderSystem;

// impl<'a> specs::System<'a> for RenderSystem {

//     type SystemData = (
//         specs::ReadStorage<'a, Position>
//     );

//     fn run(&mut self, (pos): Self::SystemData) {
//         for (pos) in (&pos).join() {
//             // TODO: draw
//             let mut params = graphics::DrawParam::default();
//             params.src = graphics::Rect { x: 0.0, y: 0.0, h: 1.0, w: (76.0 / 384.0)};
//             params.scale = graphics::mint::Vector2 { x: 0.5, y: 0.5 };
//             params.dest = p.0.into();
//             graphics::draw(
//                 ctx,
//                 &(self.car.borrow().0),
//                 params,
//             )?;        }
//     }

// }

// pub struct CollisionSystem;

// impl<'a> specs::System<'a> for CollisionSystem {
//     type SystemData = (
//         specs::ReadStorage<Position>
//     );

//     fn fun(&mut self, (mut pos): Self::SystemData) {

//     }
// }
