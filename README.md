# wthmdif (Why The Hell My Disk Is Full?)

A terminal-based disk usage analyzer written in Rust. It helps identify storage consumption with a clear and intuitive interface.

## Features

-   **Fast Scanning**: Uses efficient filesystem traversal to scan directories.
-   **Visual Feedback**:
    -   **Colored Output**: File sizes are color-coded (Red for GB/TB, Yellow for MB) for quick recognition.
    -   **Detailed Reporting**: Progress indicators for directory scanning and visual usage bars for disk analysis.
    -   **Relative Size Bars**: Visual bars next to files/folders showing their size relative to the largest item in the directory.
-   **Folder Inspection**: detailed breakdown of specific directories.
-   **Disk Analysis**: Summary of total disk usage, including free vs. used space and a visual usage indicator.

## Installation

### Method 1: Cargo Install (Recommended)
This is the easiest method if you have Rust installed. It downloads the package from crates.io and installs the binary to your path.

```bash
cargo install wthmdif
```

### Method 2: Build from Source
1.  Clone the repository:
    ```bash
    git clone https://github.com/temidaradev/wthmdif.git
    cd wthmdif
    ```
2.  Install locally:
    ```bash
    cargo install --path .
    ```

Once installed, you can run the app from anywhere:

```bash
wthmdif folder .
```

## Usage

### 1. Folder Analysis
Inspect a specific directory to list files and subdirectories sorted by size.

```bash
wthmdif folder /path/to/directory
```

**Output:**
-   **Path**: `/home/user/Projects`
-   **Total Size**: `45.2 GB`
-   **List**:
    -   `DIR  ████████████  node_modules  (2.5 GB)`
    -   `FILE ░░░░░░        backup.zip    (500 MB)`

### 2. Disk Usage
Get a summary of the disk partition hosting the specific path.

```bash
wthmdif disk /
```

**Visuals:**
-   Displays Total, Used, and Free space.
-   Shows a colored usage bar (Green < 60%, Yellow < 85%, Red > 85%).

## Commands

| Command | Arguments | Description |
| :--- | :--- | :--- |
| `folder` | `<path>` | Recursively calculates size of the given path and lists immediate children sorted by size. |
| `disk` | `<path>` | Shows filesystem usage stats (total/used/free) for the partition containing the path. |
| `--help` | | Shows the help message. |

## Tech Stack

-   **Rust**: Performance and safety.
-   **jwalk**: Parallel directory traversal.
-   **colored**: Terminal output coloring.
-   **indicatif**: Progress bars and status indicators.
-   **clap**: Command-line argument parsing.

## License

MIT
