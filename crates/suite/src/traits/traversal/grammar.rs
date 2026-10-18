pub trait Grammar {
    fn grammar(
        &self,
       text: &[u8]
    );
}