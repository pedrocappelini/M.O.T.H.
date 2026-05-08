# THE MOTH

### Macro. Octad. Tetrad. Handler.

---

It is a 2bit ALU, inside of a 4bit VM, inside of a 8bitVM, inside of a 16bitVM.

None of the VMs are able to make an sum or a subtraction by themselves. However, they do work as a single VM, as long as they are able to communicate with the ALU.

It is, in fact, made to be inefficient. It also has zero "+" or "-"

---

**It is still under development.**

---

##Handler

The handler is just an ALU.

#It has three bools:

- add -> true if it receives and ADD instruction, false if it receives an SUB instruction.
- adc -> true if it has to make a sum with carry.
- sbb -> true if it has to make a subtraction with borrow.

#And four registers:

- reg[0] -> Will always be the register used to store the result of the operations.
- reg[1] -> Will be used on operations that use two registers. For immediate operations, it will do nothing.
- reg[2] -> For now, it does nothing.
- reg[3] -> Will always be used to store either the carry of the borrow of operations (if it exists).

---

##Tetrad Instructions _(so far)_:

They are made of two or three nibbles.

First nibble will always be the OPCODE.
Second and third nibbles will either act as a address or an immediate value.

#Instructions (so far)

- (0x0) ---> The halt instruction. It halts the PC and stops the Tetrad.
- (0x1XY) ---> The assign instruction. It assings to the register [X] the value Y.
- (0x2XY) ---> SUB. Subtracts the value of register Y from the value in register X. Stores the result in register X.
- (0x3XY) ---> ADD. Adds the value of register Y to the value in register X. Stores the result in register X.
- (0x4XY) ---> ADI. Adds an immediate Y to the value in register X. Stores in register X.

#Registers

- For now, the only register with a rule to it is register [0x7]. It will always be used to store the carry or the borrow of operations.
