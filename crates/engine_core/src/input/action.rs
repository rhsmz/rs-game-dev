//! 物理入力からアクション名へのマッピング。

use std::collections::HashMap;

use super::event::{ActionName, GamepadButton, GamepadId, KeyCode, MouseButton};
use super::state::InputState;

/// 物理入力。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhysicalInput {
    Key(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadId, GamepadButton),
}

/// アクションマップ。
#[derive(Debug, Default, Clone)]
pub struct ActionMap {
    bindings: HashMap<ActionName, Vec<PhysicalInput>>,
}

impl ActionMap {
    /// 新しい空の `ActionMap` を作る。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// アクションへ物理入力をバインドする。
    pub fn bind(&mut self, action: ActionName, input: PhysicalInput) {
        self.bindings.entry(action).or_default().push(input);
    }

    /// 入力状態からトリガー済みアクション一覧を返す。
    #[must_use]
    pub fn resolve(&self, state: &InputState) -> Vec<ActionName> {
        self.bindings
            .iter()
            .filter_map(|(action, inputs)| {
                if inputs.iter().any(|input| Self::is_triggered(*input, state)) {
                    Some(*action)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_triggered(input: PhysicalInput, state: &InputState) -> bool {
        match input {
            PhysicalInput::Key(key) => state.is_just_pressed(key),
            PhysicalInput::MouseButton(button) => state.is_mouse_just_pressed(button),
            PhysicalInput::GamepadButton(id, button) => state.is_gamepad_just_pressed(id, button),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_map_resolve_key_just_pressed() {
        let mut map = ActionMap::new();
        map.bind("confirm", PhysicalInput::Key(KeyCode::Enter));

        let mut state = InputState::new();
        state.set_key_down(KeyCode::Enter);

        let actions = map.resolve(&state);
        assert_eq!(actions, vec!["confirm"]);

        state.tick();
        let actions_after_tick = map.resolve(&state);
        assert!(actions_after_tick.is_empty());
    }

    #[test]
    fn test_action_map_resolve_mouse_button_just_pressed() {
        let mut map = ActionMap::new();
        map.bind("click", PhysicalInput::MouseButton(MouseButton::Left));

        let mut state = InputState::new();
        state.set_mouse_button_down(MouseButton::Left);
        let actions = map.resolve(&state);
        assert_eq!(actions, vec!["click"]);
    }

    #[test]
    fn test_action_map_resolve_gamepad_button_just_pressed() {
        let mut map = ActionMap::new();
        map.bind("jump", PhysicalInput::GamepadButton(GamepadId(1), GamepadButton::South));

        let mut state = InputState::new();
        state.set_gamepad_button_down(GamepadId(1), GamepadButton::South);
        let actions = map.resolve(&state);
        assert_eq!(actions, vec!["jump"]);

        state.tick();
        assert!(map.resolve(&state).is_empty());
    }
}
