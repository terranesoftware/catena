use catena_suite::traits::transformation::{Edit, Formatter};

pub struct JapaneseFormatter;

impl Formatter for JapaneseFormatter {
    fn formatter(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}