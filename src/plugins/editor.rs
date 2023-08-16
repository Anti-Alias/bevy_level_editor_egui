use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Ui;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::*;


/// Plugin that adds an egui-based level editor.
#[derive(Default)]
pub struct EditorPlugin {
    pub config: EditorConfig
}
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_plugins((
            DefaultInspectorConfigPlugin,
            ResourcesPlugin::<AmbientLight>::default(),
            FlycamPlugin
        ));
        app.insert_resource(self.config.clone());
        app.init_resource::<PrefabGroups>();
        app.add_systems(Startup, startup);
        app.add_systems(Startup, setup_builtin_prefabs);
        app.add_systems(Update, render_ui);
    }
}

/// Config for the [`EditorPlugin`]
#[derive(Resource, Clone, Debug)]
pub struct EditorConfig {
    pub spawn_distance: f32
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            spawn_distance: 10.0
        }
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 4.0, 10.0),
            ..default()
        },
        Flycam::default()
    ));   
}

fn setup_builtin_prefabs(mut prefab_groups: ResMut<PrefabGroups>) {

    let mut lights = PrefabGroup::new("Lights");
    lights
        .add(PointLightPrefab)
        .add(DirectionalLightPrefab);
    
    let mut shapes = PrefabGroup::new("Shapes");
    shapes
        .add(PlanePrefab)
        .add(CubePrefab);

    prefab_groups
        .add(lights)
        .add(shapes);
}

fn render_ui(world: &mut World) {

    let mut egui_ctx = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single_mut(world)
        .clone();
    let ctx = egui_ctx.get_mut();

    egui::SidePanel::left("inspector")
        .exact_width(200.0)
        .show(ctx, |ui| inspector(world, ui));

    egui::SidePanel::right("prefabs")
        .exact_width(200.0)
        .show(ctx, |ui| prefabs(world, ui));
}

fn inspector(world: &mut World, ui: &mut Ui) {
    
    ui.heading("Entities");
    bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
    ui.separator();

    ui.heading("Resources");
    let Some(mut enabled_plugins) = world.get_resource_mut::<EnabledPlugins>() else {
        return;
    };
    for (name, enabled) in enabled_plugins.0.iter_mut() {
        if ui.radio(*enabled, *name).clicked() {
            *enabled = !*enabled;
        }
    }
}


fn prefabs(world: &mut World, ui: &mut Ui) {
    
    let prefab_groups: &PrefabGroups = world.get_resource().unwrap();

    // Heading    
    ui.spacing_mut().item_spacing.y = 10.0;
    ui.heading("Prefabs");
    ui.spacing_mut().item_spacing.y = 2.0;

    // Displays all groups. User selects prefab.
    let mut prefab_selected = None;
    for group in prefab_groups.iter() {
        ui.collapsing(group.name(), |ui| {
            for prefab in group.iter_mut() {
                if ui.button(prefab.name()).clicked() {
                    prefab_selected = Some(prefab.clone())
                }
            }
        });
    }

    // Spawn entity if prefab was selected.
    if let Some(prefab_selected) = prefab_selected {
        let spawn_distance = world.resource::<EditorConfig>().spawn_distance;
        let mut camera_query = world.query::<(&Transform, &Flycam)>();
        let (cam_transf, cam_fly) = camera_query.get_single(world).unwrap();
        prefab_selected.spawn(world, cam_transf.translation + cam_fly.direction() * spawn_distance);
    }
}