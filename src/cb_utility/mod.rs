// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

/*
    This module is designed to remove dependencies on other libraries, or platform specific code.
*/

// IF SPECS:

/// Initialize a Specs implementation of Component for the given structs using the VecStorage type.
#[macro_export]
macro_rules! init_component_implementations{
     ( $( $t:ty ),* ) => {
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
macro_rules! const_identities{
     ( ($( $i:ident ),*), $type:ty) => {
            $(
                 const $i: $type = 0;
            )*
        };
    }
