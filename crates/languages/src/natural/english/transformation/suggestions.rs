use catena_suite::traits::transformation::Suggestions;

pub struct EnglishSuggestions;

impl Suggestions for EnglishSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) {
        
    }
}