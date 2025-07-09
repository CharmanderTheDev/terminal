mod components; use components::*;
mod systems; use systems::*;
mod actions; use actions::*;

use anymap::AnyMap;

use rustc_hash::FxHashMap;

struct Archetype {
    entities: Vec<u64>

}

struct World {

    id_on: u64,
    components: AnyMap,

} impl World {
    fn insert_component<T: 'static>(&mut self, id: u64, component: Box<dyn Any>) {
        
    }

    fn insert_entity(mut self, entity: Vec<Box<dyn Component>>) {
        self.id_on += 1;

        for component in entity {
            component.add(self.id_on, &mut self);
        }
    }
}