pub mod core {
    pub use catena_core::{Language, Suite};
    
    pub mod comprehension {
        pub use catena_core::suite::comprehension::Suite;
        pub use catena_core::traits::Comprehension;
    
        pub mod inner {
            pub use catena_core::traits::comprehension::{Description, Diagnostic, Highlight};
        }
    }
    
    pub mod transformation {
        pub use catena_core::suite::transformation::Suite;
        pub use catena_core::traits::Transformation;
    
        pub mod inner {
            pub use catena_core::traits::transformation::{Completion, Formatter, Propagator, Suggestion};
        }
    }
    
    pub mod traversal {
        pub use catena_core::suite::traversal::Suite;
        pub use catena_core::traits::Traversal;
    
        pub mod inner {
            pub use catena_core::traits::traversal::{Grammar, Index};
        }
    }
}