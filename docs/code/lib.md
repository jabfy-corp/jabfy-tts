# `src/lib.rs`

> **Rôle** — Racine du crate : déclare les modules et ré-exporte toute l'API publique.

Dépôt : [jabfy-tts](README.md) · 9 lignes · statut : **actif**

Docstring du module : *jabfy-tts — a Kokoro TTS crate for jabfy-core, built on the kokoroxide Rust implementation of Kokoro TTS.*

## À retenir

- Aucun item défini ici : la surface publique n'est que des ré-exports (l.7-9).
- `GeneratedAudio`, `GraphOptimizationLevel` et `VoiceStyle` sont ré-exportés depuis kokoroxide (l.7) : le consommateur n'a pas besoin d'y dépendre directement.

## Contenu (ligne → élément)

| ligne | élément | note |
|---|---|---|
| 4 | `mod tts` | Moteur et flux : `JabfyTts`, `TtsStream`, `TtsStreamChunk`. |
| 5 | `mod types` | Configuration : `TtsAssets`, `TtsOptions`, `TtsConfig`. |
| 7 | `pub use kokoroxide::…` | `GeneratedAudio`, `GraphOptimizationLevel`, `VoiceStyle`. |
| 8 | `pub use tts::…` | API principale : `JabfyTts`, `TtsStream`, `TtsStreamChunk`. |
| 9 | `pub use types::…` | `TtsAssets`, `TtsConfig`, `TtsOptions`. |

## Relations

- **Utilise** : [src/tts.rs](tts.md), [src/types.rs](types.md), kokoroxide (externe)
- **Utilisé par** : les crates consommatrices (jabfy-core à terme)
- **Testé par** : aucun test automatisé pour l'instant — voir [tests de jabfy-tts](../tests.md)
