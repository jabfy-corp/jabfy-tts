# Voix

jabfy-tts utilise le modèle Kokoro-82M, dont les voix sont distribuées en fichiers `.bin` de style vocal. Une voix se choisit par chemin de fichier, pas par identifiant texte.

## Choisir une voix

Le chemin du fichier voix est le troisième argument de `TtsAssets::new` :

```rust
let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_bella.bin");
let mut tts = JabfyTts::create(&assets)?;
```

La voix est fixée par instance : elle est chargée à la construction et s'applique à tous les `synthesize` et `stream` de cette instance. Il n'y a pas de changement de voix par appel — créer une instance par voix :

```rust
let mut bella = JabfyTts::create(&TtsAssets::new("model.onnx", "tokenizer.json", "af_bella.bin"))?;
let mut sky = JabfyTts::create(&TtsAssets::new("model.onnx", "tokenizer.json", "af_sky.bin"))?;
```

## Obtenir les fichiers de voix

Les fichiers de voix ne sont pas embarqués dans la crate. Ils se téléchargent dans le répertoire `voices/` du dépôt Hugging Face [Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX) — un `.bin` par voix, par ex. `af_heart.bin`, `af_bella.bin`, `am_michael.bin`.

`af_heart` est un choix par défaut courant.

## Convention de nommage

- Première lettre : langue/accent — `a` américain, `b` britannique
- Deuxième lettre : genre — `f` féminin, `m` masculin
- Suffixe : nom de la voix — `heart`, `bella`, `sky`, `michael`, etc.

Ainsi `af_bella` est une voix féminine américaine et `bm_george` une voix masculine britannique.

La liste complète et à jour est sur la [page du modèle Kokoro-82M](https://huggingface.co/hexgrad/Kokoro-82M).

## `VoiceStyle`

jabfy-tts ré-exporte le type `VoiceStyle` de kokoroxide. Inutile en usage normal — `TtsAssets::voice_path` est chargé en interne via `kokoroxide::load_voice_style` — mais disponible si vous interagissez directement avec kokoroxide.
