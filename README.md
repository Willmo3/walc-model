# Walc-model

Walc-model is the core of the walc programming language. 

Walc is designed as a modular language which supports frontends in multiple languages. A TypeScript web platform (https://github.com/Willmo3/webwalc.ts) and a Rust CLI frontend (https://github.com/Willmo3/walc-frontend) have been constructed.

To enable its modular design, Walc has been published as a crate, so that different frontends can pull it in as a dependency. https://crates.io/crates/walc_model

## API
### `treewalk_interpreter::interpret`

### `bytecode_interpreter::interpret`

### `bytecode_generator::generate`
