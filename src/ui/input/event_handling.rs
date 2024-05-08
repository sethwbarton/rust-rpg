use crate::game::game_state::game_state::GameState;
use crate::ui::input::handle_ui_clicks;
use crate::{ZOOM_MAX, ZOOM_MIN};
use nannou::event::{MouseScrollDelta, Update};
use nannou::prelude::{
    Closed, DroppedFile, Focused, HoveredFile, HoveredFileCancelled, KeyPressed, KeyReleased,
    MouseEntered, MouseExited, MouseMoved, MousePressed, MouseReleased, MouseWheel, Moved,
    ReceivedCharacter, Resized, Touch, TouchPressure, Unfocused,
};
use nannou::winit::event::VirtualKeyCode;
use nannou::Event::WindowEvent;
use nannou::{App, Event};
use std::collections::HashSet;

pub fn event(_app: &App, _model: &mut GameState, _event: Event) {
    match _event {
        WindowEvent { id: _, simple } => match simple {
            None => {}
            Some(e) => match e {
                Moved(_) => {}
                KeyPressed(_) => {}
                KeyReleased(_) => {}
                ReceivedCharacter(_) => {}
                MouseMoved(_) => {}
                MousePressed(_) => {}
                MouseReleased(_) => {}
                MouseEntered => {}
                MouseExited => {}
                MouseWheel(mouse_scroll_delta, _) => match mouse_scroll_delta {
                    MouseScrollDelta::LineDelta(_x, y) => handle_scroll(y as f64, _model),
                    MouseScrollDelta::PixelDelta(pixels) => handle_scroll(pixels.y, _model),
                },
                Resized(_) => {}
                HoveredFile(_) => {}
                DroppedFile(_) => {}
                HoveredFileCancelled => {}
                Touch(_) => {}
                TouchPressure(_) => {}
                Focused => {}
                Unfocused => {}
                Closed => {}
            },
        },
        Event::DeviceEvent(_, _) => {}
        Event::Update(_) => {}
        Event::Suspended => {}
        Event::Resumed => {}
    }
}

pub fn update(_app: &App, _model: &mut GameState, _update: Update) {
    handle_key_presses(&_app.keys.down, _model);
    handle_ui_clicks(_app);
}

pub fn handle_key_presses(down_keys: &HashSet<VirtualKeyCode>, _model: &mut GameState) {
    if down_keys.contains(&VirtualKeyCode::A) {
        _model.transform.x += _model.settings.pan_speed;
    }
    if down_keys.contains(&VirtualKeyCode::D) {
        _model.transform.x -= _model.settings.pan_speed
    }
    if down_keys.contains(&VirtualKeyCode::W) {
        _model.transform.y -= _model.settings.pan_speed;
    }
    if down_keys.contains(&VirtualKeyCode::S) {
        _model.transform.y += _model.settings.pan_speed;
    }
}

pub fn handle_scroll(scroll_delta: f64, _model: &mut GameState) {
    let tapered_y_scroll = scroll_delta * _model.settings.zoom_sensitivity;
    if _model.scale + tapered_y_scroll > ZOOM_MAX {
        _model.scale = ZOOM_MAX;
        return;
    }
    if _model.scale + tapered_y_scroll < ZOOM_MIN {
        _model.scale = ZOOM_MIN;
        return;
    }
    _model.scale = _model.scale + tapered_y_scroll;
}

#[cfg(test)]
mod tests {
    use crate::game::game_state::game_state::{GameState, Settings};
    use crate::game::ship::Ship;
    use crate::ui::input::event_handling::{handle_key_presses, handle_scroll};
    use crate::{ZOOM_MAX, ZOOM_MIN};
    use nannou::geom::pt2;
    use nannou::winit::event::VirtualKeyCode;
    use std::collections::HashSet;

    #[test]
    fn test_handle_scroll_respects_zoom_sensitivity() {
        let old_scale = 1.0;
        let mut test_model = GameState {
            ship: Ship {},
            scale: old_scale,
            transform: Default::default(),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let scroll_delta: f64 = -2.0;
        handle_scroll(scroll_delta, &mut test_model);

        assert_eq!(
            test_model.scale,
            old_scale + (scroll_delta * test_model.settings.zoom_sensitivity)
        )
    }

    #[test]
    fn test_handle_scroll_respects_max_zoom() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: ZOOM_MAX,
            transform: Default::default(),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let scroll_delta: f64 = 25.0;
        handle_scroll(scroll_delta, &mut test_model);

        assert_eq!(test_model.scale, ZOOM_MAX)
    }

    #[test]
    fn test_handle_scroll_respects_min_zoom() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: ZOOM_MIN,
            transform: Default::default(),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let scroll_delta: f64 = -25.0;
        handle_scroll(scroll_delta, &mut test_model);

        assert_eq!(test_model.scale, ZOOM_MIN)
    }

    #[test]
    fn handle_key_press_moves_screen_right_on_a() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: 1.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let down_keys: HashSet<VirtualKeyCode> = HashSet::from([VirtualKeyCode::A]);
        handle_key_presses(&down_keys, &mut test_model);

        assert_eq!(test_model.transform.x, test_model.settings.pan_speed)
    }

    #[test]
    fn handle_key_press_moves_screen_left_on_d() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: 1.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let down_keys: HashSet<VirtualKeyCode> = HashSet::from([VirtualKeyCode::D]);
        handle_key_presses(&down_keys, &mut test_model);

        assert_eq!(test_model.transform.x, -test_model.settings.pan_speed)
    }

    #[test]
    fn handle_key_press_moves_screen_down_on_w() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: 1.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let down_keys: HashSet<VirtualKeyCode> = HashSet::from([VirtualKeyCode::W]);
        handle_key_presses(&down_keys, &mut test_model);

        assert_eq!(test_model.transform.y, -test_model.settings.pan_speed)
    }

    #[test]
    fn handle_key_press_moves_screen_up_on_s() {
        let mut test_model = GameState {
            ship: Ship {},
            scale: 1.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        };

        let down_keys: HashSet<VirtualKeyCode> = HashSet::from([VirtualKeyCode::S]);
        handle_key_presses(&down_keys, &mut test_model);

        assert_eq!(test_model.transform.y, test_model.settings.pan_speed)
    }
}
