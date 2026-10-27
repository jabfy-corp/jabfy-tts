# Tests

## État des tests

Le dépôt ne contient pas encore de tests automatisés — c'est dans les [TODO](README.md). La vérification repose sur le formatage, le lint et le build release.

## Vérifications locales

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Le build requiert espeak-ng et ses en-têtes de développement — voir [Installation](installation.md). Les assets du modèle ne sont pas nécessaires pour compiler, seulement pour exécuter une synthèse.

## Vérification manuelle de bout en bout

Sans tests automatisés, un essai manuel avec les assets téléchargés reste le moyen de valider une synthèse :

```rust
let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_heart.bin");
let mut tts = JabfyTts::create(&assets)?;
tts.synthesize("test", TtsOptions::default())?.save_to_wav("test.wav")?;
```

## CI

Le workflow `.github/workflows/ci.yml` tourne à chaque pull request vers `main`, en deux jobs sur Ubuntu :

1. `quality` : `cargo fmt --check` puis `cargo clippy --all-targets -- -D warnings` ;
2. `build` : `cargo build --release` (ne tourne que si `quality` passe).

Les deux installent `espeak-ng` et `libespeak-ng-dev` via apt avant de compiler.

## Publication

Le workflow `.github/workflows/publish.yml` se déclenche à la création d'une release GitHub : build release, puis `cargo publish --token $CARGO_REGISTRY_TOKEN` vers crates.io.

> Bloqué aujourd'hui par la dépendance git au fork de kokoroxide — voir les [limites connues](README.md).
