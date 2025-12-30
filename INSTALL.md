# Installation Guide for RDNSx

## Installing Rust on Windows

### Quick Installation (GNU Toolchain - Recommended)

1. **Download rustup**:
   - Visit https://rustup.rs/
   - Download `rustup-init.exe` for Windows
   - Or run in PowerShell:
   ```powershell
   Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
   ```

2. **Run the installer**:
   ```powershell
   .\rustup-init.exe
   ```

3. **When prompted**, you have two options:
   - **Option A (GNU - Recommended)**: Type `2` to customize, then select `x86_64-pc-windows-gnu` as the default toolchain
   - **Option B (MSVC)**: Type `y` to continue (requires Visual C++ build tools)

4. **Restart PowerShell** or run:
   ```powershell
   $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
   ```

5. **Verify installation**:
   ```powershell
   cargo --version
   rustc --version
   ```

### Alternative: Install GNU Toolchain Explicitly

If you already have rustup but need the GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Building RDNSx

Once Rust is installed:

```powershell
# Navigate to project directory
cd R:\RDNSx

# Build in release mode
cargo build --release

# The binary will be at:
# target\release\rdnsx.exe
```

### Troubleshooting

**Problem**: `cargo: command not found`
- **Solution**: Close and reopen PowerShell, or restart your terminal
- Verify PATH: `$env:PATH -split ';' | Select-String cargo`

**Problem**: Linker errors during build
- **Solution**: Install the GNU toolchain: `rustup toolchain install stable-x86_64-pc-windows-gnu`

**Problem**: Visual C++ build tools required
- **Solution**: Either install Visual Studio Build Tools, or use the GNU toolchain (option 2 during rustup-init)
