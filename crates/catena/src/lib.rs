pub mod suite {
    pub use catena_suite::{Language, Suite};
    
    pub mod comprehension {
        pub use catena_suite::suite::comprehension::Suite;
        pub use catena_suite::traits::Comprehension;
    
        pub mod inner {
            pub use catena_suite::traits::comprehension::{Descriptor, Diagnoser};

            pub mod descriptor {
                pub use catena_suite::traits::comprehension::descriptor::Description;
            }

            pub mod diagnoser {
                pub use catena_suite::traits::comprehension::diagnoser::Diagnostic;

                pub mod diagnostic {
                    pub use catena_suite::traits::comprehension::diagnoser::diagnostic::Severity;
                }
            }
        }
    }
    
    pub mod transformation {
        pub use catena_suite::suite::transformation::Suite;
        pub use catena_suite::traits::Transformation;
    
        pub mod inner {
            pub use catena_suite::traits::transformation::{Completer, Edit, Formatter, Propagator, Suggester};
        }
    }
    
    pub mod traversal {
        pub use catena_suite::suite::traversal::Suite;
        pub use catena_suite::traits::Traversal;
    
        pub mod inner {
            pub use catena_suite::traits::traversal::{Indexer, Segmenter};
        }
    }
}