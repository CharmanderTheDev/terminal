mod entityactions;
mod worldactions;

enum Action {
    WorldAction(WorldAction),
    EntityAction(EntityAction)
}

enum WorldAction {
    RemoveEntity(usize),
    AddEntity(usize, Entity),
    Draw(Position, ColorChar),

}

