mod heal; use heal::*;
mod moveby; use moveby::*;


macro_rules! struct_enum {
    ($($name:ident),*) => {
        enum EntityAction {
            $($name($name),)*
        }
    }
}

struct_enum!{
    Heal,
    MoveBy
}
