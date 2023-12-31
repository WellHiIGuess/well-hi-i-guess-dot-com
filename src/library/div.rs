use super::element::Element;

pub struct Div<'a> {
    pub elements: Vec<Box<&'a dyn Element>>,
    pub style: Option<String>,
}

impl Div<'_> {
    #[allow(unused)]
    pub fn new<'a>(elements: Vec<&'a dyn Element>, style: Option<&'a str>) -> Div<'a> {
        let mut b_elements = vec![];

        for i in elements {
            b_elements.push(Box::new(i));
        }

        if style != None {
            return Div {
                elements: b_elements,
                style: Some(style.unwrap().to_string())
            };
        }
        
        Div {
            elements: b_elements,
            style: None,
        }
    }
}

impl Element for Div<'_> {
    fn get_html(&self) -> String {
        let mut style_imp = String::new();

        if self.style != None {
            style_imp = " style=\"".to_owned() + self.style.as_ref().unwrap().as_str() + "\" ";
        }

        "<div".to_owned() + style_imp.as_str() + ">" + self.elements
            .iter()
            .map(|x| x.get_html())
            .collect::<Vec<_>>()
            .join("").as_str() + "</div>"
    }
}
