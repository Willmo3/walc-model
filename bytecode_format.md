# Walc format
This file describes the bytecode format for the walc interpreter.
* Each instruction comprises a single byte. 
* Operands are all 8 bytes in size. 

## Operations:
* 0x0: push
  * Push an 8-byte value onto the stack.
* 0x1: add
  * Pop two 8-byte values from the stack, add them, and push the total onto the stack.
* 0x2: subtract
  * Pop two 8-byte values from the stack, subtract them, and push the result onto the stack.
* 0x3: multiply
  * Pop two 8-byte values from the stack, multiply them, and push the result onto the stack.
* 0x4: divide
  * Pop two 8-byte values from the stack, divide them, and push the result onto the stack.

## Instruction structure:
* Byte one: instruction code.
* Bytes two-nine (optional): operand.