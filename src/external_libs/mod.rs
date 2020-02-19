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
