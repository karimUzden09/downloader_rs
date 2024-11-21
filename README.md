# Downloader-rs
This is a simple file download utility written in Rust with progress bar.
## Install 
First, install Rust from https://www.rust-lang.org/tools/install
Then clone the repository and go to it
and run the 

```bash 
cargo build --release
```

then go to the `target/release` directory and you will see the binary there `downloader_rs`
## Usage

Run the command 
```shell 
downloader_rs --help
```
Then you will see supported commands

```shell 
Usage: downloader_rs.exe [OPTIONS] --url <URL>

Options:
      --url <URL>    URL  from where the file will be downloaded
  -p, --path <path>  The directory where the file will be downloaded
  -h, --help         Print help
  -V, --version      Print version
PS C:\Dev\Rust\downloader_rs\target\debug>

```
## Example
Example of using the program on Windows. Downloading Rust installer.
```shell
.\downloader_rs.exe --url https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -p C:\Downloads
```