# Streaming Guide

The streaming API is the primary reason jabfy-tts exists. It is designed to consume Ollama token-by-token output and produce audio in real time, so the user hears speech as the model generates text rather than waiting for the full response.

## How It Works

`tts.stream()` returns a handle with four members:

- `push(token)` — feed a token into the internal text buffer
- `flush()` — force audio generation for whatever is currently buffered
- `close()` — signal that no more tokens are coming
- `output` — async iterable that yields audio chunks as they are ready

Internally, kokoro-js uses a `TextSplitterStream` to decide when enough text has accumulated to synthesize a natural-sounding segment. You do not need to manage sentence boundaries yourself.

## Basic Streaming Example

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create();
const stream = tts.stream();

// Push tokens manually
stream.push("The quick ");
stream.push("brown fox ");
stream.push("jumps over the lazy dog.");
stream.close();

let i = 0;
for await (const chunk of stream.output) {
  console.log("Synthesized:", chunk.text);
  chunk.audio.save(`chunk-${i++}.wav`);
}
```

## Integrating with Ollama

The most common use case is piping Ollama streaming output directly into jabfy-tts.

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";
import Ollama from "ollama";

const tts = await JabfyTTS.create({ voice: "af_bella" });
const stream = tts.stream();

// Consume audio chunks concurrently while tokens are being pushed
const audioConsumer = (async () => {
  let i = 0;
  for await (const chunk of stream.output) {
    chunk.audio.save(`response-${i++}.wav`);
  }
})();

// Stream tokens from Ollama
const ollamaStream = await Ollama.chat({
  model: "llama3",
  messages: [{ role: "user", content: "Tell me a short story." }],
  stream: true,
});

for await (const part of ollamaStream) {
  const token = part.message.content;
  if (token) stream.push(token);
}

stream.close();
await audioConsumer;
```

## Using flush()

`flush()` forces audio generation for any text currently in the buffer without closing the stream. This is useful when you want to ensure a specific phrase is synthesized immediately, for example after a sentence boundary you detect yourself.

```typescript
stream.push("First sentence.");
stream.flush(); // synthesize now without waiting for more text

stream.push("Second sentence.");
stream.close();
```

## Accessing Raw Audio Data

Instead of saving to files, you can pipe the raw PCM data to a speaker or audio output:

```typescript
for await (const chunk of stream.output) {
  const { data, sampling_rate } = chunk.audio;
  // data is a Float32Array of PCM samples
  // sampling_rate is typically 24000 Hz
  yourSpeaker.write(data, sampling_rate);
}
```

## Important Notes

- Create the stream before you start pushing tokens. The `output` iterable must be consumed concurrently with pushing, otherwise the internal buffer will block.
- Call `stream.close()` when you are done pushing tokens. Without it, the `for await` loop will never end.
- Each `JabfyTTS` instance can only run one stream at a time. Create multiple instances if you need concurrent streams.
