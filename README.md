# Walc-model

Walc-model is the core of the walc programming language. 

Walc is designed as a modular language which supports frontends in multiple languages. A TypeScript web platform (https://github.com/Willmo3/webwalc.ts) and a Rust CLI frontend (https://github.com/Willmo3/walc-frontend) have been constructed.

To enable its modular design, Walc has been published as a crate, so that different frontends can pull it in as a dependency. https://crates.io/crates/walc_model

## API
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

## Grammar:
start = assign
assign = IDENTIFIER EQUALS add
add = mult ((PLUS | MINUS) mult)*
mult = exp ((STAR | SLASH) exp)*
exp = [atom (DOUBLESTAR)] exp
atom = LEFT_PARENS start RIGHT_PARENS
     | NUMBER_LITERAL

## Code format:
The Walc interpreter supports both AST and Bytecode formats for execution.

### Tree format:
To bolster compatibility, Walc-model ASTs are serializable and deserializable.
We have tested serialization under a json scheme. 
* Binary operands should be capitalized and named after their operation. They should have `left` and `right` fields containing their subtrees.
* Number nodes have a `value` field containing a decimal representation of the number they contain.

#### Example:
3.1 - 2: {"Subtract":{"left":{"Number":{"value": 3.1}},"right":{"Number":{"value": 2}}}}

### Bytecode format:
* Each instruction comprises a single byte.
* Operands are all 8 bytes in size.

#### Operations:
* 0x0: push
    * Push an 8-byte value onto the stack.
* 0x1: add
    * Pop two 8-byte values from the stack, add them, and push the total onto the stack.
* 0x2: subtract
    * Pop two 8-byte values from the stack, subtract them, and push the result onto the stack.
* 0x3: multiply
    * Pop two 8-byte values from the stack, multiply them, and push the result onto the stack.
* 0x4: divide
    * Pop two 8-byte values from the stack, divide the second one by the first, and push the result onto the stack.
* 0x5: exponentiate
    * Pop two 8-byte values from the stack, raise the second one to the first one's power, and push the result onto the stack.

#### Instruction structure:
* Byte one: instruction code.
* Bytes two-nine (optional): operand.

## Design notes

### Including Frontend
Early in Walc's development, Walc contained only the execution backend of the interpreter. This is based on the strategy planned for the Twoville language's Rust interpreter, which would maintain a JavaScript lexer and parser to ease direct manipulation of the AST. However, as it became clear that Walc might use static analysis passes in the "middle end," this clean decoupling began to vanish. 

A key aspect of separating Walc's frontend from its backend was to allow bytecode to be transferred through WebAssembly's memory, rather than a larger tree-based representation. However, many static analyses (such as typechecking) require access to the tree representation. Therefore, extending Walc required taking one of three paths:
1. Requiring developers to run their own tree-based analyses, leaving the interpreter to perform only optimizations viable on linear bytecode. 
2. Transporting an AST to the Walc model, leaving both the frontend and backend with independent tree representations of the program.
3. Integrating the frontend into the core Walc interpreter.

Option on devastated the extensibility of the Walc language. Option two tightly coupled the back and front end across a language boundary. Only option three remained.

