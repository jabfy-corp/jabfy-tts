# Getting Started

## Prerequisites

- Node.js 20 or later
- Access to the `@jabfy-corp` GitHub Packages registry

## Authentication

jabfy-tts is published to GitHub Packages. You need to authenticate before installing.

Create or update your `.npmrc` file in your project root:

```
@jabfy-corp:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN
```

Replace `YOUR_GITHUB_TOKEN` with a GitHub personal access token that has `read:packages` scope.

Alternatively, authenticate via the CLI:

```bash
npm login --registry=https://npm.pkg.github.com
```

## Installation

```bash
npm install @jabfy-corp/tts
```

## First Synthesis

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";

// Load the model — do this once and reuse the instance
const tts = await JabfyTTS.create();

// Synthesize text and save to a file
const audio = await tts.synthesize("Hello from jabfy!");
audio.save("output.wav");
```

The first call to `JabfyTTS.create()` will download the Kokoro-82M ONNX model from Hugging Face. Subsequent calls reuse the cached model.

## Choosing a Voice

Pass a voice ID when creating the instance or per synthesis call:

```typescript
const tts = await JabfyTTS.create({ voice: "af_bella" });
const audio = await tts.synthesize("Hello!");
audio.save("output.wav");
```

See the [Voice Options](Voice-Options) page for available voices.

## Choosing a Model Precision

The `dtype` option controls the quantization level of the model:

```typescript
const tts = await JabfyTTS.create({ dtype: "fp32" });
```

| dtype   | Quality | Speed | Memory |
|---------|---------|-------|--------|
| fp32    | Highest | Slow  | High   |
| fp16    | High    | Medium| Medium |
| q8      | Good    | Fast  | Low    |
| q4      | Lower   | Fastest| Lowest|
| q4f16   | Lower   | Fastest| Lowest|

The default is `q8`, which is a good balance for server-side use.

## Next Steps

- [API Reference](API-Reference) — full method and type documentation
- [Streaming Guide](Streaming-Guide) — real-time synthesis with Ollama
