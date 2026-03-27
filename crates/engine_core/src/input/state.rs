//! 入力状態管理。

use std::collections::{HashMap, HashSet};

use super::event::{GamepadButton, GamepadId, KeyCode, MouseButton};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ButtonPhase {
    Pressed,
    JustPressed,
    JustReleased,
}

/// 1P 分のゲームパッド状態（最小実装）。
#[derive(Debug, Default, Clone)]
pub struct GamepadState {
    pressed: HashMap<GamepadButton, ButtonPhase>,
}

/// 入力状態をフレーム単位で管理する。
#[derive(Debug, Default, Clone)]
pub struct InputState {
    keyboard: HashMap<KeyCode, ButtonPhase>,
    mouse_buttons: HashMap<MouseButton, ButtonPhase>,
    gamepads: HashMap<GamepadId, GamepadState>,
    mouse_position: [f32; 2],
    mouse_delta: [f32; 2],
}

impl InputState {
    /// 新しい入力状態を作る。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// キーが押下中かどうか。
    #[must_use]
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.contains_key(&key)
    }

    /// このフレームでキーが押されたか。
    #[must_use]
    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        matches!(self.keyboard.get(&key), Some(ButtonPhase::JustPressed))
    }

    /// このフレームでキーが離されたか。
    #[must_use]
    pub fn is_just_released(&self, key: KeyCode) -> bool {
        matches!(self.keyboard.get(&key), Some(ButtonPhase::JustReleased))
    }

    /// マウスボタンが押下中かどうか。
    #[must_use]
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains_key(&button)
    }

    /// このフレームでマウスボタンが押されたか。
    #[must_use]
    pub fn is_mouse_just_pressed(&self, button: MouseButton) -> bool {
        matches!(self.mouse_buttons.get(&button), Some(ButtonPhase::JustPressed))
    }

    /// このフレームでマウスボタンが離されたか。
    #[must_use]
    pub fn is_mouse_just_released(&self, button: MouseButton) -> bool {
        matches!(self.mouse_buttons.get(&button), Some(ButtonPhase::JustReleased))
    }

    /// 現在のマウス座標。
    #[must_use]
    pub const fn mouse_position(&self) -> [f32; 2] {
        self.mouse_position
    }

    /// このフレームのマウス移動量。
    #[must_use]
    pub const fn mouse_delta(&self) -> [f32; 2] {
        self.mouse_delta
    }

    /// キー押下を反映する。
    pub fn set_key_down(&mut self, key: KeyCode) {
        self.keyboard.entry(key).or_insert(ButtonPhase::JustPressed);
    }

    /// キー解放を反映する。
    pub fn set_key_up(&mut self, key: KeyCode) {
        self.keyboard.insert(key, ButtonPhase::JustReleased);
    }

    /// マウスボタン押下を反映する。
    pub fn set_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_buttons.entry(button).or_insert(ButtonPhase::JustPressed);
    }

    /// マウスボタン解放を反映する。
    pub fn set_mouse_button_up(&mut self, button: MouseButton) {
        self.mouse_buttons.insert(button, ButtonPhase::JustReleased);
    }

    /// ゲームパッドボタン押下を反映する。
    pub fn set_gamepad_button_down(&mut self, gamepad: GamepadId, button: GamepadButton) {
        let state = self.gamepads.entry(gamepad).or_default();
        state.pressed.entry(button).or_insert(ButtonPhase::JustPressed);
    }

    /// ゲームパッドボタン解放を反映する。
    pub fn set_gamepad_button_up(&mut self, gamepad: GamepadId, button: GamepadButton) {
        let state = self.gamepads.entry(gamepad).or_default();
        state.pressed.insert(button, ButtonPhase::JustReleased);
    }

    /// マウス座標を更新し、差分を計算する。
    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_delta = [x - self.mouse_position[0], y - self.mouse_position[1]];
        self.mouse_position = [x, y];
    }

    /// フレーム終端処理。
    ///
    /// - `JustPressed` は `Pressed` に遷移
    /// - `JustReleased` は未押下へ遷移（マップから削除）
    /// - `mouse_delta` は 0 にリセット
    pub fn tick(&mut self) {
        self.promote_button_phases();
        self.mouse_delta = [0.0, 0.0];
    }

    fn promote_button_phases(&mut self) {
        Self::promote_map(&mut self.keyboard);
        Self::promote_map(&mut self.mouse_buttons);
        self.promote_gamepad_states();
    }

    fn promote_gamepad_states(&mut self) {
        for state in self.gamepads.values_mut() {
            Self::promote_map(&mut state.pressed);
        }
    }

    fn promote_map<T: Copy + Eq + std::hash::Hash>(map: &mut HashMap<T, ButtonPhase>) {
        let released: HashSet<T> =
            map.iter()
                .filter_map(|(key, phase)| {
                    if matches!(phase, ButtonPhase::JustReleased) { Some(*key) } else { None }
                })
                .collect();

        for phase in map.values_mut() {
            if matches!(phase, ButtonPhase::JustPressed) {
                *phase = ButtonPhase::Pressed;
            }
        }
        for key in released {
            map.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;

    #[test]
    fn test_input_state_key_transitions_across_frames() {
        let mut state = InputState::new();
        state.set_key_down(KeyCode::Space);
        assert!(state.is_just_pressed(KeyCode::Space));
        assert!(state.is_pressed(KeyCode::Space));

        state.tick();
        assert!(!state.is_just_pressed(KeyCode::Space));
        assert!(state.is_pressed(KeyCode::Space));

        state.set_key_up(KeyCode::Space);
        assert!(state.is_just_released(KeyCode::Space));
        state.tick();
        assert!(!state.is_just_released(KeyCode::Space));
        assert!(!state.is_pressed(KeyCode::Space));
    }

    #[test]
    fn test_input_state_mouse_delta_resets_on_tick() {
        let mut state = InputState::new();
        state.set_mouse_position(10.0, 20.0);
        assert_eq!(state.mouse_delta(), [10.0, 20.0]);
        state.tick();
        assert_eq!(state.mouse_delta(), [0.0, 0.0]);
    }
}
