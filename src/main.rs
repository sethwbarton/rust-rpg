mod game;

use crate::game::drawable::Drawable;
use crate::game::ship::Ship;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou::Event::WindowEvent;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Settings {
    zoom_sensitivity: f64,
    pan_speed: f32,
}

struct Model {
    ship: Ship,
    scale: f64,
    transform: Point2,
    settings: Settings,
}

fn model(_app: &App) -> Model {
    Model {
        ship: Ship {},
        scale: 1.0,
        transform: pt2(0.0, 0.0),
        settings: Settings {
            zoom_sensitivity: 0.05,
            pan_speed: 6.0,
        },
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
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
                    MouseScrollDelta::LineDelta(_, _) => {}
                    MouseScrollDelta::PixelDelta(pixels) => {
                        let tapered_y_scroll = pixels.y * _model.settings.zoom_sensitivity;
                        if _model.scale + tapered_y_scroll <= ZOOM_MAX
                            && _model.scale + tapered_y_scroll >= ZOOM_MIN
                        {
                            _model.scale += tapered_y_scroll;
                            return;
                        }
                        if _model.scale + tapered_y_scroll > ZOOM_MAX {
                            _model.scale = ZOOM_MAX;
                            return;
                        }
                        _model.scale = ZOOM_MIN
                    }
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

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _app.keys.down.contains(&VirtualKeyCode::W) {
        _model.transform.y += _model.settings.pan_speed;
    }
    if _app.keys.down.contains(&VirtualKeyCode::A) {
        _model.transform.x -= _model.settings.pan_speed
    }
    if _app.keys.down.contains(&VirtualKeyCode::D) {
        _model.transform.x += _model.settings.pan_speed
    }
    if _app.keys.down.contains(&VirtualKeyCode::S) {
        _model.transform.y -= _model.settings.pan_speed
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw().scale(_model.scale as f32);
    frame.clear(BLACK);

    _model.ship.draw(&draw, _model);

    draw.to_frame(_app, &frame).unwrap();
}
