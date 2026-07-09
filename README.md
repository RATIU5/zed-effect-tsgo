# Effect Language Service (tsgo) for Zed

Zed extension for the Effect Language Service powered by `@effect/tsgo`.

## Requirements

This extension starts the language server binary for Zed. You still need to set up your project for `@effect/tsgo` itself.

When installing this as a dev extension, Zed compiles the extension for `wasm32-wasip2`.
Install that Rust target first:

```sh
rustup target add wasm32-wasip2
```

If the target is installed but Zed still reports `can't find crate for core` or `std`,
make sure Zed is using the rustup `cargo` and `rustc`, not another Rust installation such as Homebrew.
Putting `~/.cargo/bin` before `/opt/homebrew/bin` in `PATH` is usually enough.

You also need a native TypeScript install alongside `@effect/tsgo`. As of TypeScript 7.0 stable, this is just the regular `typescript` package:

```sh
npm install -D typescript@7
```

Nightlies still ship under `@typescript/native-preview` if you want the bleeding edge instead:

```sh
npm install -D @typescript/native-preview
```

`@effect/tsgo` detects whichever one is installed.

You also need the Effect language service plugin configured in your `tsconfig.json` (see [Recommended Setup](#recommended-setup) below).

## Recommended Setup

This extension runs the `@effect/tsgo` binary directly. That binary is a superset of TypeScript-Go with the Effect language service compiled in, so it acts as your sole TypeScript language server so you don't need `vtsls` or `typescript-language-server` running alongside it.

### 1. Configure `tsconfig.json`

Add the Effect language service plugin so diagnostics are enabled:

```jsonc
// tsconfig.json
{
  "compilerOptions": {
    "plugins": [{ "name": "@effect/language-service" }],
  },
}
```

See the [`@effect/tsgo` README](https://github.com/Effect-TS/language-service) for the full list of plugin options (diagnostic severities, refactors, import aliases, etc.).

### 2. Enable in Zed

Point your TypeScript-family languages at `effect-tsgo` and disable Zed's built-in TypeScript servers so you don't get duplicate diagnostics:

```jsonc
// .zed/settings.json
{
  "languages": {
    "TypeScript": {
      "language_servers": [
        "effect-tsgo",
        "!vtsls",
        "!typescript-language-server",
      ],
    },
    "TSX": {
      "language_servers": [
        "effect-tsgo",
        "!vtsls",
        "!typescript-language-server",
      ],
    },

    // Optional
    "JavaScript": {
      "language_servers": [
        "effect-tsgo",
        "!vtsls",
        "!typescript-language-server",
      ],
    },
    "JSX": {
      "language_servers": [
        "effect-tsgo",
        "!vtsls",
        "!typescript-language-server",
      ],
    },
  },
}
```

### About `npx @effect/tsgo setup`

Running `setup` is **optional** with this extension. It adds the `@effect/tsgo` dependency, writes the `tsconfig.json` plugin block, and prints editor hints, but the extension already auto-installs `@effect/tsgo` for you, and the tsconfig block above is the only project-side piece that matters. If you'd rather run it, `npx @effect/tsgo setup` handles step 1 for you.

You do **not** need the `"prepare": "effect-tsgo patch"` script. `patch` exists to patch a plain native-TypeScript install so a generic `tsgo` binary loads the Effect language service. Since this extension launches the dedicated `@effect/tsgo` binary (which already embeds the language service), the patch step does nothing useful here.

## Configuration

The extension resolves the server binary in this order:

1. `lsp.effect-tsgo.binary.path`
2. `lsp.effect-tsgo.settings.package_version`
3. latest `@effect/tsgo` from npm

Important: the shipped executable is now `tsc` (matching TypeScript-Go upstream). Older `@effect/tsgo` packages still ship `tsgo`; this extension accepts either. Prefer the real native binary, not the `effect-tsgo` CLI name.

### Pin A Package Version

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

### Use A Local Binary

Prefer the real native `tsc` binary (or `tsgo` on older packages), not the `effect-tsgo` CLI name.

```json
{
  "lsp": {
    "effect-tsgo": {
      "binary": {
        "path": "/absolute/path/to/node_modules/@effect/tsgo-darwin-arm64/lib/tsc"
      }
    }
  }
}
```
