#[derive(PartialEq, Show)] // so that tokens can be compared
#[derive(Copy, Clone)] // so that boards can be copied
pub enum Token {
    X,
    O,
    Empty,
}

