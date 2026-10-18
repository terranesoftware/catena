use catena_suite::traits::transformation::{Edit, Formatter};

pub struct RustFormatter;

impl Formatter for RustFormatter {
    fn formatter(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}