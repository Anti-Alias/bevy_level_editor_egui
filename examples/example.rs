use bevy::prelude::*;
use bevy_level_editor_egui::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut prefab_groups: ResMut<PrefabGroups>
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    let mut lights = PrefabGroup::new("Lights");
    lights.add(PointLightPrefab);
    
    let mut shapes = PrefabGroup::new("Shapes");
    shapes.add(PlanePrefab);
    shapes.add(CubePrefab);
    shapes.add(PointLightPrefab);

    prefab_groups
        .add(lights)
        .add(shapes);
}