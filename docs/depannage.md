# Dépannage

## Le build échoue sur des erreurs espeak-ng / clang

kokoroxide se lie à `espeak-ng` et utilise `clang-sys` à la compilation. En cas d'erreur de linker ou de bindgen, installer espeak-ng avec ses en-têtes de développement :

| Plateforme | Commande |
|---|---|
| Ubuntu/Debian | `sudo apt-get install espeak-ng libespeak-ng-dev` |
| macOS | `brew install espeak-ng` |
| Windows | `winget install eSpeak-NG.eSpeak-NG` |

### Liaison sous Windows

L'installeur Windows officiel fournit `libespeak-ng.dll` mais pas de bibliothèque d'import MSVC, et le build de kokoroxide ne configure la liaison que sur macOS/Linux. Ce dépôt contourne le problème avec `vendor/espeak-ng/espeak-ng.lib` et le `-L` de `.cargo/config.toml`. Si vous consommez jabfy-tts depuis un autre crate sous Windows et voyez des erreurs `unresolved external` / `espeak-ng.lib` manquant, générez une bibliothèque d'import depuis la DLL (`dumpbin /exports` + `lib /def:`) et ajoutez son répertoire au chemin de recherche du linker de la même façon.

### Windows à l'exécution : `libespeak-ng.dll` introuvable

Ajouter le répertoire d'installation d'espeak-ng (par ex. `C:\Program Files\eSpeak NG`) au `PATH`, ou copier `libespeak-ng.dll` à côté de l'exécutable.

## `JabfyTts::create` renvoie une erreur

`create` échoue si l'un des trois chemins de `TtsAssets` est faux ou si un fichier est corrompu. Vérifier :

- `model_path` pointe sur le modèle ONNX Kokoro (`.onnx`)
- `tokenizer_path` pointe sur `tokenizer.json`
- `voice_path` pointe sur un fichier voix `.bin`

Les trois viennent du dépôt Hugging Face [Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX) — ils ne sont pas téléchargés automatiquement.

## `stream.push()` ne renvoie aucun chunk

C'est attendu : `push` n'émet de l'audio que lorsque le tampon franchit une frontière de phrase (`.`, `!`, `?` ou `\n`). Sans ponctuation dans les tokens, on obtient un `Vec` vide.

- Toujours appeler `stream.close()` (ou `flush()`) en fin d'entrée — le texte tamponné sans frontière finale n'est sinon jamais synthétisé.
- Si le LLM produit de longs passages sans ponctuation, appeler `flush()` périodiquement pour borner la latence.

## Audio absent ou corrompu

- Itérer tous les chunks renvoyés par `push`, `flush` et `close` — chacun renvoie un `Vec`, et le laisser tomber jette l'audio silencieusement.
- Vérifier que le chemin passé à `save_to_wav` est inscriptible.
- Les segments tout en blancs sont volontairement sautés et ne produisent aucun chunk.

## Synthèse trop lente ou trop rapide

Ajuster `TtsOptions::speed` — au-dessus de `1.0` accélère, en dessous ralentit :

```rust
let options = TtsOptions { speed: 1.2 };
let audio = tts.synthesize("Hello!", options)?;
```

## La résolution des dépendances échoue sur kokoroxide / ort

jabfy-tts dépend actuellement d'un fork git de kokoroxide (branche `ort_v2_migration`) : toutes les releases `ort 1.16.x` requises par la version crates.io ont été retirées (yanked). Cargo doit pouvoir joindre `github.com` pour le récupérer ; derrière un proxy, configurer `HTTPS_PROXY` ou un `[source]` de remplacement dans `.cargo/config.toml`. Contexte dans les commentaires de `Cargo.toml`.

## Échecs de lint ou de format en CI

La CI lance `cargo fmt --check` et `cargo clippy --all-targets -- -D warnings`. Les lancer localement avant de pousser :

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
```
