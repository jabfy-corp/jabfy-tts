/**
 * Example script: speak test inputs from command line or default phrases.
 * Usage:
 *   npm run build && node examples/speak-test.mjs ["your text here"]
 *
 * If no text is provided, speaks a default set of test phrases.
 */

// Use local cache to avoid OneDrive sync issues
process.env.HF_HOME = process.env.HF_HOME || "C:\\Users\\eloi.tisserand\\.cache\\huggingface";
process.env.TRANSFORMERS_CACHE = process.env.TRANSFORMERS_CACHE || "C:\\Users\\eloi.tisserand\\.cache\\huggingface\\transformers";

import { JabfyTTS } from "../dist/index.mjs";
import { mkdirSync, existsSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { spawn } from "child_process";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const OUTPUT_DIR = join(__dirname, "..", "output");

const TEST_PHRASES = [
  "Hello from jabfy text to speech!",
  "This is a streaming synthesis test.",
  "Kokoro TTS running fully locally, no API key needed.",
];

function ensureDir(p) {
  if (!existsSync(p)) mkdirSync(p, { recursive: true });
}

async function play(path) {
  const platform = process.platform;
  if (platform === "win32") {
    // Attempt to open with default player (best-effort)
    spawn("start", [path], { shell: true, detached: true });
  } else if (platform === "darwin") {
    spawn("afplay", [path], { detached: true });
  } else {
    // Try common players
    spawn("aplay", [path], { detached: true }).on("error", () => {
      spawn("paplay", [path], { detached: true }).on("error", () => {
        // silent fail
      });
    });
  }
}

async function main() {
  const inputText = process.argv.slice(2).join(" ").trim();
  const phrases = inputText ? [inputText] : TEST_PHRASES;

  console.log("Loading Kokoro model...");
  console.log("  - Creating TTS instance...");
  let tts;
  try {
    tts = await JabfyTTS.create({ voice: "af_heart", dtype: "q8" });
  } catch (err) {
    console.error("  - ERROR creating TTS:", err.message);
    console.error(err.stack);
    process.exit(1);
  }
  console.log("  - TTS instance created successfully");

  ensureDir(OUTPUT_DIR);

  console.log(`Speaking ${phrases.length} phrase(s)...`);
  for (let i = 0; i < phrases.length; i++) {
    const text = phrases[i];
    const outPath = join(OUTPUT_DIR, `test-${i + 1}.wav`);
    console.log(`  [${i + 1}/${phrases.length}] "${text.substring(0, 60)}${text.length > 60 ? "..." : ""}"`);
    const audio = await tts.synthesize(text);
    audio.save(outPath);
    console.log(`      -> saved: ${outPath}`);
    await play(outPath);
    // small delay between utterances
    if (i < phrases.length - 1) await new Promise((r) => setTimeout(r, 800));
  }
  console.log("Done.");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
