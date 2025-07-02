use {world::*, helpers::*}

pub trait Entity {

    fn update(&mut self, &World);
}

pub trait Visual {

    fn getColorChar(&self) -> ColorChar;
    
    fn getVisualPriority(&self) -> i32;
}