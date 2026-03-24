/** Available Kokoro voice IDs */
export type Voice = string;

export interface TTSOptions {
  /** Kokoro voice ID. Defaults to "af_heart" */
  voice?: Voice;
  /**
   * Model quantization dtype.
   * "q8" is a good balance of quality and speed for Node/server use.
   */
  dtype?: "fp32" | "fp16" | "q8" | "q4" | "q4f16";
}

export interface TTSStreamChunk {
  text: string;
  phonemes: string;
  /** Raw audio data — save or pipe to a speaker */
  audio: {
    save(path: string): void;
    data: Float32Array;
    sampling_rate: number;
  };
}
