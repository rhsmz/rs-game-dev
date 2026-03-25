# Phase 2: ECS 基盤 — 状況

## 計画（git_commit スキル）

| ブランチ | 内容 | 状態 |
|----------|------|------|
| `feature/p2a/ecs-world-entity` | World・Entity（ID 生成・再利用） | ✅ develop に反映済み |
| `feature/p2b/ecs-component-storage` | ComponentStorage（SparseSet） | ✅ develop に反映済み |
| `feature/p2c/ecs-system-scheduler` | System トレイト、Schedule、実行順 | ✅ develop に反映済み |
| `feature/p2d/ecs-query-api` | Query\<T\> / QueryMut\<T\> クエリ API | ✅ 実装済み（QueryIter / QueryIterMut、World::query / query_mut） |
| `feature/p2e/ecs-event-bus` | イベントキュー、EventReader / EventWriter | ✅ 実装済み（World に型付きキュー、get_event_writer / get_event_reader） |

## 検証

```bash
cargo test -p engine_core
cargo clippy -p engine_core -- -D warnings
```
