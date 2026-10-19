use catena_suite::traits::comprehension::{Descriptor, descriptor::Description};

pub struct RustDescriptor;

impl Descriptor for RustDescriptor {
    fn describe(
        &self,
        text: &[u8]
    ) -> Description {
        Description {
            subjects: Vec::new(),
            text: String::new()
        }
    }
}