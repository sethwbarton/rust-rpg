use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;
use crate::ui::app_view::View;
use nannou::geom::Vec3;
use nannou::{App, Frame};

pub fn draw_scene(_app: &App, _model: &GameState, frame: &Frame, view: &View) {
    let scene_draw_instance = _app.draw().scale(_model.scale as f32).translate(Vec3::new(
        _model.transform.x,
        _model.transform.y,
        0.0,
    ));
    _model.ship.draw(&scene_draw_instance, _model);
    scene_draw_instance.to_frame(_app, &frame).unwrap();
}
