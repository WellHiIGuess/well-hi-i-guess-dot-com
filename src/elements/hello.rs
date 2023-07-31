use crate::library::{element::Element, header::Header};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Header { h_type: "1".to_string(), text: "Hello".to_string() }.get_html()
    }
}
