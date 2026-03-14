# 03: Audio System — サウンドシステム

## 概要
`kira` クレートをベースに BGM / SE / Voice のミキシング、クロスフェード、ループ制御を実装する。音声 RMS 解析による Live2D リップシンク連携も本モジュールで提供する。

## 依存先
- `01_ecs_foundation`

## モジュール構成
```
crates/engine_core/src/audio/
├── mod.rs          # 公開 API
├── manager.rs      # AudioManager (kira ラッパー)
├── bgm.rs          # BGM トラック制御
├── se.rs           # SE 再生管理
├── voice.rs        # Voice 再生 + リップシンク
└── lip_sync.rs     # RMS 解析
```

## タスク

### 1. AudioManager
```rust
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::track::TrackBuilder;

pub struct GameAudioManager {
    manager: AudioManager,
    bgm_track: TrackHandle,
    se_track: TrackHandle,
    voice_track: TrackHandle,
}
```

- kira の `AudioManager` 初期化
- BGM / SE / Voice 用の 3 トラック構成
- ボリューム個別制御（`Decibels` + `Tween`）

### 2. BGM 制御
- ファイルからの BGM ロード (`StaticSoundData::from_file`)
- ループ再生（`loop_start` / `loop_end` 区間指定）
- クロスフェード遷移（`Tween` によるスムーズ切替）
- 一時停止 / 再開

### 3. SE 制御
- 重複再生対応（同一 SE を複数同時再生）
- ワンショット再生
- ボリューム / パン制御

### 4. Voice 再生 + リップシンク
- Voice 再生開始時にリップシンク解析を連動
- Audio RMS（実効値）をリアルタイム計算

```rust
pub fn calculate_lip_sync_value(audio_samples: &[f32], frame_size: usize) -> f32 {
    let rms = (audio_samples.iter()
        .take(frame_size)
        .map(|s| s * s)
        .sum::<f32>() / frame_size as f32)
        .sqrt();
    (rms * 10.0).clamp(0.0, 1.0)
}
```

### 5. CSV メタデータ連携
- `sound.csv` から `SoundMeta` をデシリアライズ
- `volume` (f32 0.0〜1.0) を kira の `Decibels` に変換
- `loop_start` / `loop_end` でループ区間設定

### 6. ECS 連携
```rust
pub struct AudioSource {
    pub sound_id: SoundId,
    pub volume: f32,
    pub playing: bool,
}

pub enum AudioCommand {
    PlayBgm(SoundId),
    StopBgm { fade_ms: u64 },
    PlaySe(SoundId),
    PlayVoice { voice_id: String, lip_sync: bool },
    SetVolume { track: AudioTrack, volume: f32 },
}
```

## テスト計画
- AudioManager 初期化テスト
- BGM ロード / 再生 / 停止テスト
- RMS リップシンク値計算の数値テスト

## 完了条件
- BGM / SE / Voice の個別再生・停止
- BGM クロスフェード遷移
- リップシンク値のリアルタイム取得
