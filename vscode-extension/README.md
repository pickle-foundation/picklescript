# PickleScript for VS Code

Syntax highlighting and language configuration for the PickleScript language
(`.pkl` files).

## What you get

- TextMate grammar (`source.picklescript`) covering comments, strings with
  `{...}` interpolation, characters, numbers, keywords, declarations,
  operators and punctuation.
- Language configuration: `//` and `/* */` comments, bracket matching,
  auto-closing pairs, folding and indentation rules.

## Try it

1. Open this folder in VS Code:
   ```
   code vscode-extension
   ```
2. Press `F5` to launch an Extension Development Host.
3. Open any `.pkl` file — it should be highlighted.

## Install locally

Copy (or symlink) this folder into your extensions directory:

- Windows: `%USERPROFILE%\.vscode\extensions\picklescript.picklescript-0.0.1`
- macOS / Linux: `~/.vscode/extensions/picklescript.picklescript-0.0.1`

Then restart VS Code.

## Note on the `.pkl` extension

`.pkl` is also used by Python's `pickle` module. If another extension claims
the same extension, use VS Code's **Change Language Mode** command and pick
*PickleScript*, or set an association in your `settings.json`:

```json
"files.associations": { "*.pkl": "picklescript" }
```

## Roadmap

This package is the highlighting layer. A language server (completions,
diagnostics, go-to-definition) would live in the repository's `lsp/` crate.
