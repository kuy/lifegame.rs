#[derive(Clone)]
pub enum Life {
    Live,
    Dead,
}

pub fn render(life: &Life) -> &str {
    match life {
        Life::Live => "@",
        Life::Dead => " ",
    }
}
