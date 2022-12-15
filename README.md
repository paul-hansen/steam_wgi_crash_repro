A minimal reproduction of a crash when using Windows Gaming Input with Rust and running the program through Steam.

### Steps to reproduce

1. Install rust if you haven't https://rustup.rs 
2. `git clone https://github.com/paul-hansen/steam_wgi_crash_repro.git`
3. `cd steam_wgi_crash_repro`
4. Connect a controller (doesn't crash without one)
5. `cargo run --release`
6. Observe that it does not crash and prints the name of any connected controllers to the console.
7. In Steam click `Games -> Add a Non-steam Game to My Library...`
8. Click "browse" and select the executable located at `./steam_wgi_crash_repro/target/release/steam_wgi_crash_repro.exe`
9. Run the program from your steam library.
10. The program will crash and exit shortly after starting.
