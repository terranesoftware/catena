use catena_suite::traits::transformation::{Edit, Suggester};

pub struct JapaneseSuggester;

impl Suggester for JapaneseSuggester {
    fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}