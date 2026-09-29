use bevy::prelude::*;

pub mod cursor;
pub mod network;
pub mod pages;
pub mod sign;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    Welcome,
    #[default]
    TermsOfService,
    Sign,
    Test,
    Map,
}
