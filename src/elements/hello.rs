use crate::library::{element::Element, header::Header};

pub struct Hello {
}

impl Element for Hello {
    fn get_html(&self) -> String {
        Header::new("1", "Hello", None).get_html()
    }
}
