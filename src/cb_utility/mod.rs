// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

/*
    This module is designed to remove dependencies on other libraries, or platform specific code.
*/

/// Initialize a component linker that registers components to the world as well as the component implementations.
#[macro_export]
macro_rules! init_components{
    ($i:ident, ($( $t:ty ),*) ) => {
            pub struct $i {}

            impl ComponentLinker for $i {
                fn register_components(world: &mut World) {
                    $(
                      world.register::<$t>();
                    )*
                }
            }

            $(
            impl Component for $t {
                type Storage = VecStorage<Self>;
            }
            )*
        };
}

/// Initialize multiple variables
#[macro_export]
macro_rules! let_mut_for{
     ( ($( $i:ident ),*), $type:ty, $value:expr ) => {
            $(
                 let mut $i: $type = $value;
            )*
        };
    }

/// Declare multiple constant values, starting from zero and incrementing for each item.
#[macro_export]
macro_rules! pub_const_identities{
     ( ($( $i:ident ),*), $type:ty) => {
            $(
                 pub const $i: $type = 0;
            )*
        };
    }
