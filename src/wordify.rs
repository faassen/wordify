#[derive(Debug, PartialEq, Eq)]
pub enum InputChunk<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputChunk {
    Equal(String),
    Delete(String),
    Insert(String),
}

fn wordify(chunks: &[InputChunk]) -> Vec<OutputChunk> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordify() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Delete("l"),
            InputChunk::Equal("d"),
        ];

        let chunks = wordify(&chunks);
        assert_eq!(
            chunks,
            vec![
                OutputChunk::Equal("Hello ".to_string()),
                OutputChunk::Delete("world".to_string()),
                OutputChunk::Insert("word".to_string())
            ]
        );
    }
}
