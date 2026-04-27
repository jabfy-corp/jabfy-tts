# API Reference

## JabfyTTS

The main class. Create a single instance and reuse it throughout your application.

---

### `JabfyTTS.create(options?)`

Static factory method. Loads the Kokoro model and returns a ready-to-use instance.

```typescript
static async create(options?: TTSOptions): Promise<JabfyTTS>
```

**Parameters**

| Parameter | Type       | Required | Description                  |
|-----------|------------|----------|------------------------------|
| options   | TTSOptions | No       | Model and voice configuration |

**Returns** `Promise<JabfyTTS>`

**Example**

```typescript
const tts = await JabfyTTS.create({ dtype: "q8", voice: "af_heart" });
```

---

### `tts.synthesize(text, options?)`

Generates audio for a complete text string. Waits for the full audio to be generated before returning.

```typescript
async synthesize(text: string, options?: TTSOptions): Promise<AudioOutput>
```

**Parameters**

| Parameter | Type       | Required | Description                        |
|-----------|------------|----------|------------------------------------|
| text      | string     | Yes      | The text to synthesize             |
| options   | TTSOptions | No       | Override voice for this call only  |

**Returns** `Promise<AudioOutput>` — the kokoro-js audio object with a `.save(path)` method and `.data` / `.sampling_rate` properties.

**Example**

```typescript
const audio = await tts.synthesize("Hello world", { voice: "af_bella" });
audio.save("output.wav");

// Access raw audio data
console.log(audio.sampling_rate); // e.g. 24000
console.log(audio.data);          // Float32Array
```

---

### `tts.stream(options?)`

Returns a streaming handle for real-time synthesis. Designed for feeding LLM tokens as they arrive.

```typescript
stream(options?: TTSOptions): StreamHandle
```

**Parameters**

| Parameter | Type       | Required | Description                        |
|-----------|------------|----------|------------------------------------|
| options   | TTSOptions | No       | Voice configuration for the stream |

**Returns** `StreamHandle`

```typescript
{
  push:   (token: string) => void;
  flush:  () => void;
  close:  () => void;
  output: AsyncIterable<TTSStreamChunk>;
}
```

| Property | Description                                                  |
|----------|--------------------------------------------------------------|
| push     | Feed a token or word into the stream                         |
| flush    | Emit audio for currently buffered text without closing       |
| close    | Signal end of input — no more tokens will be pushed          |
| output   | Async iterable that yields TTSStreamChunk as audio is ready  |

**Example**

```typescript
const stream = tts.stream({ voice: "af_bella" });

stream.push("Hello ");
stream.push("world");
stream.close();

for await (const chunk of stream.output) {
  chunk.audio.save(`chunk.wav`);
}
```

---

### `tts.listVoices()`

Prints all available voice IDs to the console.

```typescript
listVoices(): void
```

**Example**

```typescript
tts.listVoices();
```

---

## TTSOptions

```typescript
interface TTSOptions {
  voice?: Voice;
  dtype?: "fp32" | "fp16" | "q8" | "q4" | "q4f16";
}
```

| Field | Type   | Default      | Description                                                  |
|-------|--------|--------------|--------------------------------------------------------------|
| voice | Voice  | `"af_heart"` | Kokoro voice ID                                              |
| dtype | string | `"q8"`       | Model quantization level. Only applies to `JabfyTTS.create` |

---

## TTSStreamChunk

Each item yielded by `stream.output`.

```typescript
interface TTSStreamChunk {
  text:     string;
  phonemes: string;
  audio: {
    save(path: string): void;
    data:          Float32Array;
    sampling_rate: number;
  };
}
```

| Field          | Type         | Description                                  |
|----------------|--------------|----------------------------------------------|
| text           | string       | The text segment this chunk corresponds to   |
| phonemes       | string       | Phoneme representation of the text           |
| audio.save     | function     | Save the chunk audio to a file               |
| audio.data     | Float32Array | Raw PCM audio samples                        |
| audio.sampling_rate | number  | Sample rate in Hz (typically 24000)          |

---

## Voice

```typescript
type Voice = string;
```

A Kokoro voice ID string. Use `tts.listVoices()` to see all available options. The default is `"af_heart"`.
