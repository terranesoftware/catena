use catena_suite::traits::comprehension::{Descriptor, descriptor::Description};

pub struct EnglishDescriptor;

impl Descriptor for EnglishDescriptor {
    fn describe(
        &self,
        text: &[u8]
    ) -> Description {
        Description { subjects: Vec::new(), text: String::new() }
    }
}