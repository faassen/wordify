use dissimilar::{diff, Chunk};

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

impl<'a> From<&'a Chunk<'a>> for InputChunk<'a> {
    fn from(chunk: &'a Chunk<'a>) -> Self {
        match chunk {
            Chunk::Equal(s) => InputChunk::Equal(s),
            Chunk::Delete(s) => InputChunk::Delete(s),
            Chunk::Insert(s) => InputChunk::Insert(s),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl<'a> AnnotatedWord<'a> {
    fn is_equal(&self) -> bool {
        self.chunks.len() == 1
            && !matches!(
                self.chunks[0],
                InputChunk::Insert(_) | InputChunk::Delete(_)
            )
    }
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

impl<'a> AnnotatedString<'a> {
    fn words(&self) -> Vec<AnnotatedWord<'a>> {
        let mut words = Vec::new();
        let mut current_word = String::new();
        let mut current_chunks = Vec::new();
        let mut current_category = WordCategory::Between;

        for annotation in &self.annotations {
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
}

fn wordify(chunks: &[InputChunk]) -> Vec<OutputChunk> {
    // take words of original and take words of change
    // then resequence words based on whether the word is in delete/insert at all
    let annotated_a = reconstruct_a(chunks);
    let annotated_b = reconstruct_b(chunks);
    let words_a = annotated_a.words();
    let words_b = annotated_b.words();
    let mut words_a_iter = words_a.iter().peekable();
    let words_b_iter = words_b.iter();
    let mut output = Vec::new();
    for word_b in words_b_iter {
        if word_b.is_equal() {
            output.push(OutputChunk::Equal(word_b.word.clone()));
            // the same word must exist in a
            words_a_iter.next();
            // a may contain deltes, which we can output
            while let Some(word_a) = words_a_iter.peek() {
                if word_a.is_equal() {
                    break;
                } else {
                    output.push(OutputChunk::Delete(word_a.word.clone()));
                    words_a_iter.next();
                }
            }
        } else {
            output.push(OutputChunk::Insert(word_b.word.clone()));
            // a may contain deletes, which we can output
            while let Some(word_a) = words_a_iter.peek() {
                if word_a.is_equal() {
                    break;
                } else {
                    output.push(OutputChunk::Delete(word_a.word.clone()));
                    words_a_iter.next();
                }
            }
        }
    }
    consolidate_chunks(&output)
}

fn consolidate_chunks(chunks: &[OutputChunk]) -> Vec<OutputChunk> {
    let mut consolidated = Vec::new();
    for chunk in chunks.iter() {
        let last_chunk = consolidated.last_mut();
        if let Some(last_chunk) = last_chunk {
            match (last_chunk, chunk) {
                (OutputChunk::Equal(a), OutputChunk::Equal(b)) => {
                    a.push_str(b);
                }
                (OutputChunk::Delete(a), OutputChunk::Delete(b)) => {
                    a.push_str(b);
                }
                (OutputChunk::Insert(a), OutputChunk::Insert(b)) => {
                    a.push_str(b);
                }
                _ => {
                    let cloned = chunk.clone();
                    consolidated.push(cloned);
                }
            }
        } else {
            let cloned = chunk.clone();
            consolidated.push(cloned);
        }
    }
    consolidated
}

fn wordify_diff(a: &str, b: &str) -> Vec<OutputChunk> {
    let chunks = diff(a, b);
    wordify(&chunks.iter().map(|chunk| chunk.into()).collect::<Vec<_>>())
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
        let annotated_words = annotated.words();

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
        let annotated_words = annotated.words();

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
        let annotated_words = annotated.words();

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
        let annotated_words = annotated.words();

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
        let annotated_words = annotated.words();

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

    #[test]
    fn test_wordify2() {
        let chunks = vec![
            InputChunk::Equal("Hello "),
            InputChunk::Delete("foo"),
            InputChunk::Insert("bar"),
        ];

        let chunks = wordify(&chunks);
        assert_eq!(
            chunks,
            vec![
                OutputChunk::Equal("Hello ".to_string()),
                OutputChunk::Delete("foo".to_string()),
                OutputChunk::Insert("bar".to_string())
            ]
        );
    }

    #[test]
    fn test_wordify_diff() {
        let a = "Hello world";
        let b = "Hello word, bye universe!";
        let chunks = wordify_diff(a, b);
        assert_eq!(
            chunks,
            vec![
                OutputChunk::Equal("Hello ".to_string()),
                OutputChunk::Delete("world".to_string()),
                OutputChunk::Insert("word, bye universe!".to_string()),
            ]
        );
    }
}
