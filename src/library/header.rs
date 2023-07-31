use super::element::Element;

pub struct Header {
    pub h_type: String,
    pub text: String,
}

impl Element for Header {
    fn get_html(&self) -> String {
        "<h".to_owned() + self.h_type.as_str() + ">" + self.text.as_str() + "</h" + self.h_type.as_str() + ">"
    }
}
