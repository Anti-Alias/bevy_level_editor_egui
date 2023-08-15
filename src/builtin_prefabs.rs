use bevy::prelude::*;

use crate::{Prefab, WorldExt};

/// Simple plane
pub struct PlanePrefab;
impl Prefab for PlanePrefab {

    fn name(&self) -> &str { "Plane" }
    
    fn spawn(&self, world: &mut World) {
        let mesh = world.store_asset::<Mesh>(shape::Plane::from_size(10.0).into());
        let material = world.store_asset::<StandardMaterial>(Color::BLUE.into());
        world.spawn(PbrBundle {
            mesh,
            material,
            ..default()
        }).insert(Name::new("Plane"));
    }
}

// Simple cube
pub struct CubePrefab;
impl Prefab for CubePrefab {

    fn name(&self) -> &str { "Cube" }
    
    fn spawn(&self, world: &mut World) {
        let mesh = world.store_asset::<Mesh>(shape::Cube { size: 1.0 }.into());
        let material = world.store_asset::<StandardMaterial>(Color::YELLOW.into());
        world.spawn(PbrBundle {
            mesh,
            material,
            ..default()
        }).insert(Name::new("Cube"));
    }
}

// Simple point light
pub struct PointLightPrefab;
impl Prefab for PointLightPrefab {

    fn name(&self) -> &str { "Point Light" }

    fn spawn(&self, world: &mut World) {
        world.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 9000.0,
                range: 100.,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default()
        }).insert(Name::new("Point Light"));
    }
}