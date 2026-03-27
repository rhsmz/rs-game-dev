//! 入力イベント関連の型定義。

/// キーボードキー（最小実装）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Escape,
    Character(char),
}

/// マウスボタン。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

/// ゲームパッド識別子。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(pub u32);

/// ゲームパッドボタン（最小実装）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    South,
    East,
    West,
    North,
    LeftShoulder,
    RightShoulder,
    Start,
    Select,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Other(u16),
}

/// アクション名。
pub type ActionName = &'static str;

/// ECS へ送る入力イベント。
#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent {
    KeyPressed { key: KeyCode },
    KeyReleased { key: KeyCode },
    MouseMoved { position: [f32; 2], delta: [f32; 2] },
    MouseButtonPressed { button: MouseButton },
    MouseButtonReleased { button: MouseButton },
    ActionTriggered { name: ActionName },
}
