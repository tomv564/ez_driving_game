use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
use specs::{self, Join};
use specs::world::Builder;
use warmy;

use ggez::nalgebra as na;
use ncollide2d as nc;

use crate::components as c;
use crate::util;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::systems::*;
use crate::world::World;

const MIN_VELOCITY: f32 = 0.5;
const MAX_VELOCITY: f32 = -1.0;

pub struct LevelScene {
    done: bool,
    car: warmy::Res<resources::Image>,
    player_entity: specs::Entity,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let car = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/cars-spritesheet.png"), ctx)
            .unwrap();

        let half_height = 76.0 / 2.0;
        let half_width = 76.0 / 4.0;

        // shared collision properties
        let car_shape = nc::shape::Cuboid::new(na::Vector2::new(half_width, half_height));
        let car_collision_group = nc::pipeline::object::CollisionGroups::new();
        let contacts_query = nc::pipeline::object::GeometricQueryType::Contacts(0.0, 0.0);

        // player 1
        let player_entity = world.specs_world.create_entity()
            .with(c::Position { point: util::point2(100.0, 300.0), rotation: 0.0 })
            .with(c::Motion { velocity: util::vec2(0.0, 0.0), acceleration: util::vec2(0.0, 0.0), is_blocked: false, orientation: 0.0})
            .with(c::Sprite { clip: graphics::Rect { x: 0.0, y: 0.0, h: 1.0, w: (76.0 / 384.0)}, scale: graphics::mint::Vector2 { x: 0.5f32, y: 0.5f32 }})
            .build();

        // other car
        let car_entity = world.specs_world.create_entity()
            .with(c::Position { point: util::point2(100.0, 100.0), rotation: 0.0 })
            .with(c::Sprite { clip: graphics::Rect { x: (76.0 / 384.0), y: 0.0, h: 1.0, w: (76.0 / 384.0)}, scale: graphics::mint::Vector2 { x: 0.5f32, y: 0.5f32 }})
            .build();

        // collisions
        //
        {
            let mut collide_world = world.specs_world.write_resource::<nc::world::CollisionWorld<f32, specs::Entity>>();

            let (player_collider_handle, _) = collide_world.add(
                na::Isometry2::new(na::Vector2::new(100.0, 300.0), na::zero()),
                nc::shape::ShapeHandle::new(car_shape.clone()),
                car_collision_group,
                contacts_query,
                player_entity,
            );
            world.specs_world.write_storage::<c::Collider>().insert(player_entity, c::Collider { handle: player_collider_handle }).expect("couldn't insert Collider");

            let (car_collider_handle, _) = collide_world.add(
                na::Isometry2::new(na::Vector2::new(100.0, 100.0), na::zero()),
                nc::shape::ShapeHandle::new(car_shape.clone()),
                car_collision_group,
                contacts_query,
                car_entity,
            );
            world.specs_world.write_storage::<c::Collider>().insert(car_entity, c::Collider { handle: car_collider_handle }).expect("couldn't insert Collider");
        }


        let mut dispatcher = Self::register_systems();
        dispatcher.setup(&mut world.specs_world.res);

        LevelScene {
            done,
            car,
            player_entity,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        let builder = specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
            .with(CollisionSystem, "sys_collision", &[]);
        // builder.add_thread_local(RenderSystem);
        builder.build()
    }


    fn update_collisions(&mut self, world: &mut World) {
        let mut collide_world = world.specs_world.write_resource::<nc::world::CollisionWorld<f32, specs::Entity>>();
        collide_world.update();
        let mut motions = world.specs_world.write_storage::<c::Motion>();


        // gameworld.collide_world.update();
        for e in collide_world.contact_events() {

            match e {
                ncollide2d::pipeline::narrow_phase::ContactEvent::Started(handle1, handle2) =>
                    {
                        println!("contact started!");

                        // look up collision object
                        let obj1 = collide_world.collision_object(*handle1).expect("missing coll obj1");
                        // look up entity
                        let entity1: &specs::Entity = obj1.data();
                        if let Some(motion) = motions.get_mut(*entity1) {
                            motion.is_blocked = true;
                            motion.velocity.x = 0.0;
                            motion.velocity.y = motion.velocity.y * -1.0;
                        }

                        let obj2 = collide_world.collision_object(*handle2).expect("missin coll obj2");
                        let entity2: &specs::Entity = obj2.data();
                        if let Some(motion) = motions.get_mut(*entity2) {
                            motion.is_blocked = true;
                            motion.velocity.x = 0.0;
                            motion.velocity.y = motion.velocity.y * -1.0;
                        }
                    }
                ncollide2d::pipeline::narrow_phase::ContactEvent::Stopped(handle1, handle2) =>
                {
                    println!("contact ended");
                    let obj1 = collide_world.collision_object(*handle1).expect("missing coll obj1");
                    // look up entity
                    let entity1: &specs::Entity = obj1.data();
                    if let Some(motion) = motions.get_mut(*entity1) {
                        motion.is_blocked = false;
                    }

                    let obj2 = collide_world.collision_object(*handle2).expect("missin coll obj2");
                    let entity2: &specs::Entity = obj2.data();
                    if let Some(motion) = motions.get_mut(*entity2) {
                        motion.is_blocked = false;
                    }

                }
            }
        }
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);

        self.update_collisions(gameworld);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }


    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        let sprite = gameworld.specs_world.read_storage::<c::Sprite>();
        let offset_x: f32 = 0.5;
        let offset_y: f32 = 0.5;
        for (p, s) in (&pos, &sprite).join() {
            let mut params = graphics::DrawParam::default();
            params.src = s.clip;
            params.rotation = p.rotation;
            params.scale = s.scale;
            params.offset = na::Point2::new(offset_x, offset_y).into();
            params.dest = p.point.into();
            graphics::draw(
                ctx,
                &(self.car.borrow().0),
                params,
            )?;
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
        let mut motions = gameworld.specs_world.write_storage::<c::Motion>();
        let player_motion = motions.get_mut(self.player_entity).expect("Player w/o motion?");
        if !player_motion.is_blocked {
            let accel = gameworld.input.get_axis(input::Axis::Vert) * -1.0;
            let turn = gameworld.input.get_axis(input::Axis::Horz);
            let y_vel = player_motion.velocity.y + accel;
            let orientation = player_motion.orientation + turn;
            player_motion.orientation = orientation.min(std::f32::consts::PI).max(std::f32::consts::PI * -1.0);
            // let x_vel =

            // backwards because only forward!

            player_motion.velocity = util::vec2(0.0, y_vel.min(MIN_VELOCITY).max(MAX_VELOCITY));
        }
    }
}
