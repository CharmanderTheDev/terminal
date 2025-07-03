use {entities::*, helpers::*}

struct Spot {

    entities: Vec<Entity>

} impl Spot {

    fn udpate(&mut self, &World) {
        for entitiy in self.entities {
            entitiy.update(&World);
        }
    }

    fn getColorChar(&self) -> ColorChar {
        max_priority = i32::MIN;
        priority_colorchar = &ColorChar::default();

        for entity in self.entities {
            if entity.getVisualPriority() > max_priority {
                max_priority = entity.getVisualPriority;
                priority_colorchar = &entity.getColorChar;
            }
        }

        priority_colorchar.clone()
    }
}

struct World {

    spots: Vec<Vec<Spot>>

} impl World {

    fn getInfo(&self, pos: Position) -> Tile {
        return &spots.get(pos.getX()).get(pos.getY())
    }

    fn update(&mut self) {
        for row in self.spots { for spot in row {
            spot.update(&self);
        }}
    }

    fn getVisuals(&self)
}