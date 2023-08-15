use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::utils::HashMap;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;



pub struct ResourcesPlugin<R> {
    phantom: PhantomData<R>
}
impl<R> Default for ResourcesPlugin<R> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::<R>
        }
    }
}
impl<R: Resource + Reflect + TypePath> Plugin for ResourcesPlugin<R> {
    fn build(&self, app: &mut App) {

        // Adds type id to "EnabledPlugins"
        app.init_resource::<EnabledPlugins>();
        let mut enabled_plugins = app.world.resource_mut::<EnabledPlugins>();
        enabled_plugins.0.insert(R::short_type_path(), false);

        // Runs plugin. Only runs if enabled.
        app.add_plugins(ResourceInspectorPlugin::<R>::default().run_if(run_condition::<R>));
    }
}

#[derive(Resource, Debug, Default)]
pub struct EnabledPlugins(pub(crate) HashMap<&'static str, bool>);

fn run_condition<R: TypePath>(enabled_plugins: Res<EnabledPlugins>) -> bool {
    *enabled_plugins.0.get(R::short_type_path()).unwrap()
}