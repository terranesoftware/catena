use catena_suite::traits::transformation::{Edit, Suggestions};

pub struct JapaneseSuggestions;

impl Suggestions for JapaneseSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}