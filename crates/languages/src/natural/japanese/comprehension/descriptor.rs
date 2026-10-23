use catena_suite::traits::comprehension::{Descriptor, descriptor::Description};

pub struct JapaneseDescriptor;

impl Descriptor for JapaneseDescriptor {
    fn describe(
        &self,
        text: &[u8]
    ) -> Description {
        Description { subjects: Vec::new(), text: String::new() }
    }
}