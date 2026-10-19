use catena_suite::traits::comprehension::{Descriptor, descriptor::Description};

pub struct PythonDescriptor;

impl Descriptor for PythonDescriptor {
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