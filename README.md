A minimal reproduction of a crash when using Windows Gaming Input with Rust and running the program through steam.

### Steps to reproduce

1. Install rust if you haven't https://rustup.rs
2. `git clone https://github.com/paul-hansen/steam_wgi_crash_repro.git`
3. `cd steam_wgi_crash_repro`
4. `cargo run --release`
5. Observe that it does not crash and prints the name of any connected controllers to the console.
6. In steam click `Games -> Add a Non-steam Game to My Library...`
7. Click "browse" and select the executable located at `./steam_wgi_crash_repro/target/release/steam_wgi_crash_repro.exe`
8. Run the program from your steam library.
9. The program will crash and exit shortly after starting.
