# 07: Script System — スクリプトエンジン

## 概要
タグベースの独自スクリプト構文のパーサとステートマシンを実装する。プログラミング知識のないクリエイターが ADV パートの演出・分岐を記述できる環境を提供する。

## 依存先
- `06_asset_management`

## モジュール構成
```
crates/engine_core/src/script/
├── mod.rs          # 公開 API
├── lexer.rs        # トークナイザ
├── parser.rs       # AST パーサ
├── ast.rs          # AST ノード定義
├── vm.rs           # スクリプト実行 VM (ステートマシン)
├── command.rs      # スクリプトコマンド定義
└── error.rs        # ScriptError
```

## タスク

### 1. スクリプト構文設計
```text
; コメント行
[bg id="school_gate" transition="fade" duration="1000"]
[bgm id="morning_theme"]
[chara id="reimu" position="center" expression="smile"]
[chara id="reimu" lip_sync="on"]
[voice id="reimu_001"]
[text speaker="霊夢"]おはよう！今日はいい天気だね。[/text]
[wait click]
[choice]
  [option label="一緒に登校する" jump="walk_together"]
  [option label="先に行く" jump="go_ahead"]
[/choice]
[label walk_together]
[set var="reimu_affinity" op="add" value="1"]
[text speaker="霊夢"]嬉しい！一緒に行こう！[/text]
[if var="reimu_affinity" op="gte" value="5"]
  [text]霊夢との絆が深まった。[/text]
[/if]
[minigame type="battle" config="battle_01"]
[jump file="chapter2.txt" label="start"]
```

### 2. レキサー (Tokenizer)
```rust
pub enum Token {
    TagOpen,           // [
    TagClose,          // ]
    TagEnd,            // /
    Identifier(String),
    StringLiteral(String),
    Equals,            // =
    Text(String),      // タグ外のテキスト
    Comment(String),   // ; 行コメント
    Newline,
    Eof,
}
```

### 3. パーサ → AST
```rust
pub enum AstNode {
    /// [tag attr="value" ...]
    Tag {
        name: String,
        attributes: HashMap<String, String>,
        children: Vec<AstNode>,
    },
    /// テキストノード
    Text(String),
    /// コメント
    Comment(String),
}
```

- `[tag ...]` ... `[/tag]` のネスト構造をパース
- 属性の key="value" ペア解析

### 4. コマンド変換
```rust
pub enum ScriptCommand {
    SetBackground { id: String, transition: Transition, duration_ms: u64 },
    PlayBgm { id: String },
    StopBgm { fade_ms: u64 },
    ShowCharacter { id: String, position: Position, expression: String },
    HideCharacter { id: String },
    PlayVoice { id: String, lip_sync: bool },
    ShowText { speaker: Option<String>, text: String },
    WaitClick,
    ShowChoice { options: Vec<ChoiceOption> },
    SetVariable { name: String, op: VarOp, value: i32 },
    Conditional { var: String, op: CompareOp, value: i32, body: Vec<ScriptCommand> },
    Jump { file: Option<String>, label: String },
    Label(String),
    StartMiniGame { game_type: String, config: String },
}
```

### 5. スクリプト VM (ステートマシン)
```rust
pub struct ScriptVm {
    commands: Vec<ScriptCommand>,
    pc: usize,  // プログラムカウンタ
    state: VmState,
    variables: HashMap<String, i32>,
    call_stack: Vec<CallFrame>,
}

pub enum VmState {
    Running,
    WaitingForClick,
    WaitingForChoice,
    WaitingForMiniGame,
    Finished,
}

impl ScriptVm {
    pub fn step(&mut self, ctx: &mut ScriptContext) -> VmState;
    pub fn select_choice(&mut self, index: usize);
    pub fn resume_from_minigame(&mut self, result: MiniGameResult);
}
```

- 1 コマンド/step の逐次実行
- Wait 系コマンドで一時停止、外部入力で再開
- `jump` / `label` によるフロー制御
- `if` / `set` による条件分岐・変数操作

### 6. ECS 連携
- スクリプトコマンドを ECS イベントキューに変換
- テキスト表示 → UI コマンド
- キャラ表示 → ECS Component 操作
- ミニゲーム → Scene 遷移

## テスト計画
- レキサー: 各トークン種別の正確なトークナイズ
- パーサ: ネスト構造・属性解析の正確性
- VM: 分岐・ループ・変数操作の動作
- コマンド変換: AST → ScriptCommand の正確性

## 完了条件
- サンプルスクリプトのパースが成功
- VM でテキスト表示・選択肢・分岐が動作
- 変数操作 (set/if) が正確
