use catena_suite::traits::transformation::{Edit, Suggestions};

pub struct RustSuggestions;

impl Suggestions for RustSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}