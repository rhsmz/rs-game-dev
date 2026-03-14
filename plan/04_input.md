# 04: Input Abstraction — 入力抽象化

## 概要
キーボード、マウス、ゲームパッドの入力を統一的に扱う抽象化レイヤー。物理入力をアクション名にマッピングし、ECS イベントキューに送信する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/input/
├── mod.rs          # 公開 API
├── keyboard.rs     # キーボード入力
├── mouse.rs        # マウス入力
├── gamepad.rs      # ゲームパッド入力
└── action_map.rs   # アクションマッピング
```

## タスク

### 1. 入力状態管理
```rust
pub struct InputState {
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,
    mouse_position: (f32, f32),
    mouse_buttons: HashSet<MouseButton>,
    mouse_delta: (f32, f32),
    mouse_scroll: f32,
}
```

- `is_pressed(key)` — 押下中
- `is_just_pressed(key)` — 今フレームで押された
- `is_just_released(key)` — 今フレームで離された
- フレーム末に `just_pressed` / `just_released` をクリア

### 2. アクションマッピング
```rust
pub struct ActionMap {
    bindings: HashMap<String, Vec<InputBinding>>,
}

pub enum InputBinding {
    Key(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButton),
}
```

- `"confirm"` → `[Key(Enter), Key(Space), GamepadButton(A)]`
- `"cancel"` → `[Key(Escape), GamepadButton(B)]`
- `action_pressed("confirm")` で統一判定

### 3. ECS 連携
```rust
pub enum InputEvent {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseMoved { x: f32, y: f32 },
    MouseClicked(MouseButton),
    ActionTriggered(String),
}
```

- 入力を `InputEvent` としてイベントキューに送信
- 各 System が必要なイベントを消費

### 4. ウィンドウイベント連携
- `winit` などのウィンドウライブラリからのイベントを変換
- `InputState` をフレーム単位で更新

## テスト計画
- キー押下 / 解放状態の正確性
- アクションマッピングの解決テスト
- フレーム間の `just_pressed` リセット

## 完了条件
- キーボード / マウス入力を ECS イベントとして取得可能
- アクションマッピングによる抽象入力判定
