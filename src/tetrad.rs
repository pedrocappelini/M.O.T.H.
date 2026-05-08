use crate::handler::Vm2Bits;

//TODO: Add a ROM loader for making testing and debugging easier.

#[derive(PartialEq)]
pub enum TetradState {
    Fetch,
    AddWaitLow {
        dest_reg: u8,
        reg_high: u8,
        val_high: u8,
    },
    AddWaitHigh {
        dest_reg: u8,
    },
    SubWaitLow {
        dest_reg: u8,
        reg_high: u8,
        val_high: u8,
    },
    SubWaitHigh {
        dest_reg: u8,
    },
    Halted,
}

pub struct Tetrad {
    pub memory: [u8; 256], //0x11111111
    pub reg: [u8; 8],
    pub stack: [u8; 8],
    pub pc: u8,
    pub index: u8,
    pub sp: u8,
    pub alu: Vm2Bits,

    pub state: TetradState,
    pub temp_ops_low: u8, // temp for holding the low bits of the result (in ADD and SUB ops)
}

impl Tetrad {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 256],
            reg: [0; 8],
            stack: [0; 8],
            pc: 0,
            index: 0,
            sp: 0,
            alu: Vm2Bits::new(),
            state: TetradState::Fetch,
            temp_ops_low: 0,
        }
    }

    pub fn cycle(&mut self) {
        match self.state {
            TetradState::Fetch => {
                println!("f");
                let instruction = self.memory[self.pc as usize];

                match instruction {
                    0b0000 => {
                        //Halt
                        self.state = TetradState::Halted
                    }
                    0b0001 => {
                        println!("assigned value");
                        //Assign
                        Self::pc_adder(self); // + 1
                        let reg_adr = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let val = self.memory[self.pc as usize];

                        self.reg[reg_adr as usize] = val;

                        Self::pc_adder(self); // + 3, ready for next instruction
                    }
                    0b0010 => {
                        //SUB rx, ry -----> rx
                        Self::pc_adder(self); // + 1
                        let reg_adr1 = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let reg_adr2 = self.memory[self.pc as usize];

                        let reg1_low = self.reg[reg_adr1 as usize] & 0b11;
                        let reg1_high = self.reg[reg_adr1 as usize] >> 2;
                        let reg2_low = self.reg[reg_adr2 as usize] & 0b11;
                        let reg2_high = self.reg[reg_adr2 as usize] >> 2;

                        self.alu.add = false;
                        self.alu.sbb = false;
                        self.alu.reg[0] = reg1_low;
                        self.alu.reg[1] = reg2_low;

                        self.state = TetradState::SubWaitLow {
                            dest_reg: reg_adr1,
                            reg_high: reg1_high,
                            val_high: reg2_high,
                        };

                        Self::pc_adder(self);
                    }
                    0b0011 => {
                        //ADD rx, ry ---> rx
                        Self::pc_adder(self); // + 1
                        let reg_adr1 = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let reg_adr2 = self.memory[self.pc as usize];

                        let reg1_low = self.reg[reg_adr1 as usize] & 0b11;
                        let reg1_high = self.reg[reg_adr1 as usize] >> 2;
                        let reg2_low = self.reg[reg_adr2 as usize] & 0b11;
                        let reg2_high = self.reg[reg_adr2 as usize] >> 2;

                        self.alu.add = true;
                        self.alu.adc = false;
                        self.alu.reg[0] = reg1_low;
                        self.alu.reg[1] = reg2_low;

                        self.state = TetradState::AddWaitLow {
                            dest_reg: reg_adr1,
                            reg_high: reg1_high,
                            val_high: reg2_high,
                        };

                        Self::pc_adder(self);
                    }
                    0b0100 => {
                        //add immediate (ADD r$, x)
                        Self::pc_adder(self); // + 1
                        let reg_adr = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let val = self.memory[self.pc as usize];

                        let reg_low = self.reg[reg_adr as usize] & 0b11;
                        let reg_high = self.reg[reg_adr as usize] >> 2;
                        let val_low = val & 0b11;
                        let val_high = val >> 2;

                        //setting for the ALU cycle
                        self.alu.add = true;
                        self.alu.adc = false; // reset carry
                        self.alu.reg[0] = reg_low;
                        self.alu.reg[1] = val_low;

                        self.state = TetradState::AddWaitLow {
                            dest_reg: reg_adr,
                            reg_high,
                            val_high,
                        };

                        Self::pc_adder(self); // + 3
                    }
                    _ => {}
                }
            }
            TetradState::AddWaitLow {
                dest_reg,
                reg_high,
                val_high,
            } => {
                println!("awl");
                self.alu.cycles();
                self.temp_ops_low = self.alu.reg[0];

                self.alu.adc = self.alu.reg[3] == 1;
                self.alu.reg[0] = reg_high;
                self.alu.reg[1] = val_high;

                self.state = TetradState::AddWaitHigh { dest_reg }
            }
            TetradState::SubWaitLow {
                //subwaitlow
                dest_reg,
                reg_high,
                val_high,
            } => {
                println!("swl");
                self.alu.cycles();
                self.temp_ops_low = self.alu.reg[0];

                self.alu.adc = self.alu.reg[3] == 1;
                self.alu.reg[0] = reg_high;
                self.alu.reg[1] = val_high;

                self.state = TetradState::SubWaitHigh { dest_reg }
            }
            TetradState::AddWaitHigh { dest_reg } => {
                //addwaithigh
                println!("awh");
                self.alu.cycles();
                self.reg[dest_reg as usize] = self.temp_ops_low | (self.alu.reg[0] << 2);

                self.reg[7] = self.alu.reg[3];

                println!("{:04b}", self.reg[dest_reg as usize]);
                println!("{:04b}", self.reg[7]);

                self.state = TetradState::Fetch;
            }
            TetradState::SubWaitHigh { dest_reg } => {
                println!("swh");
                self.alu.cycles();
                self.reg[dest_reg as usize] = self.temp_ops_low | (self.alu.reg[0] << 2);

                self.reg[7] = self.alu.reg[3];

                println!("{:04b}", self.reg[dest_reg as usize]);
                println!("{:04b}", self.reg[7]);

                self.state = TetradState::Fetch;
            }
            TetradState::Halted => {
                println!("halt");
            }
        }
    }

    pub fn pc_adder(&mut self) {
        let mut b = 1;
        while b != 0 {
            let carry = self.pc & b;
            self.pc = self.pc ^ b;
            b = carry << 1;
        }
    }
}
