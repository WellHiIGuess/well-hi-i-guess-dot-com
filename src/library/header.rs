use super::element::Element;

pub struct Header {
    pub h_type: String,
    pub text: String,
    pub style: Option<String>,
}

impl Header {
    pub fn new(h_type: &str, text: &str, style: Option<&str>) -> Header {
        if style != None {
            return Header {
                h_type: h_type.to_string(),
                text: text.to_string(),
                style: Some(style.unwrap().to_string()),
            };
        }
        
        Header {
            h_type: h_type.to_string(),
            text: text.to_string(),
            style: None,
        }
    }
}

impl Element for Header {
    fn get_html(&self) -> String {
        let mut style_imp = String::new();

        if self.style != None {
            style_imp = " style=\"".to_owned() + self.style.as_ref().unwrap().as_str() + "\" ";
        }
        println!("{}", style_imp);

        "<h".to_owned() + self.h_type.as_str() + &style_imp + ">" + self.text.as_str() + "</h" + self.h_type.as_str() + ">"
    }
}
