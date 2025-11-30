# Training Set Extractor

![Build Status](https://img.shields.io/badge/build-passing-brightgreen) ![Rust](https://img.shields.io/badge/rust-2024-orange) ![License](https://img.shields.io/badge/license-Private-red)

**Training Set Extractor** is a high-performance CLI tool written in Rust, designed to streamline the extraction of image datasets from PostgreSQL. It efficiently queries the database, filters records based on metadata classifications, and saves binary image data to the local file system.

## 📖 Features

- **🚀 Async Streaming:** Utilizes `sqlx` and `futures` streams to process database rows one by one, ensuring minimal memory footprint even when extracting thousands of images.
- **🔍 Granular Filtering:** - **Classification:** Filter by exact classification match.
  - **Keywords:** Filter by case-insensitive partial keyword matches (ILIKE).
  - **Exclusion:** Exclude records containing specific keywords to clean the dataset.
- **💾 Auto-Persistence:** Automatically writes binary data (from the `original_image` column) to disk, creating directories as needed.
- **🖼️ Smart Extension Handling:** Detects existing file extensions or defaults to `.jpg` if missing.
- **🛡️ Robust Logging:** Integrated with `tracing` and `tracing-subscriber` for detailed operational logs.

## 🛠️ Tech Stack

- **Language:** Rust (Edition 2024)
- **Database:** PostgreSQL (via `sqlx` 0.8.6)
- **Async Runtime:** Tokio
- **CLI Framework:** Clap 4.5
- **Error Handling:** Anyhow
- **Logging:** Tracing

## ⚙️ Prerequisites

Before running the tool, ensure you have the following:

- **Rust Toolchain:** Stable release (1.75+ recommended).
- **PostgreSQL Database:** A running instance with the `facer_db` schema.
- **Environment:** A `.env` file or environment variables configured.

## 🚀 Installation

1. **Clone the repository:**

   ```bash
   git clone <repository-url>
   cd training_set
   ```

2. **Build the release binary:**

   ```bash
   cargo build --release
   ```

   The compiled binary will be located at `./target/release/training_set`.

## 🔧 Configuration

The application requires a database connection string to be set in the environment.

Create a `.env` file in the root directory:

```env
DB_URL=postgres://username:password@localhost:5432/facer_db
```

## 💻 Usage

Run the tool using the CLI flags to define your output target and filters.

### Basic Usage

Extract all images to a specific folder:

```bash
./target/release/training_set --output-dir ./data/raw_images
```

### Advanced Filtering

Extract images classified as "portrait", containing the keyword "smile", while excluding any marked as "blurry":

```bash
./target/release/training_set \
  --output-dir ./data/clean_portraits \
  --classification "portrait" \
  --keywords "smile" \
  --exclude-keyword "blurry"
```

### CLI Arguments

| Short | Long | Description | Required |
|:---:|:---|:---|:---:|
| `-o` | `--output-dir` | Target directory path to save images. | ✅ |
| `-c` | `--classification` | Filter by exact classification string. | ❌ |
| `-k` | `--keywords` | Filter by partial keyword match (case-insensitive). | ❌ |
| `-e` | `--exclude-keyword` | Exclude records containing this keyword. | ❌ |

## 🏗️ Project Structure

```
├── src
│   ├── app
│   │   ├── db.rs      # Database connection & Query builder logic
│   │   ├── io.rs      # File system operations (async write)
│   │   └── models.rs  # SQLx struct definitions (ImageRecord)
│   ├── app.rs         # Main application logic & Stream processing
│   └── main.rs        # Entry point & Env setup
└── Cargo.toml         # Dependencies
```
