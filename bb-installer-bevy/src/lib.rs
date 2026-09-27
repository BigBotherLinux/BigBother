use bevy::prelude::*;

pub mod cursor;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Welcome,
    TermsOfService,
    Sign,
    Test,
    Map,
}
