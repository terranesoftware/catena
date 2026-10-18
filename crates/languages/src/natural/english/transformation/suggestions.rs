use catena_suite::traits::transformation::{Edit, Suggestions};

pub struct EnglishSuggestions;

impl Suggestions for EnglishSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}