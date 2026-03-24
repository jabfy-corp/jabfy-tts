# @jabfy-corp/tts

Kokoro TTS wrapper for jabfy-core. Converts Ollama AI text output to speech using the [Kokoro-82M](https://huggingface.co/hexgrad/Kokoro-82M) model — 82M params, runs fully locally, no API key needed.

## Install

```bash
# Authenticate with GitHub Packages first (once)
npm login --registry=https://npm.pkg.github.com

npm install @jabfy-corp/tts
```

## Usage

### One-shot synthesis

```ts
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create();
const audio = await tts.synthesize("Hello from jabfy!");
audio.save("output.wav");
```

### Streaming (for Ollama token-by-token output)

```ts
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create({ voice: "af_bella" });
const stream = tts.stream();

// Pipe Ollama tokens in as they arrive
ollamaStream.on("token", (token) => stream.push(token));
ollamaStream.on("end", () => stream.close());

let i = 0;
for await (const chunk of stream.output) {
  chunk.audio.save(`chunk-${i++}.wav`);
}
```

### Options

| Option  | Type     | Default      | Description                          |
|---------|----------|--------------|--------------------------------------|
| `voice` | `Voice`  | `"af_heart"` | Kokoro voice ID                      |
| `dtype` | `string` | `"q8"`       | Model precision (`fp32`, `q8`, `q4`) |

Call `tts.listVoices()` to see all available voices.

## Publishing

A GitHub release triggers the publish workflow automatically via GitHub Actions.
