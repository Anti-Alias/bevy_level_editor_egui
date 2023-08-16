use std::f32::consts::FRAC_2_PI;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

use crate::{Prefab, WorldExt};

/// Simple plane
pub struct PlanePrefab;
impl Prefab for PlanePrefab {
    fn name(&self) -> &str { "Plane" }
    fn spawn(&self, world: &mut World, location: Vec3) {
        let mut material: StandardMaterial = Color::BLUE.into();
        material.cull_mode = None;
        let material = world.store_asset::<StandardMaterial>(material);
        let mesh = world.store_asset::<Mesh>(shape::Plane::from_size(10.0).into());
        world
            .spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(location),
                ..default()
            })
            .insert(Name::new("Plane"));
    }
}

// Simple cube
pub struct CubePrefab;
impl Prefab for CubePrefab {
    fn name(&self) -> &str { "Cube" }
    fn spawn(&self, world: &mut World, location: Vec3) {
        let mesh = world.store_asset::<Mesh>(shape::Cube { size: 1.0 }.into());
        let material = world.store_asset::<StandardMaterial>(Color::YELLOW.into());
        world.
            spawn(PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(location),
                ..default()
            })
            .insert(Name::new("Cube"));
    }
}

// Simple point light
pub struct PointLightPrefab;
impl Prefab for PointLightPrefab {
    fn name(&self) -> &str { "Point Light" }
    fn spawn(&self, world: &mut World, location: Vec3) {

        let mesh: Mesh = shape::Icosphere { radius: 0.1, subdivisions: 1 }.try_into().unwrap();
        let mesh = world.store_asset::<Mesh>(mesh);
        let mut material: StandardMaterial = Color::WHITE.into();
        material.unlit = true;
        let material = world.store_asset::<StandardMaterial>(material);

        world
            .spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 9000.0,
                    range: 100.,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_translation(location),
                ..default()
            })
            .insert(Name::new("Point Light"))
            .with_children(|p| {
                p.spawn((
                    PbrBundle {
                        material,
                        mesh,
                        ..default()
                    },
                    NotShadowCaster
                ));
            });
    }
}



// Simple directional light
pub struct DirectionalLightPrefab;
impl Prefab for DirectionalLightPrefab {

    fn name(&self) -> &str { "Directional Light" }

    fn spawn(&self, world: &mut World, location: Vec3) {
        let mesh: Mesh = shape::Cylinder {
            radius: 0.1,
            height: 0.2,
            resolution: 5,
            segments: 1,
        }.try_into().unwrap();
        let mesh = world.store_asset::<Mesh>(mesh);
        let mut material: StandardMaterial = Color::WHITE.into();
        material.unlit = true;
        let material = world.store_asset::<StandardMaterial>(material);

        world
            .spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::IDENTITY
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, -FRAC_2_PI, 0.0, 0.0))
                    .with_translation(location),
                ..default()
            })
            .insert(Name::new("Directional Light"))
            .with_children(|p| {
                p.spawn((
                    PbrBundle {
                        material,
                        mesh,
                        ..default()
                    },
                    NotShadowCaster
                ));
            });
    }
}