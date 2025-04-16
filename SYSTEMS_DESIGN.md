# system_design.md

## overview

`gitpeek` is a fast, minimal command-line interface tool designed to search across github repositories—both remote and locally cached—through an extensible config-driven architecture. it enables developers to view and query repos quickly, manage a personal repo bank, and pattern-match through project source files without needing to clone or navigate each repo manually.

the tool prioritizes modular rust design, async concurrency, high-speed local search, and intuitive cli interaction.

---

## architecture overview

```
+-----------------------+
|      user cli        |
|  (clap interface)    |
+----------+------------+
           |
           v
+----------+------------+
|     command router    |
+----------+------------+
           |
     +-----+-----+
     |           |
     v           v
[config]     [repo handler]
     |           |
     v           v
[resolver]   [search engine]
     |
     v
[github api client]
```

---

## core modules

### 1. `cli.rs`
- uses `clap` for parsing commands and flags.
- maps user input to command execution flow.
- supports subcommands like:
  - `add <alias> <url>`
  - `remove <alias>`
  - `list`
  - `search <pattern> --repo <alias>`
  - `fetch <alias>`

### 2. `config.rs`
- manages a toml-based config file stored at:
  `~/.config/gitpeek/gitpeek.toml` (linux/mac)
  or `%APPDATA%\gitpeek\config.toml` (windows)
- config structure:
  ```toml
  [repos]
  rust_book = "https://github.com/rust-lang/book"
  kaizen = "https://github.com/avneet-kaizen/kaizenos"
  ```
- handles loading, writing, updating entries atomically.

### 3. `repo.rs`
- responsible for managing local repo state.
- on `add`, optionally clones or pulls repo.
- stores files under:
  `~/.cache/gitpeek/repos/<alias>/`
- supports shallow clones (`--depth 1`) to speed things up.
- ensures updates via `git pull` if the repo already exists locally.

### 4. `github.rs`
- async github api client using `reqwest + tokio`.
- features:
  - fetch raw file contents.
  - recursively list directory structures.
  - handle pagination and rate limits.
  - optional personal access token auth support.
- implements retry + backoff behavior for stability.

### 5. `search.rs`
- fast file content searching using either:
  - `regex` crate for simple patterns.
  - optional `ripgrep` backend via child process for speed.
- supports:
  - filtering by filename patterns
  - multiline match capture
  - path-based search scoping

### 6. `utils.rs`
- reusable helpers (e.g., fs ops, json helpers, logging).
- small abstractions to reduce repeated logic.

---

## async execution

`tokio` is used to handle all async workloads including:
- parallel github api calls
- concurrent file reads during search
- multi-repo scanning (e.g., search across all repos)

example:
```rust
let tasks: Vec<_> = repos
  .iter()
  .map(|repo| tokio::spawn(search_in_repo(repo, pattern)))
  .collect();

for task in tasks {
  let _ = task.await?;
}
```

---

## dependency stack

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
directories = "5"
regex = "1"
```

---

## sample workflow

1. user runs: `gitpeek add rust-book https://github.com/rust-lang/book`
   - config is updated.
   - repo is shallow-cloned locally.

2. user runs: `gitpeek search "async fn" --repo rust-book`
   - local cache is checked or pulled.
   - directory is scanned for matching files.
   - matches are printed with filename + line number.

---

## future improvements

- fuzzy search over file paths
- support for other vcs providers (gitlab, bitbucket)
- parallel prefetching and indexing
- full-text index engine via `tantivy` for large-scale performance
- bat-style preview output with syntax highlighting
- export search results to json/yaml

---

## philosophy

lean, readable rust with modular layers.
avoid premature optimization.
async where it counts.
always respect user time and mental space.


