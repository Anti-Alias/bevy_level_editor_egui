use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::Ui;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::PrefabGroups;
use crate::{ResourcesPlugin, EnabledPlugins};


/// Plugin that adds an egui-based level editor.
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_plugins((
            DefaultInspectorConfigPlugin,
            ResourcesPlugin::<AmbientLight>::default(),
        ));
        app.init_resource::<PrefabGroups>();
        app.add_systems(Startup, startup);
        app.add_systems(Update, render_ui);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
    
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
    let prefab_groups: &PrefabGroups = world.get_resource().expect("PrefabGroups resource not found");

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
        prefab_selected.spawn(world);
    }
}