use crate::stop_words::filter_stop_words;
use std::collections::HashMap;

struct Book {
    title: String,
    content: String,
    sorted_words: Vec<(String, i32)>,
    word_count: i32,
}

impl Book {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            sorted_words: sort_words(&content),
            word_count: content.split_whitespace().count() as i32,
            content,
        }
    }

    pub fn top_five(&self) -> &[(String, i32)] {
        let upper_bound = self.sorted_words.len().min(5);
        &self.sorted_words[0..upper_bound]
    }

    pub fn bottom_five(&self) -> &[(String, i32)] {
        let len = self.sorted_words.len();
        let start = len.saturating_sub(5);
        &self.sorted_words[start..]
    }
}

fn sort_words(content: &String) -> Vec<(String, i32)> {
    let mut word_dictionary: HashMap<String, i32> = HashMap::new();
    let uncommon_words: Vec<String> = filter_stop_words(&content);

    for word in uncommon_words {
        *word_dictionary.entry(word).or_insert(0) += 1;
    }

    let mut return_vec: Vec<_> = word_dictionary.into_iter().collect();
    return_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count () {
        let book = Book::new(
            "title".to_string(), 
            "Lorem lorem lorem lorem lorem lorem lorem Ipsum Ipsum Ipsum Ipsum dummy, dummy. Hamilton, hamilton, hamilton, bla im alone hamilton, hamilton. Tommy tommy, tommy".to_string());
        assert_eq!(book.word_count, 24)
    }

    #[test]
    fn test_sort_words_single_word() {
        let content = String::from("hello hello hello hello hello world");
        let result = sort_words(&content);
        let expected = vec![("hello".to_string(), 5), ("world".to_string(), 1)];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sort_words_multiple_words() {
        let content = String::from("Hello world! Love, love, world. World! Peace");
        let mut result = sort_words(&content);
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

    #[test]
    fn test_top_five() {
        let book = Book::new(
            "title".to_string(), 
            "Lorem lorem lorem lorem lorem lorem lorem Ipsum Ipsum Ipsum Ipsum dummy, dummy. Hamilton, hamilton, hamilton, bla im alone hamilton, hamilton. Tommy tommy, tommy".to_string());
        let result: &[(String, i32)] = book.top_five();

        let expected: [(String, i32); 5] = [
            ("lorem".to_string(), 7),
            ("hamilton".to_string(), 5),
            ("ipsum".to_string(), 4),
            ("tommy".to_string(), 3),
            ("dummy".to_string(), 2),
        ];
        assert_eq!(result, &expected[..]);
    }

    #[test]
    fn test_bottom_five() {
        let book = Book::new(
            "title".to_string(), 
            "Lorem lorem lorem lorem lorem lorem lorem Ipsum ipsum ipsum Ipsum Ipsum blah tommy tommy Ipsum dummy bla bla, dummy dummy. Hamilton dummy Hamilton, hamilton, hamilton, bla blah alone hamilton, hamilton. Tommy tommy, tommy".to_string());
        let result = book.bottom_five();

        let expected: [(String, i32); 5] = [
            ("tommy".to_string(), 5),
            ("dummy".to_string(), 4),
            ("bla".to_string(), 3),
            ("blah".to_string(), 2),
            ("alone".to_string(), 1),
        ];

        assert_eq!(result, expected);
    }
}
