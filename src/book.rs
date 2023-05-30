use crate::stop_words::filter_stop_words;
use std::collections::HashMap;

struct Book {
    title: String,
    content: String,
}

impl Book {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    fn sort_words(&self) -> Vec<(String, i32)> {
        let mut word_dictionary: HashMap<String, i32> = HashMap::new();
        let uncommon_words: Vec<String> = filter_stop_words(&self.content);

        for word in uncommon_words {
            *word_dictionary.entry(word).or_insert(0) += 1;
        }

        let mut return_vec: Vec<_> = word_dictionary.into_iter().collect();
        return_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        return_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_words_single_word() {
        let book = Book {
            title: String::from("Test Book"),
            content: String::from("hello hello hello hello hello world"),
        };
        let result = book.sort_words();
        let expected = vec![("hello".to_string(), 5), ("world".to_string(), 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sort_words_multiple_words() {
        let content = "Hello world! Love, love, world. World! Peace";
        let book = Book {
            title: "Test".to_string(),
            content: content.to_string(),
        };
        let mut result = book.sort_words();
        let mut expected = vec![
            ("world".to_string(), 3),
            ("love".to_string(), 2),
            ("hello".to_string(), 1),
            ("peace".to_string(), 1),
        ];
        result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        expected.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        assert_eq!(expected, result);
    }
}
