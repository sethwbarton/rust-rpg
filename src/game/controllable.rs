use nannou::App;

pub trait Controllable {
    fn handle_input(self: &mut Self, _app: &App) -> ();
}