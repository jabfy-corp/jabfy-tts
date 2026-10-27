# Fonctionnement interne

## Vue d'ensemble

```text
Texte entrant (token LLM ou chaîne complète)
        ↓
TtsStream                (src/tts.rs) — tampon, frontières de phrase   [flux seulement]
        ↓
JabfyTts::synthesize     (src/tts.rs) — un appel par segment
        ↓
kokoroxide::KokoroTTS    tokenisation → phonèmes (espeak-ng) → inférence ONNX
        ↓
GeneratedAudio           samples f32, 24 kHz par défaut
```

## Règles d'implémentation

1. **Une voix par instance.** Le `VoiceStyle` est chargé dans `with_config` à la construction et s'applique à tous les appels ; il n'existe pas d'option de voix par appel. Pour plusieurs voix, créer plusieurs instances.
2. **Le streaming est une façade.** kokoroxide n'a pas d'API incrémentale : `TtsStream` tamponne le texte poussé et appelle `synthesize` dès qu'une frontière de phrase (`.`, `!`, `?` ou `\n`) apparaît dans le tampon.
3. **Tout est synchrone.** `push`, `flush` et `close` renvoient directement un `Vec<TtsStreamChunk>` : un vec vide signifie qu'aucune phrase n'est encore terminée. Il n'y a ni async ni itérateur de sortie séparé.
4. **Un seul flux par instance.** `TtsStream` emprunte `&mut JabfyTts` pour toute sa durée de vie : impossible d'appeler `synthesize` pendant qu'un flux existe. Pour des flux concurrents, créer plusieurs instances.
5. **Les segments blancs ne produisent rien.** Un segment qui ne contient que des espaces est sauté sans synthèse.
6. **Le texte sans frontière reste tamponné.** Il n'est synthétisé qu'au `flush()` ou au `close()` (alias l'un de l'autre). En fin d'entrée, toujours appeler `close()` — sinon la queue de texte est perdue.

## Flux d'un streaming

```text
push("The quick brown fox ")        → tampon : "The quick brown fox "        → []
push("jumps over the lazy dog.")    → phrase complète trouvée                → [chunk "The quick brown fox jumps over the lazy dog."]
push("Second")                      → pas de frontière                        → []
close()                             → force la synthèse du reste             → [chunk "Second"]
```

La frontière est cherchée avec `char_indices` sur le premier `.`, `!`, `?` ou `\n` ; la phrase émise inclut le caractère de ponctuation. Chaque `push` peut renvoyer plusieurs chunks si plusieurs phrases se terminent dans le même token.

## Modules

| Module | Rôle |
|---|---|
| `src/lib.rs` | Racine du crate : déclare les modules et ré-exporte toute l'API publique |
| `src/tts.rs` | `JabfyTts` (moteur + voix), `TtsStream` (tampon + frontières), `TtsStreamChunk` |
| `src/types.rs` | `TtsAssets` (chemins), `TtsOptions` (par appel), `TtsConfig` (modèle) |

Détail fichier par fichier : [documentation du code](code/README.md).

## Choix d'implémentation

- **Le chargement est coûteux, fait une fois.** `JabfyTts::create` charge le modèle ONNX et la voix ; l'instance est réutilisée pour toutes les synthèses.
- **La synthèse tourne sur le thread appelant.** Pour pipeliner (synthétiser la phrase N pendant que les tokens de N+1 arrivent), transmettre les chunks à un thread de travail — `GeneratedAudio` est `Send`.
- **Ré-exports de kokoroxide.** `GeneratedAudio`, `VoiceStyle` et `GraphOptimizationLevel` sont ré-exportés : le consommateur n'a pas besoin d'une dépendance directe à kokoroxide.
- **Erreurs uniformes.** Toutes les API fallibles renvoient `Box<dyn std::error::Error>` — le `?` fonctionne dans n'importe quelle fonction compatible.
