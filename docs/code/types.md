# `src/types.rs`

> **Rôle** — Types de configuration : chemins d'assets, options par appel, réglages du modèle.

Dépôt : [jabfy-tts](README.md) · 55 lignes · statut : **actif**

Docstring du module : *Configuration and option types for [`crate::JabfyTts`].*

## À retenir

- `TtsAssets` l.8 : trois chemins `String` — rien n'est embarqué dans la crate, tout pointe sur des fichiers locaux.
- `TtsOptions` l.30 : `Copy` ; seul réglage par appel, `speed` (défaut `1.0`, impl `Default` l.35).
- `TtsConfig` l.43 : miroir de `kokoroxide::TTSConfig` ; défauts `sample_rate` 24000 et `max_tokens_length` 512 (impl `Default` l.48).

## Contenu (ligne → élément)

| ligne | élément | note |
|---|---|---|
| 8 | struct `TtsAssets` | `model_path`, `tokenizer_path`, `voice_path`. — méthode : `new` 15 |
| 30 | struct `TtsOptions` | `speed: f32`. — impl : `Default` 35 |
| 43 | struct `TtsConfig` | `sample_rate: u32`, `max_tokens_length: usize`. — impl : `Default` 48 |

## Relations

- **Utilise** : aucun fichier du dépôt (bibliothèque standard uniquement)
- **Utilisé par** : [src/tts.rs](tts.md), [src/lib.rs](lib.md)
- **Testé par** : aucun test automatisé pour l'instant — voir [tests de jabfy-tts](../tests.md)
