use crate::library::{element::Element, paragraph::Paragraph};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Paragraph::new("Hello", None).get_html()
    }
}
