pub mod view {
    use nannou::color::BLACK;
    use nannou::geom::Vec3;
    use nannou::{App, Frame};

    use crate::game::drawable::Drawable;
    use crate::game::game_state::game_state::GameState;

    pub fn view(_app: &App, _model: &GameState, frame: Frame) {
        let draw = _app.draw().scale(_model.scale as f32).translate(Vec3::new(
            _model.transform.x,
            _model.transform.y,
            0.0,
        ));
        frame.clear(BLACK);

        _model.ship.draw(&draw, _model);

        draw.to_frame(_app, &frame).unwrap();
    }
}
