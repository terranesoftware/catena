use catena_suite::traits::transformation::{Edit, Formatter};

pub struct EnglishFormatter;

impl Formatter for EnglishFormatter {
    fn formatter(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}