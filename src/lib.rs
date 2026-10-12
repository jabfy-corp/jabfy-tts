//! jabfy-tts — a Kokoro TTS crate for jabfy-core, built on the [`kokoroxide`]
//! Rust implementation of Kokoro TTS.

mod tts;
mod types;

pub use kokoroxide::{GeneratedAudio, GraphOptimizationLevel, VoiceStyle};
pub use tts::{JabfyTts, TtsStream, TtsStreamChunk};
pub use types::{TtsAssets, TtsConfig, TtsOptions};
