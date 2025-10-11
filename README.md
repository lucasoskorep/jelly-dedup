# Jelly Dedup

A command-line tool to identify and manage duplicate episodes and movies in your Jellyfin media server. This tool analyzes your TV show and movie libraries, detects duplicates, and provides removal commands to free up storage space.

## Features

- Scans TV shows and/or movies in your Jellyfin library
- Identifies duplicate episodes based on season and episode numbers
- Identifies duplicate movies based on title and year
- Intelligently selects lower-quality files for removal using smart codec comparison
- Generates shell commands for safe file deletion
- Displays space savings estimates
- Supports custom path prefix removal for cleaner output
- Flexible media type selection (TV shows only, movies only, or both)

## Build Reqs

- Latest Rust
- Access to a Jellyfin server
- A valid Jellyfin API key

## Installation

### From Source

1. Build the project:
```bash
cargo build --release
```

2. The binary will be available at `target/release/jelly-dedup`

Optionally, install it system-wide:
```bash
cargo install --path .
```

## Configuration

jelly-dedup can be configured using either environment variables or command-line arguments. Command-line arguments take precedence over environment variables.

### Environment Variables

Create a `.env` file in the project directory:

```bash
JELLYFIN_URL=http://localhost:8096
JELLYFIN_API_KEY=your_api_key_here
PATH_PREFIX_TO_REMOVE=/mnt/media  # Optional
```

### Command-Line Arguments

All configuration can be passed as command-line arguments:

```bash
jelly-dedup --jellyfin-url http://localhost:8096 --api-key your_api_key_here
```

## Usage

### Basic Usage

Using environment variables (via `.env` file):
```bash
jelly-dedup
```

Using command-line arguments:
```bash
jelly-dedup --api-key YOUR_API_KEY
```

### All Options

```bash
jelly-dedup [OPTIONS]

Options:
  -j, --jellyfin-url <JELLYFIN_URL>
          Jellyfin server URL [env: JELLYFIN_URL] [default: http://localhost:8096]

  -a, --api-key <API_KEY>
          Jellyfin API key [env: JELLYFIN_API_KEY]

  -p, --path-prefix-to-remove <PATH_PREFIX_TO_REMOVE>
          Path prefix to remove from displayed file paths [env: PATH_PREFIX_TO_REMOVE]

  -t, --media-type <MEDIA_TYPE>
          Type of media to process [default: both] [possible values: tv, movies, both]

  -h, --help
          Print help

  -V, --version
          Print version
```

### Examples

1. **Scan both TV shows and movies (default):**
   ```bash
   jelly-dedup --api-key abc123def456
   ```

2. **Scan TV shows only:**
   ```bash
   jelly-dedup --api-key abc123def456 --media-type tv
   ```

3. **Scan movies only:**
   ```bash
   jelly-dedup --api-key abc123def456 --media-type movies
   ```

4. **Specifying a remote server:**
   ```bash
   jelly-dedup --jellyfin-url https://jellyfin.example.com --api-key abc123def456
   ```

5. **Removing path prefix for cleaner output:**
   ```bash
   jelly-dedup --api-key abc123def456 --path-prefix-to-remove /mnt/media
   ```

6. **Using environment variables:**
   ```bash
   export JELLYFIN_URL=http://localhost:8096
   export JELLYFIN_API_KEY=abc123def456
   jelly-dedup
   ```

## Getting Your Jellyfin API Key

1. Log in to your Jellyfin server web interface
2. Go to Dashboard → API Keys
3. Click "+" to create a new API key
4. Give it a name (e.g., "jelly-dedup")
5. Copy the generated API key

## Output

The tool will:
1. Scan TV shows and/or movies in your library (depending on `--media-type` option)
2. Display duplicate episodes/movies found with detailed quality information
3. Provide a summary with:
   - Total episodes/movies with duplicates
   - Total files marked for deletion
   - Estimated space savings in GB
4. Generate `rm` commands for each file to be deleted

**Note:** The tool does NOT delete files automatically. It only generates the commands for you to review and execute manually.

### Quality Selection

The tool uses intelligent quality comparison to select the best version:
- **Resolution First**: Higher resolution always wins (1080p beats 720p)
- **Codec Efficiency**: When resolutions match, codec efficiency is considered:
  - AV1: 2.0x multiplier (most efficient)
  - H.265/HEVC: 1.5x multiplier
  - H.264: 1.0x baseline
- **Effective Bitrate**: Calculates quality based on bitrate × codec efficiency

For example, a 1080p H.265 file at 6 Mbps (effective: 9.0) will be selected over a 1080p H.264 file at 8 Mbps (effective: 8.0).

## Safety

- The tool is read-only and makes no modifications to your Jellyfin library or filesystem
- All file deletion commands are generated for manual review and execution
- File paths in deletion commands are properly escaped for shell safety
- Always review the generated commands before executing them
