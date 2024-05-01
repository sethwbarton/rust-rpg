pub mod view {
    use nannou::{App, Frame};
    use nannou::color::BLACK;

    use crate::game::drawable::Drawable;
    use crate::game::game_state::game_state::GameState;

    pub fn view(_app: &App, _model: &GameState, frame: Frame) {
        let draw = _app.draw().scale(_model.scale as f32);
        frame.clear(BLACK);

        _model.ship.draw(&draw, _model);

        draw.to_frame(_app, &frame).unwrap();
    }
}
