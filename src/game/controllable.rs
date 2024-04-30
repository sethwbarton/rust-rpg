use std::collections::HashSet;
use nannou::App;
use nannou::event::Key;
use nannou::state::keys::Down;

pub trait Controllable {
    fn handle_input(self: &mut Self, down_keys: &HashSet<Key>) -> ();
}