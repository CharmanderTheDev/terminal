use super::World;

macro_rules! export_components {
    ($($modname:ident :: $typename:ident),*) => {
        $(pub mod $modname;)*
        $(pub use $modname::$typename;)*
    }
}

export_components! {
    position::Position

}

pub trait Component {
    fn add(self, id: u64, _: &mut World);
}

/*NOTE: This macro is only for use with regards to Components under this module.

#[proc_macro_derive(Component)]
pub fn component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    if let Data::Struct(s) = input.data {
        return quote! {
            use super::super::World;
            impl super::Component for #name {
                pub fn add(self, id: u64, world: &mut World) {
                    world.insert::<#name>(self, id);
                }
            }
        }.into()
    } else {
        panic!("Component can only be derived for Structs")
    }
} */