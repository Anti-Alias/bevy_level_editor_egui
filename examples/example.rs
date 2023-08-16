use bevy::prelude::*;
use bevy_level_editor_egui::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin::default()
        ))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut prefab_groups: ResMut<PrefabGroups>) {

    let mut lights = PrefabGroup::new("Lights");
    lights.add(PointLightPrefab);
    
    let mut shapes = PrefabGroup::new("Shapes");
    shapes
        .add(PlanePrefab)
        .add(CubePrefab)
        .add(PointLightPrefab);

    prefab_groups
        .add(lights)
        .add(shapes);
}