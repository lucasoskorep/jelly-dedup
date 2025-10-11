# Jelly Dedup

A command-line tool to identify and manage duplicate episodes in your Jellyfin media server. This tool analyzes your TV show library, detects duplicate episodes, and provides removal commands to free up storage space.

## Features

- Scans all TV shows in your Jellyfin library
- Identifies duplicate episodes based on season and episode numbers
- Intelligently selects lower-quality files for removal
- Generates shell commands for safe file deletion
- Displays space savings estimates
- Supports custom path prefix removal for cleaner output

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

  -h, --help
          Print help

  -V, --version
          Print version
```

### Examples

1. **Using default local server with API key:**
   ```bash
   jelly-dedup --api-key abc123def456
   ```

2. **Specifying a remote server:**
   ```bash
   jelly-dedup --jellyfin-url https://jellyfin.example.com --api-key abc123def456
   ```

3. **Removing path prefix for cleaner output:**
   ```bash
   jelly-dedup --api-key abc123def456 --path-prefix-to-remove /mnt/media
   ```

4. **Using environment variables:**
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
1. Scan all TV shows in your library
2. Display duplicate episodes found for each show
3. Provide a summary with:
   - Total episodes with duplicates
   - Total files marked for deletion
   - Estimated space savings in GB
4. Generate `rm` commands for each file to be deleted

**Note:** The tool does NOT delete files automatically. It only generates the commands for you to review and execute manually.

## Safety

- The tool is read-only and makes no modifications to your Jellyfin library or filesystem
- All file deletion commands are generated for manual review and execution
- File paths in deletion commands are properly escaped for shell safety
- Always review the generated commands before executing them
