#[derive(Debug)]
pub enum Action {
    SubAdd,
    SubRemove,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::SubAdd => "SubAdd".to_string(),
            Action::SubRemove => "SubRemove".to_string(),
        }
    }
}
