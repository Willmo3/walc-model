# Walc-model

Walc-model is the core of the walc programming language. 

Walc is designed as a modular language which supports frontends in multiple languages. A TypeScript web platform (https://github.com/Willmo3/webwalc.ts) and a Rust CLI frontend (https://github.com/Willmo3/walc-frontend) have been constructed.

To enable its modular design, Walc has been published as a crate, so that different frontends can pull it in as a dependency. https://crates.io/crates/walc_model

## API
### `treewalk_interpreter::interpret`
This function executes a program AST without converting to bytecode.

#### Params:
- ast: `&ASTNode`
  - Tree to interpret.

#### Returns:
- Result of computation as a 64-bit float or a String error message.

### `bytecode_interpreter::interpret`
This function executes a collection of Walc bytecode. 
Note that a standard AST must first be translated to bytecode.

#### Params:
- Bytecode: `&Vec<U8>`
  - Stream of Walc bytecode.

#### Returns:
- Result of computation as 64-bit float, or a String error.

### `bytecode_generator::generate`
Given an AST, returns a stream of Walc bytecode. 
This function is intended to serve as a reference for other implementations.
In many cases, it will be ideal to write translation functions on the frontend -- for instance, in a WebAssembly environment, where data transports can dominate computation time, translating to bytecode before using walc-model may be ideal.

#### Params: 
- ast: `&ASTNode`
  - Syntax tree to translate.

#### Returns:
- Result of translation as a stream of bytes. Note that `bytecode_generate` performs no checking on the validity of the AST.

Note that other public fields exist, which may be viewed on the `crates.io` package explorer. This documentation merely serves to explain the key interface of `walc-model`. 