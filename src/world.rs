use crate::{components, input, resources};

use log::*;
use specs::{self};
use warmy;
use ncollide2d as nc;

use std::path;

pub struct World {
    pub resources: resources::Store,
    pub input: input::State,
    pub specs_world: specs::World
    // pub collide_world: nc::world::CollisionWorld<f32, specs::Entity>
}

impl World {
    pub fn new(resource_dir: &path::Path) -> Self {
        // We to bridge the gap between ggez and warmy path
        // handling here; ggez assumes its own absolute paths, warmy
        // assumes system-absolute paths; so, we make warmy look in
        // the specified resource dir (normally
        // $CARGO_MANIFEST_DIR/resources) or the ggez default resource
        // dir.
        //
        // TODO: ...though this doesn't SEEM to quite do reloading right, so
        // work on it more.
        info!("Setting up resource path: {:?}", resource_dir);
        let opt = warmy::StoreOpt::default().set_root(resource_dir);
        let store = warmy::Store::new(opt)
            .expect("Could not create asset store?  Does the directory exist?");

        let mut w = specs::World::new();
        components::register_components(&mut w);

        let cw: nc::world::CollisionWorld<f32, specs::Entity> = nc::world::CollisionWorld::new(0.2);

        w.add_resource(cw);

        let the_world = Self {
            resources: store,
            input: input::State::new(),
            specs_world: w
            // collide_world: cw
        };

        the_world
    }
}
