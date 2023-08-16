use bevy::prelude::*;
use bevy_level_editor_egui::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin::default()
        ))
        .run();
}