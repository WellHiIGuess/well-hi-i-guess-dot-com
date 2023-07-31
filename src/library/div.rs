use super::element::{Element};

pub struct Div<'a> {
    pub elements: Vec<Box<&'a dyn Element>>,
}

impl Div<'_> {
    pub fn new(elements: Vec<&dyn Element>) -> Div {
        let mut b_elements = vec![];

        for i in elements {
            b_elements.push(Box::new(i));
        }

        Div {
            elements: b_elements,
        }
    }
}

impl Element for Div<'_> {
    fn get_html(&self) -> String {
        "<div>".to_owned() + self.elements
            .iter()
            .map(|x| x.get_html())
            .collect::<Vec<_>>()
            .join("").as_str() + "</div>"
    }
}
