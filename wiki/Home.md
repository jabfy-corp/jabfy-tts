# jabfy-tts Wiki

Welcome to the jabfy-tts documentation. This package provides a lightweight wrapper around kokoro-js for text-to-speech synthesis, designed specifically for use with jabfy-core and Ollama streaming output.

## Quick Links

- [Getting Started](Getting-Started)
- [API Reference](API-Reference)
- [Streaming Guide](Streaming-Guide)
- [Voice Options](Voice-Options)
- [Publishing](Publishing)
- [Troubleshooting](Troubleshooting)

## Overview

jabfy-tts is a TypeScript package that converts text to speech using the Kokoro-82M model. Key features:

- 82M parameter model running fully locally
- No API keys or external services required
- Streaming support for real-time synthesis
- Multiple voice options
- Optimized for Ollama token-by-token output
- Built on kokoro-js

## Installation

```bash
# Authenticate with GitHub Packages (one-time setup)
npm login --registry=https://npm.pkg.github.com

# Install the package
npm install @jabfy-corp/tts
```

## Basic Example

```typescript
import { JabfyTTS } from "@jabfy-corp/tts";

const tts = await JabfyTTS.create();
const audio = await tts.synthesize("Hello from jabfy!");
audio.save("output.wav");
```

## Architecture

jabfy-tts wraps the kokoro-js library and provides:

1. Simplified API for common use cases
2. TypeScript type definitions
3. Streaming interface optimized for LLM output
4. Sensible defaults for voice and model quantization

The package uses the `onnx-community/Kokoro-82M-v1.0-ONNX` model from Hugging Face.
