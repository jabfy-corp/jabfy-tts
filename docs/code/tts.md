# `src/tts.rs`

> **Rôle** — `JabfyTts`, surcouche ergonomique de kokoroxide, et `TtsStream`, le pseudo-streaming par phrase.

Dépôt : [jabfy-tts](README.md) · 139 lignes · statut : **actif**

Doc comment de `JabfyTts` : *JabfyTts — thin wrapper around kokoroxide for use with jabfy-core.*

## À retenir

- `with_config` l.28 : construit le `TTSConfig` de kokoroxide depuis `TtsAssets` + `TtsConfig`, charge le moteur `KokoroTTS` (l.36) puis le `VoiceStyle` une seule fois (l.37).
- `create` l.23 délègue à `with_config` avec `TtsConfig::default()` (24 kHz, 512 tokens).
- `synthesize` l.43 → `engine.generate_speech(text, &voice, options.speed)` : synchrone et bloquant.
- `stream` l.55 renvoie `TtsStream<'_>` qui emprunte `&mut self` : `synthesize` est inutilisable pendant la vie du flux.
- `push` l.83 concatène au tampon puis `drain_sentences(false)` ; `flush` l.89 et `close` l.95 appellent `drain_sentences(true)` — `close` est un alias de `flush`.
- `next_boundary` l.133 : index juste après le premier `.`, `!`, `?` ou `\n` du tampon, sûr en UTF-8 grâce à `char_indices`.
- `synthesize_if_nonempty` l.121 saute les segments tout en blancs ; `drain_sentences` l.98 peut émettre plusieurs chunks par appel.

## Contenu (ligne → élément)

| ligne | élément | note |
|---|---|---|
| 16 | struct `JabfyTts` | Moteur `KokoroTTS` + `VoiceStyle`. — méthodes : `create` 23, `with_config` 28, `synthesize` 43, `stream` 55 |
| 65 | struct `TtsStreamChunk` | `text: String` + `audio: GeneratedAudio` d'une phrase. |
| 75 | struct `TtsStream<'a>` | `&'a mut JabfyTts` + options + tampon. — méthodes : `push` 83, `flush` 89, `close` 95, `drain_sentences` 98, `synthesize_if_nonempty` 121, `next_boundary` 133 |

## Relations

- **Utilise** : [src/types.rs](types.md) ; kokoroxide (`KokoroTTS`, `TTSConfig`, `load_voice_style`, `GeneratedAudio`, `VoiceStyle`)
- **Utilisé par** : [src/lib.rs](lib.md)
- **Testé par** : aucun test automatisé pour l'instant — voir [tests de jabfy-tts](../tests.md)
