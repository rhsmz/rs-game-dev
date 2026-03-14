# 08: Scene Management — シーン管理

## 概要
`Scene` トレイトによるゲームパートの抽象化、シーンスタック管理、非同期ロード遷移を実装する。

## 依存先
- `02_renderer`, `06_asset_management`

## モジュール構成
```
crates/engine_core/src/scene/
├── mod.rs              # 公開 API, Scene トレイト
├── manager.rs          # SceneManager (スタック管理)
├── context.rs          # SceneContext (シーン間データ受け渡し)
├── transition.rs       # SceneTransition, 遷移エフェクト
└── loading.rs          # ロード画面制御
```

## タスク

### 1. Scene トレイト
```rust
pub trait Scene: Send + Sync {
    fn on_enter(&mut self, ctx: &mut SceneContext);
    fn update(&mut self, ctx: &mut SceneContext, dt: f32) -> SceneTransition;
    fn render(&self, ctx: &RenderContext);
    fn on_exit(&mut self, ctx: &mut SceneContext);
    fn required_assets(&self) -> Vec<AssetDescriptor>;
}
```

### 2. SceneTransition
```rust
pub enum SceneTransition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>),
    Quit,
}
```

### 3. SceneManager
```rust
pub struct SceneManager {
    stack: Vec<Box<dyn Scene>>,
    pending_transition: Option<SceneTransition>,
    loading_state: LoadingState,
}

impl SceneManager {
    pub fn update(&mut self, world: &mut World, dt: f32);
    pub fn render(&self, ctx: &RenderContext);
}
```

- スタックベースのシーン管理
- 遷移フロー: `on_exit` → アセットパージ → 非同期ロード → `on_enter`
- ローディング画面の表示

### 4. SceneContext
```rust
pub struct SceneContext {
    pub world: World,
    pub asset_loader: AssetLoader,
    pub audio: GameAudioManager,
    pub input: InputState,
    pub minigame_result: Option<MiniGameResult>,
    messages: Vec<SceneMessage>,
}
```

- シーン間データ受け渡し（ミニゲーム結果等）
- ECS World やサブシステムへのアクセス

### 5. ローディング画面
```rust
pub struct LoadingScreen {
    progress: f32,  // 0.0 〜 1.0
    assets_total: usize,
    assets_loaded: usize,
}
```

- アセットロード進捗の表示
- ロード完了まで更新ループを実行

### 6. 遷移エフェクト
```rust
pub enum TransitionEffect {
    None,
    Fade { duration_ms: u64 },
    Slide { direction: Direction, duration_ms: u64 },
}
```

## テスト計画
- シーンスタック Push/Pop/Replace テスト
- ライフサイクル (`on_enter` → `update` → `on_exit`) 順序テスト
- ロード画面の進捗更新テスト

## 完了条件
- Scene トレイト実装でゲームパートが動作
- シーン遷移時のアセットパージ + 非同期ロード
- ミニゲーム結果の受け渡し
