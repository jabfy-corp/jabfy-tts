# Installation et lancement

## Prérequis

- Une toolchain Rust stable (`rustup`)
- `espeak-ng` — utilisé par kokoroxide pour la conversion texte → phonèmes
- Les assets du modèle Kokoro, téléchargés à part (rien n'est embarqué dans la crate)

## Installer espeak-ng

| Plateforme | Commande |
|---|---|
| Ubuntu/Debian | `sudo apt-get install espeak-ng libespeak-ng-dev` |
| macOS | `brew install espeak-ng` |
| Windows | `winget install eSpeak-NG.eSpeak-NG` |

Le paquet de développement (`libespeak-ng-dev` ou équivalent) est requis à la compilation ; la bibliothèque partagée l'est à l'exécution.

### Windows

L'installeur officiel d'espeak-ng fournit `libespeak-ng.dll` mais aucune bibliothèque d'import MSVC, et le script de build de kokoroxide ne configure la liaison que sur macOS/Linux. Ce dépôt embarque donc `vendor/espeak-ng/espeak-ng.lib` (générée depuis les exports de la DLL via `dumpbin`/`lib.exe`) et un `.cargo/config.toml` qui l'ajoute au chemin de recherche du linker : `cargo build` fonctionne tel quel **dans ce dépôt**.

À l'exécution, le répertoire d'installation d'espeak-ng (par ex. `C:\Program Files\eSpeak NG`) doit être dans `PATH` pour que `libespeak-ng.dll` soit trouvée. Pour un crate consommateur hors de ce dépôt, voir [Dépannage](depannage.md).

## Télécharger les assets du modèle

`TtsAssets` attend trois fichiers, tous disponibles dans le dépôt Hugging Face [Kokoro-82M-v1.0-ONNX](https://huggingface.co/onnx-community/Kokoro-82M-v1.0-ONNX) :

| Asset | Fichier | Rôle |
|---|---|---|
| Modèle | `model.onnx` (ou une variante quantifiée) | Le modèle Kokoro-82M en ONNX |
| Tokenizer | `tokenizer.json` | Configuration du tokenizer |
| Voix | par ex. `af_heart.bin` | Embedding de style de voix (répertoire `voices/`) |

Voir [Voix](voix.md) pour la liste et la convention de nommage des fichiers de voix.

## Installation

```bash
cargo add jabfy-tts
```

Ou dans `Cargo.toml` :

```toml
[dependencies]
jabfy-tts = "0.1"
```

## Première synthèse

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assets = TtsAssets::new("model.onnx", "tokenizer.json", "af_heart.bin");
    let mut tts = JabfyTts::create(&assets)?; // chargement coûteux : une seule fois
    let audio = tts.synthesize("Hello from jabfy!", TtsOptions::default())?;
    audio.save_to_wav("output.wav")?;
    Ok(())
}
```

Toutes les API fallibles de jabfy-tts renvoient `Result<_, Box<dyn std::error::Error>>`.

## Configuration

`JabfyTts::create` utilise `TtsConfig::default()` (24 kHz, 512 tokens max). `with_config` permet de les changer :

```rust
use jabfy_tts::{JabfyTts, TtsAssets, TtsConfig};

let config = TtsConfig {
    sample_rate: 24_000,
    max_tokens_length: 1024,
};
let mut tts = JabfyTts::with_config(&assets, &config)?;
```

La voix se choisit par le fichier `.bin` passé dans `TtsAssets` — il n'y a pas d'option de voix par appel. Créer une instance `JabfyTts` par voix nécessaire.

## Commandes de développement

| Commande | Effet |
|---|---|
| `cargo build --release` | Compile la crate (ce que la CI vérifie) |
| `cargo fmt --check` | Vérifie le formatage (CI) |
| `cargo clippy --all-targets -- -D warnings` | Lint strict (CI) |

## Publication

La création d'une release GitHub déclenche `.github/workflows/publish.yml` : build release puis `cargo publish --token $CARGO_REGISTRY_TOKEN` vers crates.io. Les tags suivent les conventions de `CONTRIBUTING.md` (`Year.MonthDay.Version`).

> Aujourd'hui, cette publication échouerait : `cargo publish` refuse la dépendance git au fork de kokoroxide. Retour à la version crates.io dès que possible — voir les [limites connues](README.md).
