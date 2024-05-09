use crate::game::game_state::game_state::GameState;
use crate::{ZOOM_MAX, ZOOM_MIN};
use nannou::event::MouseScrollDelta;
use nannou::winit::event::{VirtualKeyCode, WindowEvent};
use nannou::App;
use std::collections::HashSet;

pub fn handle_raw_window_event(_app: &App, model: &mut GameState, event: &WindowEvent) {
    match event {
        WindowEvent::Resized(_) => {}
        WindowEvent::Moved(_) => {}
        WindowEvent::CloseRequested => {}
        WindowEvent::Destroyed => {}
        WindowEvent::DroppedFile(_) => {}
        WindowEvent::HoveredFile(_) => {}
        WindowEvent::HoveredFileCancelled => {}
        WindowEvent::ReceivedCharacter(_) => {}
        WindowEvent::Focused(_) => {}
        WindowEvent::KeyboardInput { .. } => {}
        WindowEvent::ModifiersChanged(_) => {}
        WindowEvent::Ime(_) => {}
        WindowEvent::CursorMoved { .. } => {}
        WindowEvent::CursorEntered { .. } => {}
        WindowEvent::CursorLeft { .. } => {}
        WindowEvent::MouseInput { .. } => {}
        WindowEvent::TouchpadMagnify { .. } => {}
        WindowEvent::SmartMagnify { .. } => {}
        WindowEvent::TouchpadRotate { .. } => {}
        WindowEvent::TouchpadPressure { .. } => {}
        WindowEvent::AxisMotion { .. } => {}
        WindowEvent::Touch(_) => {}
        WindowEvent::ScaleFactorChanged { .. } => {}
        WindowEvent::ThemeChanged(_) => {}
        WindowEvent::Occluded(_) => {}
        WindowEvent::MouseWheel {
            device_id,
            delta,
            phase,
            modifiers: _,
        } => match delta {
            MouseScrollDelta::LineDelta(_, y) => handle_scroll_lines(*y, model),
            MouseScrollDelta::PixelDelta(pixels) => handle_scroll_pixels(pixels.y, model),
        },
    }

    // Let egui also handle any events
    match &mut model.egui {
        Some(ref mut egui) => egui.handle_raw_event(event),
        None => {
            panic!("Egui wasn't initialized in handle event")
        }
    }
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

pub fn handle_scroll_pixels(scroll_delta: f64, _model: &mut GameState) {
    let tapered_y_scroll = scroll_delta * _model.settings.zoom_sensitivity;
    handle_scroll(_model, tapered_y_scroll);
}

pub fn handle_scroll_lines(scroll_delta: f32, _model: &mut GameState) {
    let tapered_y_scroll = scroll_delta as f64 * _model.settings.zoom_sensitivity;
    handle_scroll(_model, tapered_y_scroll);
}

fn handle_scroll(_model: &mut GameState, tapered_y_scroll: f64) -> bool {
    if _model.scale + tapered_y_scroll > ZOOM_MAX {
        _model.scale = ZOOM_MAX;
        return true;
    }
    if _model.scale + tapered_y_scroll < ZOOM_MIN {
        _model.scale = ZOOM_MIN;
        return true;
    }
    _model.scale = _model.scale + tapered_y_scroll;
    false
}
