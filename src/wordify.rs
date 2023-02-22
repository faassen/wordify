#[derive(Debug, PartialEq, Eq)]
pub enum InputChunk<'a> {
    Equal(&'a str),
    Delete(&'a str),
    Insert(&'a str),
}

impl InputChunk<'_> {
    pub fn len(&self) -> usize {
        match self {
            InputChunk::Equal(s) => s.len(),
            InputChunk::Delete(s) => s.len(),
            InputChunk::Insert(s) => s.len(),
        }
    }

    pub fn value(&self) -> &str {
        match self {
            InputChunk::Equal(s) => s,
            InputChunk::Delete(s) => s,
            InputChunk::Insert(s) => s,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OutputChunk {
    Equal(String),
    Delete(String),
    Insert(String),
}

#[derive(Debug, PartialEq, Eq)]
struct Annotation<'a> {
    start: usize,
    chunk: &'a InputChunk<'a>,
}

#[derive(Debug, PartialEq, Eq)]
struct AnnotatedString<'a> {
    string: String,
    annotations: Vec<Annotation<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
enum WordCategory {
    Between,
    Word,
}

#[derive(Debug, PartialEq, Eq)]
struct AnnotatedWord<'a> {
    category: WordCategory,
    word: String,
    chunks: Vec<&'a InputChunk<'a>>,
}

fn reconstruct_a<'a>(chunks: &'a [InputChunk<'a>]) -> AnnotatedString<'a> {
    let mut string = String::new();
    let mut annotations = Vec::new();
    for chunk in chunks {
        let start = string.len();
        let included = match chunk {
            InputChunk::Equal(s) => {
                string.push_str(s);
                true
            }
            InputChunk::Delete(s) => {
                string.push_str(s);
                true
            }
            InputChunk::Insert(_) => false,
        };
        if included {
            annotations.push(Annotation { start, chunk });
        }
    }
    AnnotatedString {
        string,
        annotations,
    }
}

fn reconstruct_b<'a>(chunks: &'a [InputChunk<'a>]) -> AnnotatedString<'a> {
    let mut string = String::new();
    let mut annotations = Vec::new();
    for chunk in chunks {
        let start = string.len();
        let included = match chunk {
            InputChunk::Equal(s) => {
                string.push_str(s);
                true
            }
            InputChunk::Delete(_) => false,
            InputChunk::Insert(s) => {
                string.push_str(s);
                true
            }
        };
        if included {
            annotations.push(Annotation { start, chunk });
        }
    }
    AnnotatedString {
        string,
        annotations,
    }
}

fn annotated_words<'a>(annotated: &AnnotatedString<'a>) -> Vec<AnnotatedWord<'a>> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut current_chunks = Vec::new();
    let mut current_category = WordCategory::Between;

    for annotation in &annotated.annotations {
        let chunk = annotation.chunk;

        for c in chunk.value().chars() {
            if c.is_whitespace() || c.is_ascii_punctuation() {
                if current_category == WordCategory::Word {
                    words.push(AnnotatedWord {
                        category: WordCategory::Word,
                        word: current_word.clone(),
                        chunks: current_chunks.clone(),
                    });
                    current_category = WordCategory::Between;
                    current_word.clear();
                    current_chunks.clear();
                }
            } else if current_category == WordCategory::Between {
                if !current_word.is_empty() {
                    words.push(AnnotatedWord {
                        category: WordCategory::Between,
                        word: current_word.clone(),
                        chunks: current_chunks.clone(),
                    });
                }
                current_category = WordCategory::Word;
                current_word.clear();
                current_chunks.clear();
            }
            current_word.push(c);
            if current_chunks.last() != Some(&chunk) {
                current_chunks.push(chunk);
            }
        }
    }

    if !current_word.is_empty() {
        words.push(AnnotatedWord {
            category: current_category,
            word: current_word.clone(),
            chunks: current_chunks.clone(),
        });
    }
    words
}

fn wordify(chunks: &[InputChunk]) -> Vec<OutputChunk> {
    // take words of original and take words of change
    // then resequence words based on whether the word is in delete/insert at all

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_a_delete() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Delete("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_a(&chunks);
        assert_eq!(annotated.string, "Hello world");
        assert_eq!(
            annotated.annotations,
            vec![
                Annotation {
                    start: 0,
                    chunk: &chunks[0]
                },
                Annotation {
                    start: 9,
                    chunk: &chunks[1]
                },
                Annotation {
                    start: 10,
                    chunk: &chunks[2]
                },
            ]
        )
    }

    #[test]
    fn test_reconstruct_a_insert() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Insert("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_a(&chunks);
        assert_eq!(annotated.string, "Hello word");
        assert_eq!(
            annotated.annotations,
            vec![
                Annotation {
                    start: 0,
                    chunk: &chunks[0]
                },
                Annotation {
                    start: 9,
                    chunk: &chunks[2]
                },
            ]
        )
    }

    #[test]
    fn test_reconstruct_b_delete() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Delete("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_b(&chunks);
        assert_eq!(annotated.string, "Hello word");
        assert_eq!(
            annotated.annotations,
            vec![
                Annotation {
                    start: 0,
                    chunk: &chunks[0]
                },
                Annotation {
                    start: 9,
                    chunk: &chunks[2]
                },
            ]
        )
    }

    #[test]
    fn test_reconstruct_b_insert() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Insert("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_b(&chunks);
        assert_eq!(annotated.string, "Hello world");
        assert_eq!(
            annotated.annotations,
            vec![
                Annotation {
                    start: 0,
                    chunk: &chunks[0]
                },
                Annotation {
                    start: 9,
                    chunk: &chunks[1]
                },
                Annotation {
                    start: 10,
                    chunk: &chunks[2]
                },
            ]
        )
    }

    #[test]
    fn test_annotated_words_a_delete() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Delete("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_a(&chunks);
        let annotated_words = annotated_words(&annotated);

        assert_eq!(
            annotated_words,
            vec![
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "Hello".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Between,
                    word: " ".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "world".to_string(),
                    chunks: vec![&chunks[0], &chunks[1], &chunks[2]]
                },
            ]
        )
    }

    #[test]
    fn test_annotated_words_a_insert() {
        let chunks = vec![
            InputChunk::Equal("Hello wor"),
            InputChunk::Insert("l"),
            InputChunk::Equal("d"),
        ];

        let annotated = reconstruct_a(&chunks);
        let annotated_words = annotated_words(&annotated);

        assert_eq!(
            annotated_words,
            vec![
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "Hello".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Between,
                    word: " ".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "word".to_string(),
                    chunks: vec![&chunks[0], &chunks[2]]
                },
            ]
        )
    }

    #[test]
    fn test_annotated_words_a_separate_chunks() {
        let chunks = vec![
            InputChunk::Equal("Hello "),
            InputChunk::Delete("bar"),
            InputChunk::Insert("foo"),
        ];

        let annotated = reconstruct_a(&chunks);
        let annotated_words = annotated_words(&annotated);

        assert_eq!(
            annotated_words,
            vec![
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "Hello".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Between,
                    word: " ".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "bar".to_string(),
                    chunks: vec![&chunks[1]]
                },
            ]
        )
    }

    #[test]
    fn test_annotated_words_extra_whitespace() {
        let chunks = vec![
            InputChunk::Equal("Hello   "),
            InputChunk::Delete("bar"),
            InputChunk::Insert("foo"),
        ];

        let annotated = reconstruct_a(&chunks);
        let annotated_words = annotated_words(&annotated);

        assert_eq!(
            annotated_words,
            vec![
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "Hello".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Between,
                    word: "   ".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "bar".to_string(),
                    chunks: vec![&chunks[1]]
                },
            ]
        )
    }

    #[test]
    fn test_annotated_words_punctuation() {
        let chunks = vec![
            InputChunk::Equal("Hello / "),
            InputChunk::Delete("bar"),
            InputChunk::Insert("foo"),
        ];

        let annotated = reconstruct_a(&chunks);
        let annotated_words = annotated_words(&annotated);

        assert_eq!(
            annotated_words,
            vec![
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "Hello".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Between,
                    word: " / ".to_string(),
                    chunks: vec![&chunks[0]]
                },
                AnnotatedWord {
                    category: WordCategory::Word,
                    word: "bar".to_string(),
                    chunks: vec![&chunks[1]]
                },
            ]
        )
    }

    // #[test]
    // fn test_wordify() {
    //     let chunks = vec![
    //         InputChunk::Equal("Hello wor"),
    //         InputChunk::Delete("l"),
    //         InputChunk::Equal("d"),
    //     ];

    //     let chunks = wordify(&chunks);
    //     assert_eq!(
    //         chunks,
    //         vec![
    //             OutputChunk::Equal("Hello ".to_string()),
    //             OutputChunk::Delete("world".to_string()),
    //             OutputChunk::Insert("word".to_string())
    //         ]
    //     );
    // }
}
