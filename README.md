# effect-tsgo: Effect Language Service for Zed

This extension integrates `@effect/tsgo` — the Effect Language Service fork of Microsoft's native Go-based TypeScript compiler — into the Zed editor, providing Effect-TS diagnostics alongside the performance benefits of the native TypeScript compiler.

## Why `effect-tsgo`?

`@effect/tsgo` builds on `tsgo` (Microsoft's native Go port of TypeScript) and adds Effect-TS-specific diagnostics via the `@effect/language-service` plugin. You get:

- **Effect-TS diagnostics** — Effect-specific rules and hints surfaced directly in the editor
- **Faster compilation** — up to 10x speed improvements from the native Go compiler
- **Reduced memory usage** — optimized memory handling in native execution
- **Improved editor performance** — faster IntelliSense and language services

> _Example Benchmarks (tsgo baseline)_:
>
> - **VS Code**: 77.8s → 7.5s (10.4x speedup)
> - **Playwright**: 11.1s → 1.1s (10.1x speedup)
> - **TypeORM**: 17.5s → 1.3s (13.5x speedup)
>
> _Source: [Microsoft Developer Blog](https://devblogs.microsoft.com/typescript/typescript-native-port/)_

## Prerequisites

This extension only manages the LSP binary. For Effect diagnostics to appear, your project must also be configured with the `@effect/language-service` TypeScript plugin. Run the interactive setup in your project directory:

```bash
npx @effect/tsgo setup
npm install
```

The `setup` command configures your `tsconfig.json` and `package.json` automatically.

## Installation

1. Open Zed's Extensions page.
2. Search for `effect-tsgo` and install the extension.

## Configuration

### Basic Setup

Enable `effect-tsgo` in your Zed settings:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["effect-tsgo"]
    }
  }
}
```

You can also use `effect-tsgo` in tandem with other language servers. Zed will use `effect-tsgo` for features it supports and fall back to the next server in the list:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["effect-tsgo", "vtsls"]
    }
  }
}
```

### Pinning a Package Version

By default, the extension installs and uses the latest version of `@effect/tsgo` from npm. To pin a specific version:

```json
{
  "lsp": {
    "effect-tsgo": {
      "settings": {
        "package_version": "0.0.15"
      }
    }
  }
}
```

This is useful for ensuring consistent behavior across a team or avoiding automatic updates.

## Status

This extension is in early development. Contributions and feedback are welcome at [https://github.com/RATIU5/zed-effect-tsgo](https://github.com/RATIU5/zed-effect-tsgo).
