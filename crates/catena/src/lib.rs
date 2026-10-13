pub mod suite {
    pub use catena_suite::{Language, Suite};
    
    pub mod comprehension {
        pub use catena_suite::suite::comprehension::Suite;
        pub use catena_suite::traits::Comprehension;
    
        pub mod inner {
            pub use catena_suite::traits::comprehension::{Description, Diagnostic, Highlight};
        }
    }
    
    pub mod transformation {
        pub use catena_suite::suite::transformation::Suite;
        pub use catena_suite::traits::Transformation;
    
        pub mod inner {
            pub use catena_suite::traits::transformation::{Completion, Formatter, Propagator, Suggestion};
        }
    }
    
    pub mod traversal {
        pub use catena_suite::suite::traversal::Suite;
        pub use catena_suite::traits::Traversal;
    
        pub mod inner {
            pub use catena_suite::traits::traversal::{Grammar, Index};
        }
    }
}