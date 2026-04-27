# Troubleshooting

## Model Download Fails

The first call to `JabfyTTS.create()` downloads the Kokoro-82M ONNX model from Hugging Face. If this fails:

- Check your internet connection
- Verify that `https://huggingface.co` is accessible from your environment
- If you are behind a proxy, set the `HTTPS_PROXY` environment variable

## Authentication Error When Installing

```
npm error code E401
```

You are not authenticated with GitHub Packages. Run:

```bash
npm login --registry=https://npm.pkg.github.com
```

Or add a valid token to your `.npmrc`:

```
@jabfy-corp:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=YOUR_GITHUB_TOKEN
```

The token needs `read:packages` scope.

## Audio Output Is Silent or Corrupted

- Make sure you are calling `stream.close()` after pushing all tokens. Without it, the async iterable may not flush the final buffer.
- If using `flush()`, ensure you still call `close()` at the end.
- Check that the output file path is writable.

## Stream Hangs and Never Completes

The `for await` loop over `stream.output` will not end until `stream.close()` is called. Ensure your token-pushing logic always calls `close()`, including in error paths:

```typescript
try {
  for await (const token of yourTokenSource) {
    stream.push(token);
  }
} finally {
  stream.close();
}
```

## High Memory Usage

The default `dtype` is `q8`. If memory is a concern, use `q4`:

```typescript
const tts = await JabfyTTS.create({ dtype: "q4" });
```

Note that lower quantization reduces audio quality.

## TypeScript Errors

Run the type checker:

```bash
npm run lint
```

This runs `tsc --noEmit` and reports any type errors without producing output files.

## Build Fails

```bash
npm run build
```

Ensure you have Node.js 20 or later installed. The build uses `tsup` to produce CJS, ESM, and type declaration outputs.
