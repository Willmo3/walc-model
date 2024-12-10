# Walc-model

Walc-model is the core of the walc programming environment for calculations. Which is a fancy way of saying "an overengineered calculator". But only for now!

Walc is designed as a modular language which supports frontends in multiple languages. A TypeScript frontend is underway (https://github.com/Willmo3/webwalc.ts) and a Rust terminal frontend has also been constructed (https://github.com/Willmo3/walc-frontend).

Walc is being implemented to test the implications of creating a Rust WebAssembly based web programming environment.

To enable its modular design, Walc has been published as a crate, so that different frontends can pull it in as a dependency. https://crates.io/crates/walc_model
