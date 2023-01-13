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

    pub fn set_state(&mut self, state: AgendaState){
        self.state = state;
    }

    pub fn to_new_state(state: &str) -> AgendaState {
      match state {
            "OK" => AgendaState::DELETED,
            "PENDING" => AgendaState::OK,
            "DELETED" => AgendaState::DELETED,
            "TODO" => AgendaState::PENDING,
            _ => AgendaState::TODO
        }
    }
}