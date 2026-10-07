use bevy::prelude::*;

pub mod cursor;
pub mod network;
pub mod pages;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    Welcome,
    TermsOfService,
    #[default]
    Sign,
    Test,
    Map,
}
