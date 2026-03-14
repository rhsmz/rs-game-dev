# 01: ECS Foundation — Entity Component System 基盤

## 概要
ゲームエンジンの中核となる ECS (Entity Component System) を実装する。全ゲームオブジェクト（3Dモデル、Live2D、スプライト、サウンドソース）を Entity として管理し、Component でデータ、System でロジックを分離する。

## 依存先
- `00_project_scaffolding`

## モジュール構成
```
crates/engine_core/src/ecs/
├── mod.rs          # 公開 API re-export
├── world.rs        # World (Entity/Component ストレージ)
├── entity.rs       # Entity ID 管理
├── component.rs    # Component トレイト & ストレージ
├── system.rs       # System トレイト & スケジューラ
├── query.rs        # Component クエリ
├── event.rs        # イベントキュー (ECS ↔ UI 通信用)
└── resource.rs     # グローバルリソース管理
```

## タスク

### 1. Entity 管理
```rust
/// Entity は u64 の ID。Generation + Index で再利用を安全に管理。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    generation: u32,
}

pub struct EntityAllocator {
    next_id: u32,
    free_list: Vec<Entity>,
    generations: Vec<u32>,
}
```

- Entity の生成・破棄・世代管理
- Free list による ID 再利用
- `is_alive(entity)` チェック

### 2. Component ストレージ
```rust
pub trait Component: 'static + Send + Sync {}

/// SparseSet ベースの高速コンポーネントストレージ
pub struct ComponentStorage<T: Component> {
    dense: Vec<T>,
    dense_to_entity: Vec<Entity>,
    sparse: Vec<Option<usize>>,
}
```

- `ComponentStorage<T>` — SparseSet 方式で O(1) 挿入/削除/参照
- `insert(entity, component)`, `remove(entity)`, `get(entity) -> Option<&T>`
- 変更検知用の `Changed` フラグ（UI 同期 System 向け）

### 3. World
```rust
pub struct World {
    entities: EntityAllocator,
    components: HashMap<TypeId, Box<dyn AnyComponentStorage>>,
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    event_queues: EventQueues,
}
```

- Entity の生成 (`spawn`) / 破棄 (`despawn`)
- Component の追加 (`insert_component`) / 削除 (`remove_component`)
- Resource の登録・取得
- 型消去による動的ディスパッチ

### 4. クエリシステム
```rust
/// 複数 Component を同時に取得するクエリ
pub struct Query<'w, Q: QueryParam> {
    world: &'w World,
    _marker: PhantomData<Q>,
}

pub trait QueryParam {
    type Item<'a>;
    fn fetch(world: &World, entity: Entity) -> Option<Self::Item<'_>>;
}

// 使用例:
// for (entity, (transform, model)) in query::<(&Transform, &Model3D)>(world) { ... }
```

- タプル型による複数 Component クエリ
- `&T` (読み取り) / `&mut T` (書き込み) 参照
- `With<T>` / `Without<T>` フィルタ

### 5. System トレイトとスケジューラ
```rust
pub trait System: Send + Sync {
    fn run(&mut self, world: &mut World);
}

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
    // 将来: 依存関係グラフによる並列実行
}

impl Schedule {
    pub fn add_system(&mut self, system: impl System + 'static);
    pub fn run(&mut self, world: &mut World);
}
```

- System の登録と順次実行
- Phase 分割: `PreUpdate` → `Update` → `PostUpdate` → `Render`

### 6. イベントキュー (メッセージパッシング)
```rust
/// UI → ECS, ECS → UI の疎結合通信チャネル
pub struct EventQueue<T: Send + 'static> {
    events: Vec<T>,
}

impl<T: Send + 'static> EventQueue<T> {
    pub fn send(&mut self, event: T);
    pub fn drain(&mut self) -> impl Iterator<Item = T>;
    pub fn iter(&self) -> impl Iterator<Item = &T>;
}
```

- 型付きイベントキュー
- フレーム単位でのイベント蓄積・消費
- UI コマンドキュー / ECS→UI 変更通知に使用

### 7. 基本 Component 定義
```rust
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4], // Quaternion
    pub scale: [f32; 3],
}

pub struct Name(pub String);

pub struct Active(pub bool);
```

## テスト計画
- Entity 生成・破棄・世代チェック
- Component 挿入・削除・取得
- Query による複数 Component イテレーション
- EventQueue の send/drain サイクル
- World の統合テスト

## 完了条件
- `cargo test -p engine_core` の ECS 関連テスト全パス
- Entity 1万件の生成・クエリが 1ms 以内
