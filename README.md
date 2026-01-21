# mdif (My Disk Is Full)

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

### Method 1: Cargo Install
This is the easiest method if you have Rust installed. It downloads the package from crates.io and installs the binary to your path.

```bash
cargo install mdif
```

### Method 2: Build from Source
1.  Clone the repository:
    ```bash
    git clone https://github.com/temidaradev/mdif.git
    cd wthmdif
    ```
2.  Install locally:
    ```bash
    cargo install --path .
    ```

Once installed, you can run the app from anywhere:

```bash
mdif folder .
```

## Usage

### 1. Folder Analysis
Inspect a specific directory to list files and subdirectories sorted by size.

```bash
mdif folder /path/to/directory
```

**Output:**
-   **Path**: `/home/user/Projects`
-   **Total Size**: `45.2 GB`
-   **List**:
    -   `DIR  ████████████  target  (625.53 MB)`
    -   `FILE ░░░░░░        Cargo.lock    (12.34 KB)`

there is also a --detail flag but it wouldnt be really efficient to use in large folders. 

```bash
mdif folder --detail /path/to/directory
```

**Output:**
-   **Path**: `~/Documents/mdif/`
-   **Total Size**: `625.59 MB`
-   625.53 MB  DIR   "./target"
    522.00 MB  DIR   "./target/debug"
    262.70 MB  DIR   "./target/debug/deps"
    175.95 MB  DIR   "./target/debug/incremental"
    103.46 MB  DIR   "./target/release"
    89.88 MB  DIR   "./target/release/deps"
    59.72 MB  DIR   "./target/debug/build"
    36.48 MB  DIR   "./target/debug/incremental/wthmdif-38smkiffafzkx"
    35.99 MB  DIR   "./target/debug/incremental/wthmdif-2df0ema320six"
    35.29 MB  DIR   "./target/debug/incremental/wthmdif-27z9hk543syj4"
    23.54 MB  FILE  "./target/debug/deps/wthmdif-2c66bb95d694d2b8"
    23.54 MB  FILE  "./target/debug/wthmdif"

and this goes on....

### 2. Disk Usage
Get a summary of the disk partition hosting the specific path.

```bash
mdif disk /
```

an example for disk mounted on /mnt

```bash
mdif disk /mnt/your-disk/
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

## Stuff

-   **jwalk**: Parallel directory traversal.
-   **colored**: Terminal output coloring.
-   **indicatif**: Progress bars and status indicators.
-   **clap**: Command-line argument parsing.

## License

MIT
