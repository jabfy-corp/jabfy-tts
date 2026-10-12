use kokoroxide::{load_voice_style, GeneratedAudio, KokoroTTS, TTSConfig, VoiceStyle};

use crate::types::{TtsAssets, TtsConfig as JabfyTtsConfig, TtsOptions};

/// JabfyTts — thin wrapper around kokoroxide for use with jabfy-core.
///
/// Usage:
///   let tts = JabfyTts::create(&assets)?;
///   let audio = tts.synthesize("Hello world", TtsOptions::default())?;
///   audio.save_to_wav("output.wav")?;
///
/// Streaming (ideal for Ollama token-by-token output):
///   let mut stream = tts.stream(TtsOptions::default());
///   for chunk in stream.push("Hello ")? { chunk.audio.save_to_wav(...)?; }
///   for chunk in stream.close()? { chunk.audio.save_to_wav(...)?; }
pub struct JabfyTts {
    engine: KokoroTTS,
    voice: VoiceStyle,
}

impl JabfyTts {
    /// Load the Kokoro model and a voice style with default config. Call once and reuse.
    pub fn create(assets: &TtsAssets) -> Result<Self, Box<dyn std::error::Error>> {
        Self::with_config(assets, &JabfyTtsConfig::default())
    }

    /// Load the model with explicit config knobs (sample rate, max token length).
    pub fn with_config(
        assets: &TtsAssets,
        config: &JabfyTtsConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tts_config = TTSConfig::new(&assets.model_path, &assets.tokenizer_path)
            .with_sample_rate(config.sample_rate)
            .with_max_tokens_length(config.max_tokens_length);

        let engine = KokoroTTS::with_config(tts_config)?;
        let voice = load_voice_style(&assets.voice_path)?;

        Ok(Self { engine, voice })
    }

    /// Generate audio for a complete text string.
    pub fn synthesize(
        &mut self,
        text: &str,
        options: TtsOptions,
    ) -> Result<GeneratedAudio, Box<dyn std::error::Error>> {
        Ok(self
            .engine
            .generate_speech(text, &self.voice, options.speed)?)
    }

    /// Start a streaming session — feed tokens as they arrive and get back
    /// audio for each completed sentence.
    pub fn stream(&mut self, options: TtsOptions) -> TtsStream<'_> {
        TtsStream {
            tts: self,
            options,
            buffer: String::new(),
        }
    }
}

/// One completed sentence's synthesized audio, produced while streaming.
pub struct TtsStreamChunk {
    pub text: String,
    pub audio: GeneratedAudio,
}

/// Buffers incoming text and synthesizes audio one sentence at a time.
///
/// kokoroxide has no built-in incremental/streaming API, so this buffers
/// pushed tokens and calls `synthesize` once a sentence boundary (`.`, `!`,
/// `?`, or newline) is seen.
pub struct TtsStream<'a> {
    tts: &'a mut JabfyTts,
    options: TtsOptions,
    buffer: String,
}

impl<'a> TtsStream<'a> {
    /// Feed a token/word from the LLM. Returns audio for any newly completed sentences.
    pub fn push(&mut self, token: &str) -> Result<Vec<TtsStreamChunk>, Box<dyn std::error::Error>> {
        self.buffer.push_str(token);
        self.drain_sentences(false)
    }

    /// Emit audio for whatever text is buffered, without waiting for a sentence boundary.
    pub fn flush(&mut self) -> Result<Vec<TtsStreamChunk>, Box<dyn std::error::Error>> {
        self.drain_sentences(true)
    }

    /// Signal end of input. Alias for `flush`.
    pub fn close(&mut self) -> Result<Vec<TtsStreamChunk>, Box<dyn std::error::Error>> {
        self.flush()
    }

    fn drain_sentences(
        &mut self,
        force: bool,
    ) -> Result<Vec<TtsStreamChunk>, Box<dyn std::error::Error>> {
        let mut chunks = Vec::new();

        while let Some(boundary) = self.next_boundary() {
            let sentence: String = self.buffer.drain(..boundary).collect();
            if let Some(chunk) = self.synthesize_if_nonempty(sentence)? {
                chunks.push(chunk);
            }
        }

        if force && !self.buffer.trim().is_empty() {
            let sentence = std::mem::take(&mut self.buffer);
            if let Some(chunk) = self.synthesize_if_nonempty(sentence)? {
                chunks.push(chunk);
            }
        }

        Ok(chunks)
    }

    fn synthesize_if_nonempty(
        &mut self,
        text: String,
    ) -> Result<Option<TtsStreamChunk>, Box<dyn std::error::Error>> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        let audio = self.tts.synthesize(&text, self.options)?;
        Ok(Some(TtsStreamChunk { text, audio }))
    }

    /// Index just past the first sentence-ending punctuation/newline in the buffer, if any.
    fn next_boundary(&self) -> Option<usize> {
        self.buffer
            .char_indices()
            .find(|(_, c)| matches!(c, '.' | '!' | '?' | '\n'))
            .map(|(i, c)| i + c.len_utf8())
    }
}
