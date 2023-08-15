use std::sync::Arc;

use bevy::prelude::*;

/// An object that can spawn an [`Entity`].
pub trait Prefab: Send + Sync + 'static {
    fn name(&self) -> &str;
    fn spawn(&self, world: &mut World);
}

/// A grouping of related prefabs.
pub struct PrefabGroup {
    name: String,
    prefabs: Vec<Arc<dyn Prefab>>
}

impl PrefabGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), prefabs: Vec::new() }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add(&mut self, prefab: impl Prefab) -> &mut Self {
        self.prefabs.push(Arc::new(prefab));
        self
    }
    pub fn iter_mut(&self) -> impl Iterator<Item=&Arc<dyn Prefab>> {
        self.prefabs.iter()
    }
}

/// A listing of all [`PrefabGroup`]s.
#[derive(Resource, Default)]
pub struct PrefabGroups(Vec<PrefabGroup>);
impl PrefabGroups {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, group: PrefabGroup) -> &mut Self {
        self.0.push(group);
        self
    }
    pub fn iter(&self) -> impl Iterator<Item=&PrefabGroup> {
        self.0.iter()
    }
}