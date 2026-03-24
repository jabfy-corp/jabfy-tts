import { KokoroTTS, TextSplitterStream } from "kokoro-js";
import type { TTSOptions, TTSStreamChunk } from "./types";

const MODEL_ID = "onnx-community/Kokoro-82M-v1.0-ONNX";
const DEFAULT_VOICE = "af_heart";
const DEFAULT_DTYPE = "q8";

/**
 * JabfyTTS — thin wrapper around kokoro-js for use with jabfy-core.
 *
 * Usage:
 *   const tts = await JabfyTTS.create();
 *   const audio = await tts.synthesize("Hello world");
 *   audio.save("output.wav");
 *
 * Streaming (ideal for Ollama token-by-token output):
 *   const stream = tts.stream({ voice: "af_bella" });
 *   for await (const chunk of stream.output) { ... }
 *   stream.push("Hello "); stream.push("world"); stream.close();
 */
export class JabfyTTS {
  private constructor(private readonly model: KokoroTTS) {}

  /** Load the Kokoro model. Call once and reuse the instance. */
  static async create(options: TTSOptions = {}): Promise<JabfyTTS> {
    const model = await KokoroTTS.from_pretrained(MODEL_ID, {
      dtype: options.dtype ?? DEFAULT_DTYPE,
      device: "cpu",
    });
    return new JabfyTTS(model);
  }

  /** Generate audio for a complete text string. */
  async synthesize(text: string, options: TTSOptions = {}) {
    return this.model.generate(text, {
      voice: options.voice ?? DEFAULT_VOICE,
    });
  }

  /**
   * Stream audio chunks as text arrives — designed for Ollama streaming.
   *
   * Returns a { push, flush, close, output } handle:
   *   - push(token)  — feed a token/word from the LLM
   *   - flush()      — emit audio for buffered text without closing
   *   - close()      — signal end of input
   *   - output       — async iterable of TTSStreamChunk
   */
  stream(options: TTSOptions = {}): {
    push: (token: string) => void;
    flush: () => void;
    close: () => void;
    output: AsyncIterable<TTSStreamChunk>;
  } {
    const splitter = new TextSplitterStream();
    const rawStream = this.model.stream(splitter, {
      voice: options.voice ?? DEFAULT_VOICE,
    });

    return {
      push: (token: string) => splitter.push(token),
      flush: () => splitter.flush(),
      close: () => splitter.close(),
      output: rawStream as AsyncIterable<TTSStreamChunk>,
    };
  }

  /** List all available voices. */
  listVoices(): void {
    this.model.list_voices();
  }
}
