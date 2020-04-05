use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
use specs::{self, Join};
use specs::world::Builder;
use warmy;

use crate::components as c;
use crate::util;
use crate::input;
use crate::resources;
use crate::scenes;
use crate::systems::*;
use crate::world::World;

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

        let player_entity = world.specs_world.create_entity()
            .with(c::Position(util::point2(0.0, 0.0)))
            .with(c::Motion { velocity: util::vec2(1.0, 1.0), acceleration: util::vec2(0.0, 0.0)})
            .build();

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
        specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
            .build()
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        for p in pos.join() {
            let mut params = graphics::DrawParam::default();
            params.src = graphics::Rect { x: 0.0, y: 0.0, h: 1.0, w: (76.0 / 384.0)};
            params.scale = graphics::mint::Vector2 { x: 0.5, y: 0.5 };
            params.dest = p.0.into();
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
        player_motion.velocity += util::vec2(0.0, gameworld.input.get_axis(input::Axis::Vert)* -1.0);
    }
}
