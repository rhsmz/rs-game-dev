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
        Self::apply_down(&mut self.keyboard, key);
    }

    /// キー解放を反映する。
    pub fn set_key_up(&mut self, key: KeyCode) {
        Self::apply_up(&mut self.keyboard, key);
    }

    /// マウスボタン押下を反映する。
    pub fn set_mouse_button_down(&mut self, button: MouseButton) {
        Self::apply_down(&mut self.mouse_buttons, button);
    }

    /// マウスボタン解放を反映する。
    pub fn set_mouse_button_up(&mut self, button: MouseButton) {
        Self::apply_up(&mut self.mouse_buttons, button);
    }

    /// ゲームパッドボタン押下を反映する。
    pub fn set_gamepad_button_down(&mut self, gamepad: GamepadId, button: GamepadButton) {
        let state = self.gamepads.entry(gamepad).or_default();
        Self::apply_down(&mut state.pressed, button);
    }

    /// ゲームパッドボタン解放を反映する。
    ///
    /// 当該ゲームパッドが未登録（一度も押下イベントが無い）の場合は何もしない。
    /// 未知の `GamepadId` からの解放イベントだけで空の [`GamepadState`] を作らない。
    pub fn set_gamepad_button_up(&mut self, gamepad: GamepadId, button: GamepadButton) {
        if let Some(state) = self.gamepads.get_mut(&gamepad) {
            Self::apply_up(&mut state.pressed, button);
        }
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
        // 全ボタンが未押下になったゲームパッドはエントリごと削除（空の蓄積を防ぐ）
        self.gamepads.retain(|_, state| !state.pressed.is_empty());
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

    fn apply_down<T: Copy + Eq + std::hash::Hash>(map: &mut HashMap<T, ButtonPhase>, key: T) {
        use std::collections::hash_map::Entry;

        match map.entry(key) {
            Entry::Vacant(entry) => {
                entry.insert(ButtonPhase::JustPressed);
            }
            Entry::Occupied(mut entry) => {
                if matches!(entry.get(), ButtonPhase::JustReleased) {
                    entry.insert(ButtonPhase::JustPressed);
                }
            }
        }
    }

    fn apply_up<T: Copy + Eq + std::hash::Hash>(map: &mut HashMap<T, ButtonPhase>, key: T) {
        use std::collections::hash_map::Entry;

        if let Entry::Occupied(mut entry) = map.entry(key) {
            if !matches!(entry.get(), ButtonPhase::JustReleased) {
                entry.insert(ButtonPhase::JustReleased);
            }
        }
    }

    /// ゲームパッドボタンが押下中かどうか。
    #[must_use]
    pub fn is_gamepad_pressed(&self, gamepad: GamepadId, button: GamepadButton) -> bool {
        self.gamepads.get(&gamepad).is_some_and(|state| state.pressed.contains_key(&button))
    }

    /// このフレームでゲームパッドボタンが押されたか。
    #[must_use]
    pub fn is_gamepad_just_pressed(&self, gamepad: GamepadId, button: GamepadButton) -> bool {
        self.gamepads
            .get(&gamepad)
            .and_then(|state| state.pressed.get(&button))
            .is_some_and(|phase| matches!(phase, ButtonPhase::JustPressed))
    }

    /// このフレームでゲームパッドボタンが離されたか。
    #[must_use]
    pub fn is_gamepad_just_released(&self, gamepad: GamepadId, button: GamepadButton) -> bool {
        self.gamepads
            .get(&gamepad)
            .and_then(|state| state.pressed.get(&button))
            .is_some_and(|phase| matches!(phase, ButtonPhase::JustReleased))
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

    #[test]
    fn test_input_state_key_repress_in_same_frame_remains_pressed() {
        let mut state = InputState::new();
        state.set_key_down(KeyCode::Enter);
        state.set_key_up(KeyCode::Enter);
        state.set_key_down(KeyCode::Enter);
        assert!(state.is_just_pressed(KeyCode::Enter));

        state.tick();
        assert!(state.is_pressed(KeyCode::Enter));
    }

    #[test]
    fn test_input_state_mouse_repress_in_same_frame_remains_pressed() {
        let mut state = InputState::new();
        state.set_mouse_button_down(MouseButton::Left);
        state.set_mouse_button_up(MouseButton::Left);
        state.set_mouse_button_down(MouseButton::Left);
        assert!(state.is_mouse_just_pressed(MouseButton::Left));

        state.tick();
        assert!(state.is_mouse_pressed(MouseButton::Left));
    }

    #[test]
    fn test_input_state_gamepad_repress_in_same_frame_remains_pressed() {
        let mut state = InputState::new();
        let id = GamepadId(0);
        state.set_gamepad_button_down(id, GamepadButton::South);
        state.set_gamepad_button_up(id, GamepadButton::South);
        state.set_gamepad_button_down(id, GamepadButton::South);
        assert!(state.is_gamepad_just_pressed(id, GamepadButton::South));

        state.tick();
        assert!(state.is_gamepad_pressed(id, GamepadButton::South));
        assert!(!state.is_gamepad_just_released(id, GamepadButton::South));
    }

    #[test]
    fn test_input_state_key_up_without_press_is_ignored() {
        let mut state = InputState::new();
        state.set_key_up(KeyCode::Enter);
        assert!(!state.is_just_released(KeyCode::Enter));
    }

    #[test]
    fn test_input_state_mouse_up_without_press_is_ignored() {
        let mut state = InputState::new();
        state.set_mouse_button_up(MouseButton::Left);
        assert!(!state.is_mouse_just_released(MouseButton::Left));
    }

    #[test]
    fn test_input_state_gamepad_up_without_press_is_ignored() {
        let mut state = InputState::new();
        let id = GamepadId(0);
        state.set_gamepad_button_up(id, GamepadButton::South);
        assert!(!state.is_gamepad_just_released(id, GamepadButton::South));
        assert!(
            !state.gamepads.contains_key(&id),
            "未押下の up では gamepads にエントリを作らない"
        );
    }

    #[test]
    fn test_input_state_gamepad_up_unknown_ids_do_not_accumulate() {
        let mut state = InputState::new();
        for i in 0..64 {
            state.set_gamepad_button_up(GamepadId(i), GamepadButton::South);
        }
        assert!(state.gamepads.is_empty(), "未知の GamepadId への up のみでは gamepads が増えない");
    }

    #[test]
    fn test_input_state_gamepad_cleared_after_release_tick() {
        let mut state = InputState::new();
        let id = GamepadId(0);
        state.set_gamepad_button_down(id, GamepadButton::South);
        state.set_gamepad_button_up(id, GamepadButton::South);
        state.tick();
        assert!(
            !state.gamepads.contains_key(&id),
            "解放後の tick で空のゲームパッドエントリは削除される"
        );
    }
}
