# @jabfy-corp/tts

Kokoro TTS wrapper for jabfy-core. Converts Ollama AI text output to speech using the [Kokoro-82M](https://huggingface.co/hexgrad/Kokoro-82M) model — 82M parameters, runs fully locally, no API key needed.

## Install

Add the registry to your `.npmrc`:

```
@jabfy-corp:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN
```

Then install:

```bash
npm install @jabfy-corp/tts
```

## Usage

### One-shot synthesis

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create();
const audio = await tts.synthesize("Hello from jabfy!");
audio.save("output.wav");
```

### Streaming (for Ollama token-by-token output)

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create({ voice: "af_bella" });
const stream = tts.stream();

// Consume audio concurrently while tokens are pushed
const audioConsumer = (async () => {
  let i = 0;
  for await (const chunk of stream.output) {
    chunk.audio.save(`chunk-${i++}.wav`);
  }
})();

// Feed Ollama tokens as they arrive
ollamaStream.on("token", (token) => stream.push(token));
ollamaStream.on("end", () => stream.close());

await audioConsumer;
```

### Options

| Option  | Type     | Default      | Description                                          |
|---------|----------|--------------|------------------------------------------------------|
| `voice` | `string` | `"af_heart"` | Kokoro voice ID                                      |
| `dtype` | `string` | `"q8"`       | Model quantization: `fp32`, `fp16`, `q8`, `q4`, `q4f16` |

Call `tts.listVoices()` to print all available voice IDs.

## API

### `JabfyTTS.create(options?)`

Loads the model and returns a `JabfyTTS` instance. Call once and reuse.

### `tts.synthesize(text, options?)`

Generates audio for a complete string. Returns a kokoro-js audio object with `.save(path)`, `.data` (Float32Array), and `.sampling_rate`.

### `tts.stream(options?)`

Returns a streaming handle:

| Member   | Description                                                  |
|----------|--------------------------------------------------------------|
| `push`   | Feed a token into the stream                                 |
| `flush`  | Emit audio for buffered text without closing the stream      |
| `close`  | Signal end of input                                          |
| `output` | Async iterable of `TTSStreamChunk`                           |

Each `TTSStreamChunk` contains `text`, `phonemes`, and `audio` (with `.save()`, `.data`, `.sampling_rate`).

### `tts.listVoices()`

Prints all available Kokoro voice IDs to the console.

## Publishing

Creating a GitHub Release triggers the publish workflow automatically. The workflow builds the package and publishes it to GitHub Packages using `GITHUB_TOKEN`.

See the [wiki](../../wiki) for full documentation including the streaming guide, voice options, and troubleshooting.
