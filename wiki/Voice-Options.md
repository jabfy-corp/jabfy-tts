# Voice Options

jabfy-tts uses the Kokoro-82M model, which ships with a set of built-in voices. Voices are identified by string IDs.

## Default Voice

The default voice is `af_heart`. It is used when no `voice` option is provided.

## Listing Available Voices

Call `listVoices()` on any `JabfyTTS` instance to print all available voice IDs to the console:

```typescript
const tts = await JabfyTTS.create();
tts.listVoices();
```

## Setting a Voice

You can set a voice at the instance level or per call.

**Instance level** — applies to all synthesize and stream calls:

```typescript
const tts = await JabfyTTS.create({ voice: "af_bella" });
```

**Per call** — overrides the instance default for a single synthesis:

```typescript
const audio = await tts.synthesize("Hello!", { voice: "af_sky" });
```

**Per stream** — sets the voice for a streaming session:

```typescript
const stream = tts.stream({ voice: "af_bella" });
```

## Voice Naming Convention

Kokoro voice IDs follow a pattern:

- The first two characters indicate the language/accent (e.g., `af` for American Female, `am` for American Male, `bf` for British Female, `bm` for British Male)
- The suffix is the voice name (e.g., `heart`, `bella`, `sky`)

Refer to the [Kokoro-82M model page](https://huggingface.co/hexgrad/Kokoro-82M) for the full and up-to-date list of voices.

## Voice Type

The `Voice` type is a plain string alias:

```typescript
type Voice = string;
```

This means any valid Kokoro voice ID can be passed without needing to import an enum.
