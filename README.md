# THE MOTH

### Macro. Octad. Tetrad. Handler.

THE MOTH is an intentionally inefficient, nested virtual machine architecture. It consists of a 2-bit ALU running inside a 4-bit VM, which operates inside an 8-bit VM, all enclosed within a 16-bit VM.

None of the individual VMs can perform arithmetic on their own. They function as a single unit by routing all addition and subtraction through the central ALU, and through other VMs if necessary.

**This project is currently under development.**

---

## The Handler

The Handler acts as the system's central 2-bit ALU.

### State Flags

- **add:** `true` for an ADD instruction, `false` for a SUB instruction.
- **adc:** `true` if performing an addition with carry.
- **sbb:** `true` if performing a subtraction with borrow.

### Registers

- **reg[0]:** Accumulator. Always used to store the result of an operation.
- **reg[1]:** Operand. Used to store the second value in operations that require two registers. (Does nothing during immediate operations).
- **reg[2]:** Reserved. Currently unused.
- **reg[3]:** Carry/Borrow. Always used to store the carry or borrow of an operation, if one exists.

---

## The Tetrad (4-Bit VM)

Tetrad instructions consist of two or three nibbles. The first nibble is always the OPCODE. The second and third nibbles (`X` and `Y`) represent either a register address or an immediate value.

### Instruction Set (So Far)

| Opcode  | Mnemonic | Description                                                                               |
| :------ | :------- | :---------------------------------------------------------------------------------------- |
| `0x0`   | HALT     | Stops the Program Counter and halts the Tetrad.                                           |
| `0x1XY` | ASSIGN   | Assigns the immediate value `Y` to register `X`.                                          |
| `0x2XY` | SUB      | Subtracts the value in register `Y` from register `X`. Stores the result in register `X`. |
| `0x3XY` | ADD      | Adds the value in register `Y` to register `X`. Stores the result in register `X`.        |
| `0x4XY` | ADI      | Adds the immediate value `Y` to register `X`. Stores the result in register `X`.          |

**_More will be added soon._**

### Registers

Currently, the only register with a specific hardware rule is **reg[0x7]**. It is strictly reserved to store the carry or borrow output of Tetrad operations.
