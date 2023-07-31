use super::element::Element;

pub fn page(elements: Vec<&dyn Element>) -> String {
    let output: String = elements
        .iter()
        .map(|&m| m.get_html())
        .collect();
    output
}
