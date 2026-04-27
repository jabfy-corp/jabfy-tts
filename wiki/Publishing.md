# Publishing

jabfy-tts is published to GitHub Packages under the `@jabfy-corp` scope.

## Automated Publishing

Publishing is handled automatically by a GitHub Actions workflow. When a new GitHub Release is created, the workflow:

1. Checks out the repository
2. Sets up Node.js 20 with the GitHub Packages registry
3. Runs `npm install`
4. Runs `npm run build` (compiles TypeScript to CJS and ESM via tsup)
5. Runs `npm publish`

The workflow uses `GITHUB_TOKEN` for authentication, so no additional secrets are needed.

## Creating a Release

To publish a new version:

1. Update the `version` field in `package.json`
2. Commit and push the change
3. Go to the repository on GitHub
4. Create a new Release with a matching tag (e.g., `v0.2.0`)
5. The publish workflow triggers automatically on release creation

## Build Output

The build produces three artifacts in the `dist/` folder:

| File            | Format      | Description                        |
|-----------------|-------------|------------------------------------|
| index.js        | CommonJS    | For `require()` consumers          |
| index.mjs       | ESM         | For `import` consumers             |
| index.d.ts      | TypeScript  | Type declarations                  |

Only the `dist/` folder is included in the published package (controlled by the `files` field in `package.json`).

## Manual Publishing

If you need to publish manually:

```bash
# Authenticate
npm login --registry=https://npm.pkg.github.com

# Build
npm run build

# Publish
npm publish
```

## Registry Configuration

The package is scoped to `@jabfy-corp` and published to GitHub Packages. The `.npmrc` file in the repository root configures this:

```
@jabfy-corp:registry=https://npm.pkg.github.com
```

Consumers of the package need the same `.npmrc` entry (or equivalent environment configuration) to install it.
