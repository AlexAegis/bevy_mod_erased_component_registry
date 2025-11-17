use std::any::{Any, TypeId};

use bevy_ecs::{
	resource::Resource,
	world::{FromWorld, World},
};
use bevy_platform::collections::HashMap;

#[derive(Resource, Default)]
pub struct ErasedComponentRegistry {
	registry: HashMap<TypeId, fn(&mut World) -> Box<dyn Any + Send + Sync>>,
}

impl ErasedComponentRegistry {
	pub(crate) fn register<T: FromWorld + Send + Sync + 'static>(&mut self) {
		self.registry
			.insert(TypeId::of::<T>(), |world: &mut World| {
				Box::new(T::from_world(world))
			});
	}

	pub(crate) fn get_constructor(
		&self,
		type_id: TypeId,
	) -> Option<&fn(&mut World) -> Box<dyn Any + Send + Sync>> {
		self.registry.get(&type_id)
	}

	pub fn is_registered(&self, type_id: &TypeId) -> bool {
		self.registry.contains_key(type_id)
	}
}

#[cfg(test)]
mod test {
	use std::any::TypeId;

	use bevy_app::{App, Startup};
	use bevy_ecs::{component::Component, entity::Entity, resource::Resource, system::Commands};

	use crate::{
		AppRegisterErasedComponentExtension, EntityCommandInsertErasedComponentByTypeIdExtension,
	};

	#[derive(Component, Default, Debug)]
	struct A;

	#[derive(Component, Default, Debug)]
	struct B;

	#[derive(Resource, Clone)]
	struct SpawnedEntities {
		a: Entity,
		b: Entity,
	}

	#[test]
	fn it_should_insert_the_correct_components() {
		let mut app = App::new();

		app.register_erased_component::<A>()
			.register_erased_component::<B>()
			.add_systems(Startup, |mut commands: Commands| {
				let a = commands
					.spawn_empty()
					.insert_component_by_type_id(TypeId::of::<A>())
					.id();
				let b = commands
					.spawn_empty()
					.insert_component_by_type_id(TypeId::of::<B>())
					.id();

				commands.insert_resource(SpawnedEntities { a, b });
			});

		let a_component_id = app.world().components().get_id(TypeId::of::<A>()).unwrap();
		let b_component_id = app.world().components().get_id(TypeId::of::<B>()).unwrap();

		app.update();

		let spawned_entities = app.world().resource::<SpawnedEntities>().clone();

		{
			let entity_info_a = app
				.world_mut()
				.inspect_entity(spawned_entities.a)
				.unwrap()
				.collect::<Vec<_>>();
			assert!(
				entity_info_a.iter().any(|c| c.id() == a_component_id),
				"Component A was not inserted!"
			);
			assert!(
				!entity_info_a.iter().any(|c| c.id() == b_component_id),
				"Component B was inserted on the wrong entity!"
			);
		}
		{
			let entity_info_b = app
				.world_mut()
				.inspect_entity(spawned_entities.b)
				.unwrap()
				.collect::<Vec<_>>();

			assert!(
				!entity_info_b.iter().any(|c| c.id() == a_component_id),
				"Component A was inserted on the wrong entity!"
			);
			assert!(
				entity_info_b.iter().any(|c| c.id() == b_component_id),
				"Component B was not inserted!"
			);
		}
	}
}
