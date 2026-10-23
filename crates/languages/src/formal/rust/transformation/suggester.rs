use catena_suite::traits::transformation::{Edit, Suggester};

pub struct RustSuggester;

impl Suggester for RustSuggester {
    fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}