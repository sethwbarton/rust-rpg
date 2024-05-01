use std::collections::HashSet;
use nannou::event::Key;

pub trait Controllable {
    fn handle_input(self: &mut Self, down_keys: &HashSet<Key>, window_size: &(u32, u32)) -> ();
}