#[derive(Clone, Copy)]
pub enum Life {
    Live,
    Dead,
}

pub fn render(life: &Life) -> String {
    match life {
        Life::Live => String::from("@"),
        Life::Dead => String::from(" "),
    }
}
