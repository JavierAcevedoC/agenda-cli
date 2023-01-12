use enums::status::AgendaState;
use crate::agenda::enums;

pub struct AgendaPage {
    pub title: String,
    state: AgendaState
}

impl AgendaPage {
    pub fn new(title: String) -> Self {
        Self {
            title,
            state: AgendaState::TODO,
        }
    }

    pub fn get_state(&self) -> String {
        match self.state {
            AgendaState::OK => String::from("OK"),
            AgendaState::PENDING => String::from("PENDING"),
            AgendaState::DELETED => String::from("DELETED"),
            AgendaState::TODO  => String::from("TODO"),
        }
    }
}