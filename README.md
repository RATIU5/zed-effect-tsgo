# effect-tsgo for Zed

Zed extension for the Effect Language Service powered by `@effect/tsgo`.

## Requirements

This extension starts the language server binary for Zed. You still need to set up your project for `@effect/tsgo` itself.

Run the upstream setup command in your project:

```sh
npx @effect/tsgo setup
```

That setup handles the project-side requirements from the `@effect/tsgo` README, including:

- adding `@effect/tsgo`
- configuring `tsconfig.json` for the Effect language service plugin
- guiding any additional editor-related setup

You currently still need the standard native TypeScript install alongside `@effect/tsgo`:

```sh
npm install -D @typescript/native-preview
```

## Enable In Zed

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["effect-tsgo"]
    }
  }
}
```

## Configuration

The extension resolves the server binary in this order:

1. `lsp.effect-tsgo.settings.binary.path`
2. `lsp.effect-tsgo.settings.package_version`
3. latest `@effect/tsgo` from npm

Important: the executable name is `tsgo`, not `effect-tsgo`.

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

Prefer the real native `tsgo` binary, not the `effect-tsgo` CLI name.

```json
{
  "lsp": {
    "effect-tsgo": {
      "settings": {
        "binary": {
          "path": "/absolute/path/to/node_modules/@effect/tsgo-darwin-arm64/lib/tsgo"
        }
      }
    }
  }
}
```

### Use Zed's Raw Binary Override

If you intentionally use Zed's built-in `lsp.effect-tsgo.binary.path`, Zed bypasses the extension wrapper. In that mode you must point at `tsgo` and provide the startup arguments yourself.

```json
{
  "lsp": {
    "effect-tsgo": {
      "binary": {
        "path": "./node_modules/.bin/tsgo",
        "arguments": ["--lsp", "--stdio"]
      }
    }
  }
}
```

Use this raw override only when you want Zed to launch the command directly from the worktree.
