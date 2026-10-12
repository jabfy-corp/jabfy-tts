# jabfy-tts

Kokoro TTS crate for jabfy-core. Converts Ollama AI text output to speech using [kokoroxide](https://crates.io/crates/kokoroxide), a Rust implementation of Kokoro TTS — runs fully locally, no API key needed.

## Install

Add to your `Cargo.toml`:

```toml
[dependencies]
jabfy-tts = "0.1"
```

### Prerequisites

- `espeak-ng` (used by kokoroxide for text-to-phoneme conversion) — see [kokoroxide's system requirements](https://docs.rs/kokoroxide) for platform-specific install instructions.
- Kokoro model assets, downloaded separately (not bundled): the ONNX model, `tokenizer.json`, and a voice style `.bin` file from [Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX).

### Windows notes

The official `espeak-ng` Windows installer (`winget install eSpeak-NG.eSpeak-NG`) ships `libespeak-ng.dll` but no MSVC import library, and kokoroxide's build script only wires up linking for macOS/Linux. This repo includes a pre-generated `vendor/espeak-ng/espeak-ng.lib` (built from that DLL's exports via `dumpbin`/`lib.exe`) and a [.cargo/config.toml](.cargo/config.toml) that adds it to the linker search path. Make sure `espeak-ng`'s install directory (e.g. `C:\Program Files\eSpeak NG`) is on `PATH` at runtime so `libespeak-ng.dll` can be found.

## Usage

### One-shot synthesis

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsOptions};

let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_heart.bin");
let mut tts = JabfyTts::create(&assets)?;

let audio = tts.synthesize("Hello from jabfy!", TtsOptions::default())?;
audio.save_to_wav("output.wav")?;
```

### Streaming (for Ollama token-by-token output)

kokoroxide has no built-in incremental API, so `JabfyTts::stream` buffers
pushed tokens and synthesizes audio once a sentence boundary (`.`, `!`, `?`,
or newline) is seen.

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsOptions};

let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_bella.bin");
let mut tts = JabfyTts::create(&assets)?;
let mut stream = tts.stream(TtsOptions::default());

// Feed Ollama tokens as they arrive
for token in ollama_tokens {
    for chunk in stream.push(&token)? {
        chunk.audio.save_to_wav(&format!("{}.wav", chunk.text))?;
    }
}

// Flush whatever's left when the LLM stream ends
for chunk in stream.close()? {
    chunk.audio.save_to_wav(&format!("{}.wav", chunk.text))?;
}
```

### Options

| Option  | Type    | Default | Description                    |
|---------|---------|---------|---------------------------------|
| `speed` | `f32`   | `1.0`   | Playback speed multiplier      |

Model-level knobs (`sample_rate`, `max_tokens_length`) are set via `TtsConfig` and `JabfyTts::with_config`.

## API

### `JabfyTts::create(assets)`

Loads the model and a voice style with default config. Call once and reuse.

### `JabfyTts::with_config(assets, config)`

Loads the model with explicit `TtsConfig` knobs (sample rate, max token length).

### `tts.synthesize(text, options)`

Generates audio for a complete string. Returns kokoroxide's `GeneratedAudio`, with `.save_to_wav(path)`, `.duration_seconds`, and `.sample_rate`.

### `tts.stream(options)`

Returns a `TtsStream` with:

| Method  | Description                                                    |
|---------|------------------------------------------------------------------|
| `push`  | Feed a token into the stream; returns audio for completed sentences |
| `flush` | Emit audio for buffered text without closing the stream          |
| `close` | Flush remaining buffered text; call when input ends              |

Each returned `TtsStreamChunk` contains `text` and `audio` (`GeneratedAudio`).

## Publishing

Creating a GitHub Release triggers the publish workflow automatically. The workflow builds the crate and publishes it to crates.io using `CARGO_REGISTRY_TOKEN`.

See the [wiki](../../wiki) for full documentation including the streaming guide, voice options, and troubleshooting.

