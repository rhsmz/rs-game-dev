# 📋 実装プラン全体概要

本ドキュメントは `rs-game-dev` ゲームエンジンの機能別実装プランの全体像を示す。

## フェーズ構成と依存関係

```mermaid
graph TD
    P0A[00 Project Scaffolding] --> P0B[01 ECS Foundation]
    P0B --> P1A[02 Renderer]
    P0B --> P1B[03 Audio]
    P0B --> P1C[04 Input]
    P0B --> P1D[05 RNG]
    P0B --> P2A[06 Asset Management]
    P2A --> P2B[07 Script System]
    P1A --> P2C[08 Scene Management]
    P2A --> P2C
    P1A --> P3A[09 Live2D Integration]
    P1A --> P3B[10 VRM Integration]
    P1A --> P4A[11 UI System]
    P0B --> P4B[12 Save System]
    P2B --> P5A[13 Script Editor]
    P4A --> P5A
    P2C --> P5B[14 Game Player]
    P4B --> P5B

    style P0A fill:#4CAF50,color:#fff
    style P0B fill:#4CAF50,color:#fff
    style P1A fill:#2196F3,color:#fff
    style P1B fill:#2196F3,color:#fff
    style P1C fill:#2196F3,color:#fff
    style P1D fill:#2196F3,color:#fff
    style P2A fill:#FF9800,color:#fff
    style P2B fill:#FF9800,color:#fff
    style P2C fill:#FF9800,color:#fff
    style P3A fill:#9C27B0,color:#fff
    style P3B fill:#9C27B0,color:#fff
    style P4A fill:#F44336,color:#fff
    style P4B fill:#F44336,color:#fff
    style P5A fill:#607D8B,color:#fff
    style P5B fill:#607D8B,color:#fff
```

## プラン一覧

| Phase | # | プラン名 | 依存先 | 推定規模 |
|-------|---|----------|--------|----------|
| **0** | [00](./00_project_scaffolding.md) | Project Scaffolding | なし | S |
| **0** | [01](./01_ecs_foundation.md) | ECS Foundation | 00 | M |
| **1** | [02](./02_renderer.md) | Renderer (Filament) | 01 | XL |
| **1** | [03](./03_audio.md) | Audio System | 01 | M |
| **1** | [04](./04_input.md) | Input Abstraction | 01 | S |
| **1** | [05](./05_rng.md) | Dual RNG | 01 | S |
| **2** | [06](./06_asset_management.md) | Asset Management | 01 | L |
| **2** | [07](./07_script_system.md) | Script System | 06 | L |
| **2** | [08](./08_scene_management.md) | Scene Management | 02, 06 | M |
| **3** | [09](./09_live2d_integration.md) | Live2D Integration | 02 | XL |
| **3** | [10](./10_vrm_integration.md) | VRM/FBX Integration | 02 | XL |
| **4** | [11](./11_ui_system.md) | UI System (MVVM) | 02 | XL |
| **4** | [12](./12_save_system.md) | Save System | 01 | M |
| **5** | [13](./13_script_editor.md) | Script Editor | 07, 11 | XL |
| **5** | [14](./14_game_player.md) | Game Player | 08, 12 | M |
| **X** | [15](./15_phase2_readiness_plan.md) | Phase 2 Readiness Plan | 02, 03, 04, 05 | M |

> **規模目安**: S=1-3日, M=3-7日, L=1-2週, XL=2-4週

## 開発方針

- Phase 0 → 1 → 2 は順次実装（基盤から積み上げ）
- Phase 1 内の 02〜05 は**並行開発可能**
- Phase 3 の Live2D/VRM は Renderer 完成後に開始
- Phase 4 の UI/Save は **並行開発可能**
- Phase 5 は全コア機能完成後に着手


## 補助プラン

- [15](./15_phase2_readiness_plan.md): Phase 2 着手前の改善項目を WBS 化した実行計画（英語版）。
