use catena_suite::traits::transformation::{Edit, Formatter};

pub struct PythonFormatter;

impl Formatter for PythonFormatter {
    fn formatter(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}