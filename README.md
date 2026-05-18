# sway-autotiler

An ultra-lightweight, event-driven autotiling daemon for the Sway window manager written in Rust. It automatically manages window split orientations dynamically based on current window dimensions, bringing a seamless `bspwm`-style ("longest side") tiling experience to Sway.

Designed from the ground up to replace heavy Python or shell-based tiling scripts, this binary runs with practically zero CPU overhead and keeps a microscopic memory footprint (~1–2 MB RAM)—making it perfect for resource-constrained systems or minimal hardware setups.

## Features

* **Zero-Overhead Event Loop:** Uses native Sway IPC event subscriptions (`swayipc`). The daemon idles silently and only wakes up when a window focus event actually occurs.
* **Smart Orientation Tiling:** Automatically detects window layout geometry. Wide containers split horizontally (side-by-side); tall containers split vertically (stacked).
* **Floating Aware:** Automatically ignores floating windows to prevent disrupting your manual layer layouts.
* **Resource Optimized:** Written in pure Rust with zero shell subshell calls or external scripting dependencies.

## Installation & Compilation

### 1. Prerequisites

Ensure you have the Rust toolchain installed. If not, install it via your package manager or rustup:

```bash
# For Arch Linux
sudo pacman -S rust cargo

```

### 2. Build the Optimized Binary

Clone or navigate into your project repository directory and compile using the optimized release configuration:

```bash
cd sway_autotiler
cargo build --release

```

The resulting optimized binary will be located at `target/release/sway_autotiler`.

### 3. Binary Size Optimization (Optional)

To reduce the binary size to its bare minimum and strip out unnecessary symbols, verify your `Cargo.toml` contains the following release profile configuration:

```toml
[profile.release]
opt-level = "z"     # Optimize explicitly for size
lto = true          # Enable Link Time Optimization (LTO)
codegen-units = 1   # Reduce parallel code generation units for better size optimization
strip = true        # Automatically strip debug symbols from the final binary
panic = "abort"     # Remove heavy panic unwinding mechanisms

```

### 4. Move to System Path

Copy the compiled release binary into your local executable path:

```bash
sudo cp target/release/sway_autotiler /usr/local/bin/

```

## Configuration

To have the autotiler automatically initialize alongside your window manager session, append the following execution rule to your Sway configuration file:

```text
# ~/.config/sway/config

# Launch the custom Rust autotiler daemon
exec_always --no-startup-id /usr/local/bin/sway_autotiler

```

Reload your Sway configuration using `swaymsg reload` or restart your session to apply the changes.

## License

This project is open-source and available under the [MIT License](https://www.google.com/search?q=LICENSE).
