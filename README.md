# 🦀 Multithreaded Web Scraper in Rust

A concurrent web scraper built with Rust's native threading capabilities. Fetches multiple web pages simultaneously and extracts titles, meta descriptions, headings, and links from HTML content. Results are exported to JSON and CSV.

---

## Features

- **Multithreaded**: Uses a thread pool to fetch multiple pages concurrently
- **HTML Parsing**: Extracts titles, meta descriptions, headings (h1-h6), and links
- **Multiple Output Formats**: Exports results to JSON and CSV
- **Error Handling**: Robust error handling with custom error types — scraper never crashes on bad HTML or network errors
- **Modular Design**: Each concern (fetching, parsing, output) is separated into its own module

---

## Project Structure

```
web-scraper/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
└── src/
    ├── main.rs        # Entry point, wires everything together
    ├── error.rs       # Custom error types with thiserror
    ├── models.rs      # Data structures: PageData, Heading, Link
    ├── fetcher.rs     # HTTP client with timeout and user-agent
    ├── parser.rs      # HTML parsing with CSS selectors
    ├── scraper.rs     # Thread pool engine and concurrency logic
    └── output.rs      # JSON and CSV writers
```

---

## Dependencies

```toml
[dependencies]
reqwest  = { version = "0.11", features = ["blocking", "json"] }
scraper  = "0.18"
serde    = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
csv      = "1.3"
thiserror = "1.0"
threadpool = "1.8"
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

### Installation

```bash
git clone https://github.com/your-username/web-scraper.git
cd web-scraper
cargo build
```

### Running

```bash
cargo run
```

The scraper will fetch the configured URLs and generate two output files:

- `data.json` — full structured data
- `data.csv` — flat summary table

---

## Usage

Edit the URL list in `src/main.rs`:

```rust
let urls = vec![
    "https://example.com".to_string(),
    "https://rust-lang.org".to_string(),
    "https://github.com".to_string(),
];
```

Configure the number of concurrent threads:

```rust
let scraper = ScraperEngine::new(4); // 4 threads
```

---

## Output

### JSON (`data.json`)

```json
[
  {
    "url": "https://example.com",
    "title": "Example Domain",
    "meta_description": "An example website",
    "headings": [{ "level": "h1", "text": "Example Domain" }],
    "links": [
      {
        "href": "https://www.iana.org/domains/example",
        "text": "More information..."
      }
    ]
  }
]
```

### CSV (`data.csv`)

| url                 | title          | meta_description   | headings_count | links_count |
| ------------------- | -------------- | ------------------ | -------------- | ----------- |
| <https://example.com> | Example Domain | An example website | 1              | 1           |

---

## Architecture

### Concurrency Model

```
main.rs
  └── ScraperEngine
        ├── ThreadPool (N threads)
        │     ├── Thread 1 → fetch URL → parse HTML → send via channel
        │     ├── Thread 2 → fetch URL → parse HTML → send via channel
        │     └── Thread N → fetch URL → parse HTML → send via channel
        └── mpsc::channel → collect all results → Vec<PageData>
```

### Key Rust Concepts Used

| Concept                | Where Used                                   |
| ---------------------- | -------------------------------------------- |
| `Arc<T>`               | Sharing `HttpFetcher` across threads safely  |
| `mpsc::channel`        | Collecting results from threads back to main |
| `ThreadPool`           | Managing concurrent HTTP requests            |
| `Result<T, E>` + `?`   | Error propagation without panics             |
| `#[derive(Serialize)]` | Automatic JSON/CSV serialization             |
| `Option<T>`            | Handling missing HTML fields gracefully      |

---

## Error Handling

The scraper defines a custom `ScraperError` enum covering:

- `RequestError` — network or HTTP failures
- `HtmlParseError` — malformed or unexpected HTML
- `IoError` — file system errors
- `JsonError` — JSON serialization failures
- `CsvError` — CSV writing failures

Each thread handles its own errors independently. A failed URL is logged to `stderr` and skipped — the scraper continues processing remaining URLs.

---

## Limitations

- Does not currently check `robots.txt`
- CSV output flattens headings and links to counts only (use JSON for full data)
- Blocking I/O model (uses `reqwest::blocking`)

---

## Built With

- [Rust](https://www.rust-lang.org/)
- [reqwest](https://docs.rs/reqwest) — HTTP client
- [scraper](https://docs.rs/scraper) — HTML parsing
- [serde](https://serde.rs/) — serialization
- [threadpool](https://docs.rs/threadpool) — thread pool
- [thiserror](https://docs.rs/thiserror) — error handling

---

## License

MIT
