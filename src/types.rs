//! Configuration and option types for [`crate::JabfyTts`].

/// Paths to the on-disk assets kokoroxide requires: the ONNX model, the
/// tokenizer config, and a voice style file. These are not bundled with the
/// crate — download them from the Kokoro model distribution and point at
/// your local copies.
#[derive(Clone, Debug)]
pub struct TtsAssets {
    pub model_path: String,
    pub tokenizer_path: String,
    pub voice_path: String,
}

impl TtsAssets {
    pub fn new(
        model_path: impl Into<String>,
        tokenizer_path: impl Into<String>,
        voice_path: impl Into<String>,
    ) -> Self {
        Self {
            model_path: model_path.into(),
            tokenizer_path: tokenizer_path.into(),
            voice_path: voice_path.into(),
        }
    }
}

/// Per-call synthesis options.
#[derive(Clone, Copy, Debug)]
pub struct TtsOptions {
    /// Playback speed multiplier. Defaults to 1.0.
    pub speed: f32,
}

impl Default for TtsOptions {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

/// Model-level configuration, mirrors the knobs on `kokoroxide::TTSConfig`.
#[derive(Clone, Debug)]
pub struct TtsConfig {
    pub sample_rate: u32,
    pub max_tokens_length: usize,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            sample_rate: 24_000,
            max_tokens_length: 512,
        }
    }
}
