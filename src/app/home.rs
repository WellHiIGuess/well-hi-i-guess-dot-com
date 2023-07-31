use crate::library::{page::page, header::Header};

pub fn home() -> String {
    page(vec![
        &Header{ h_type: "1".to_string(), text: "Hello, World".to_string() },
    ])
}
