#![crate_type = "bin"]
#![warn(non_camel_case_types)]

extern crate native;
extern crate rsfml;
extern crate nphysics = "nphysics2df32";
extern crate nalgebra;
extern crate ncollide = "ncollide2df32";
extern crate graphics2d;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use nalgebra::na::{Vec2, Translation};
use nalgebra::na;
use ncollide::volumetric::Volumetric;
use ncollide::geom::{Plane, Cuboid, Compound, CompoundData, Geom};
use nphysics::world::World;
use nphysics::object::RigidBody;
use graphics2d::engine::GraphicsManager;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    GraphicsManager::simulate(cross_2d)
}

pub fn cross_2d(graphics: &mut GraphicsManager) -> World {
    /*
     * World
     */
    let mut world = World::new();
    world.set_gravity(Vec2::new(0.0f32, 9.81));

    /*
     * First plane
     */
    let mut rb = RigidBody::new_static(Plane::new(Vec2::new(-1.0f32, -1.0)), 0.3, 0.6);

    rb.append_translation(&Vec2::new(0.0, 10.0));

    let body = Rc::new(RefCell::new(rb));

    world.add_body(body.clone());
    graphics.add(body);

    /*
     * Second plane
     */
    let mut rb = RigidBody::new_static(Plane::new(Vec2::new(1.0f32, -1.0)), 0.3, 0.6);

    rb.append_translation(&Vec2::new(0.0, 10.0));

    let body = Rc::new(RefCell::new(rb));

    world.add_body(body.clone());
    graphics.add(body);

    /*
     * Cross shaped geometry
     */
    let mut cross_geoms = CompoundData::new();
    cross_geoms.push_geom(na::one(), Cuboid::new(Vec2::new(5.0f32, 0.25)), 1.0);
    cross_geoms.push_geom(na::one(), Cuboid::new(Vec2::new(0.25f32, 5.0)), 1.0);

    let compound = Compound::new(cross_geoms);
    let mass     = compound.mass_properties(&1.0);
    let cross = Arc::new(box compound as Box<Geom + Send + Sync>);

    /*
     * Create the boxes
     */
    let num     = (750.0f32.sqrt()) as uint;
    let rad     = 5.0;
    let shift   = 2.5 * rad;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift * (num as f32) / 2.0;

    for i in range(0u, num) {
        for j in range(0u, num) {
            let x = i as f32 * 2.5 * rad - centerx;
            let y = j as f32 * 2.5 * rad - centery * 2.0 - 250.0;

            let mut rb = RigidBody::new(cross.clone(), Some(mass), 0.3, 0.6);

            rb.append_translation(&Vec2::new(x, y));

            let body = Rc::new(RefCell::new(rb));

            world.add_body(body.clone());
            graphics.add(body);
        }
    }

    /*
     * The end.
     */
    world
}
