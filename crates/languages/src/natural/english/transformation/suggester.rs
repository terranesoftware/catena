use catena_suite::traits::transformation::{Edit, Suggester};

pub struct EnglishSuggester;

impl Suggester for EnglishSuggester {
    fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}