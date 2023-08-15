use bevy::asset::Asset;
use bevy::prelude::*;

pub trait WorldExt {
    fn store_asset<A: Asset>(&mut self, resource: A) -> Handle<A>;
}

impl WorldExt for World {
    fn store_asset<A: Asset>(&mut self, resource: A) -> Handle<A> {
        self
            .get_resource_mut::<Assets<A>>()
            .expect(&format!("Could not find asset store for type {}", A::type_path()))
            .add(resource)
    }
}