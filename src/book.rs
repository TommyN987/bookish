use crate::analytics::Analytics;

#[allow(unused)]

pub struct Book {
    title: String,
    content: String,
    analytics: Analytics,
}

#[allow(unused)]

impl Book {
    pub fn new(title: String, content: String) -> Self {
        Self {
            title,
            analytics: Analytics::new(&content),
            content,
        }
    }
}
