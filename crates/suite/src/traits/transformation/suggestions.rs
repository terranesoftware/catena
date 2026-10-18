pub trait Suggestions {
    fn suggestions(
        &self,
        text: &[u8]
    );
}