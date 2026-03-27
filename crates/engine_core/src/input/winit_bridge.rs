//! winit イベントから入力状態/入力イベントへ変換するブリッジ。

use super::event::{InputEvent, KeyCode, MouseButton};
use super::state::InputState;

const fn map_mouse_button(button: winit::event::MouseButton) -> MouseButton {
    match button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Other(4),
        winit::event::MouseButton::Forward => MouseButton::Other(5),
        winit::event::MouseButton::Other(id) => MouseButton::Other(id),
    }
}

fn map_key(event: &winit::event::KeyEvent) -> Option<KeyCode> {
    use winit::keyboard::{Key, NamedKey};

    match &event.logical_key {
        Key::Named(NamedKey::ArrowUp) => Some(KeyCode::Up),
        Key::Named(NamedKey::ArrowDown) => Some(KeyCode::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(KeyCode::Left),
        Key::Named(NamedKey::ArrowRight) => Some(KeyCode::Right),
        Key::Named(NamedKey::Space) => Some(KeyCode::Space),
        Key::Named(NamedKey::Enter) => Some(KeyCode::Enter),
        Key::Named(NamedKey::Escape) => Some(KeyCode::Escape),
        Key::Character(text) => text.chars().next().map(KeyCode::Character),
        _ => None,
    }
}

/// `winit` の `WindowEvent` を `InputEvent` へ変換する。
///
/// 同時に `InputState` も更新する。
#[must_use]
pub fn convert_winit_event(
    event: &winit::event::WindowEvent,
    state: &mut InputState,
) -> Vec<InputEvent> {
    match event {
        winit::event::WindowEvent::KeyboardInput { event, .. } => {
            map_key(event).map_or_else(Vec::new, |key| match event.state {
                winit::event::ElementState::Pressed => {
                    state.set_key_down(key);
                    vec![InputEvent::KeyPressed { key }]
                }
                winit::event::ElementState::Released => {
                    state.set_key_up(key);
                    vec![InputEvent::KeyReleased { key }]
                }
            })
        }
        winit::event::WindowEvent::CursorMoved { position, .. } => {
            #[allow(clippy::cast_possible_truncation)]
            state.set_mouse_position(position.x as f32, position.y as f32);
            vec![InputEvent::MouseMoved {
                position: state.mouse_position(),
                delta: state.mouse_delta(),
            }]
        }
        winit::event::WindowEvent::MouseInput { state: element_state, button, .. } => {
            let mapped = map_mouse_button(*button);
            match element_state {
                winit::event::ElementState::Pressed => {
                    state.set_mouse_button_down(mapped);
                    vec![InputEvent::MouseButtonPressed { button: mapped }]
                }
                winit::event::ElementState::Released => {
                    state.set_mouse_button_up(mapped);
                    vec![InputEvent::MouseButtonReleased { button: mapped }]
                }
            }
        }
        _ => Vec::new(),
    }
}
