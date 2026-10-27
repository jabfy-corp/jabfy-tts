# Interfaces

jabfy-tts est une bibliothèque : son interface est son API publique Rust, ré-exportée depuis `src/lib.rs`. Toutes les API fallibles renvoient `Result<_, Box<dyn std::error::Error>>`.

## `JabfyTts`

Le type principal. Une seule instance, réutilisée — le chargement du modèle est coûteux.

| Méthode | Signature | Rôle |
|---|---|---|
| `create` | `fn create(assets: &TtsAssets) -> Result<Self, _>` | Charge le modèle et la voix avec `TtsConfig::default()` |
| `with_config` | `fn with_config(assets: &TtsAssets, config: &TtsConfig) -> Result<Self, _>` | Charge avec des réglages explicites (sample rate, tokens max) |
| `synthesize` | `fn synthesize(&mut self, text: &str, options: TtsOptions) -> Result<GeneratedAudio, _>` | Synthétise une chaîne complète, bloquant |
| `stream` | `fn stream(&mut self, options: TtsOptions) -> TtsStream<'_>` | Démarre une session de streaming |

## `TtsStream`

Renvoyé par `stream`. Tamponne le texte poussé et synthétise une phrase à la fois. Frontières : `.`, `!`, `?`, `\n`.

| Méthode | Signature | Rôle |
|---|---|---|
| `push` | `fn push(&mut self, token: &str) -> Result<Vec<TtsStreamChunk>, _>` | Ajoute un token ; renvoie l'audio des phrases nouvellement terminées |
| `flush` | `fn flush(&mut self) -> Result<Vec<TtsStreamChunk>, _>` | Synthétise le tampon courant sans fermer le flux |
| `close` | `fn close(&mut self) -> Result<Vec<TtsStreamChunk>, _>` | Fin d'entrée — alias de `flush` |

Chaque méthode renvoie un `Vec` possiblement vide — vide tant qu'aucune frontière n'est atteinte.

## `TtsStreamChunk`

L'audio d'une phrase terminée, produit en streaming.

| Champ | Type | Rôle |
|---|---|---|
| `text` | `String` | Le segment de texte correspondant au chunk |
| `audio` | `GeneratedAudio` | L'audio synthétisé |

## `TtsAssets`

Chemins des assets requis par kokoroxide. Rien n'est embarqué dans la crate.

| Champ | Type | Rôle |
|---|---|---|
| `model_path` | `String` | Chemin du modèle ONNX Kokoro |
| `tokenizer_path` | `String` | Chemin de `tokenizer.json` |
| `voice_path` | `String` | Chemin du fichier voix `.bin` |

Constructeur : `TtsAssets::new(model_path, tokenizer_path, voice_path)`.

## `TtsOptions`

Options par appel. `Copy`, avec `Default`.

| Champ | Type | Défaut | Rôle |
|---|---|---|---|
| `speed` | `f32` | `1.0` | Multiplicateur de vitesse (> 1 = plus rapide) |

## `TtsConfig`

Réglages du modèle, miroir de `kokoroxide::TTSConfig`. Passé à `with_config`.

| Champ | Type | Défaut | Rôle |
|---|---|---|---|
| `sample_rate` | `u32` | `24000` | Fréquence d'échantillonnage de sortie, en Hz |
| `max_tokens_length` | `usize` | `512` | Longueur maximale d'entrée en tokens |

## `GeneratedAudio`

Ré-exporté depuis kokoroxide. Le résultat d'une synthèse.

| Champ | Type | Rôle |
|---|---|---|
| `samples` | `Vec<f32>` | Samples PCM bruts |
| `sample_rate` | `u32` | Fréquence d'échantillonnage |
| `duration_seconds` | `f32` | Durée de l'audio |

| Méthode | Rôle |
|---|---|
| `save_to_wav(path)` | Écrit l'audio dans un fichier `.wav` |
| `to_wav_bytes()` | Encode l'audio en WAV en mémoire (`Vec<u8>`) |

## Ré-exports

```rust
pub use kokoroxide::{GeneratedAudio, GraphOptimizationLevel, VoiceStyle};
```

`VoiceStyle` est la voix chargée ; `GraphOptimizationLevel` contrôle les optimisations de graphe d'ONNX Runtime, pour les usages avancés.

## Exemple complet

### Synthèse ponctuelle

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsOptions};

let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_heart.bin");
let mut tts = JabfyTts::create(&assets)?;
let audio = tts.synthesize("Hello from jabfy!", TtsOptions::default())?;
audio.save_to_wav("output.wav")?;
```

### Streaming sur la sortie d'Ollama

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsOptions};

let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_bella.bin");
let mut tts = JabfyTts::create(&assets)?;
let mut stream = tts.stream(TtsOptions::default());

for token in ollama_tokens {
    for chunk in stream.push(&token)? {
        chunk.audio.save_to_wav(&format!("{}.wav", chunk.text))?;
    }
}
for chunk in stream.close()? {
    chunk.audio.save_to_wav(&format!("{}.wav", chunk.text))?;
}
```

### Accès aux samples bruts

Pour alimenter un haut-parleur ou un pipeline audio plutôt que des fichiers :

```rust
for chunk in stream.push(token)? {
    let audio = chunk.audio;
    // audio.samples : Vec<f32> de samples PCM
    // audio.sample_rate : typiquement 24000
    your_sink.write(&audio.samples, audio.sample_rate);
}
```

`audio.to_wav_bytes()` donne un encodage WAV en mémoire si besoin.
