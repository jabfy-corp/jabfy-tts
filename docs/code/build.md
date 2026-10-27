# jabfy-tts — build et CI

| fichier | ce qu'il fait |
|---|---|
| `Cargo.toml` | Crate `jabfy-tts` 0.1.0 (édition 2021, MIT) ; dépendance au fork git de kokoroxide (branche `ort_v2_migration`) et `ort` 2.0.0-rc.13 avec les features `coreml` + `lax-feature-matching` — voir les commentaires du fichier. |
| `Cargo.lock` | Lockfile : fige le fork de kokoroxide et tout l'arbre `ort`. |
| `.cargo/config.toml` | `-L vendor/espeak-ng` dans les rustflags MSVC Windows : ajoute la bibliothèque d'import au chemin du linker. |
| `vendor/espeak-ng/espeak-ng.lib` | Bibliothèque d'import générée depuis les exports de `libespeak-ng.dll` (`dumpbin` + `lib.exe`), absente de l'installeur Windows officiel. |
| `.github/workflows/ci.yml` | Sur PR vers `main` : `cargo fmt --check` et `clippy -D warnings` (job `quality`), puis `cargo build --release` (job `build`), Ubuntu + espeak-ng. |
| `.github/workflows/publish.yml` | Sur release GitHub : build release puis `cargo publish --token CARGO_REGISTRY_TOKEN` vers crates.io. |

À noter : `cargo publish` refuse les dépendances git sans champ `version` — la publication crates.io est donc bloquée tant que le fork de kokoroxide reste une dépendance git. Retour prévu à la version crates.io après le merge de la PR upstream (voir [limites connues](../README.md)).
