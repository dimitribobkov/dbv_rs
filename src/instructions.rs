#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instructions{
                    // ______________________________________________________________________________
                    // |VAL |  EXAMPLE USAGE          | Description
                    // |____|_________________________|______________________________________________
    HLT = 0x0,      // | 0  |  HLT                    | Halts the program
    PSH = 0x1,      // | 1  |  PSH val                | Push val to stack
    POP = 0x2,      // | 2  |  POP reg                | Remove from stack and reg to the value
    SET = 0x3,      // | 3  |  SET reg val            | Set a register
    MOV = 0x4,      // | 4  |  MOV reg_a reg_b        | Moves value from register a to register b
    IF = 0x5,       // | 5  |  IF reg val addr        | If reg == val, goto addr
    IFN = 0x6,      // | 6  |  IFN reg val addr       | If reg != val, goto addr
    JMP = 0x7,      // | 7  |  JMP addr               | Set IP to addr
    SLT = 0x8,      // | 8  |  SLT reg_a reg_b reg_c  | Set reg_a to 1 if reg_b < reg_c
    ADD = 0x9,      // | 9  |  ADD reg_a reg_b reg_c  | Adds two registers, storing the result in a register [reg_a = reg_b + reg_c]
    SUB = 0xA,      // | 10 |  SUB reg_a reg_b reg_c  | Subtracts two registers, storing the result in a register [reg_a = reg_b - reg_c]
    MUL = 0xB,      // | 11 |  MUL reg_a reg_b reg_c  | Multiplies two registers, storing the result in a register [reg_a = reg_b * reg_c]
    DIV = 0xC,      // | 12 |  DIV reg_a reg_b reg_c  | Divides two registers, storing the result in a register [reg_a = reg_b / reg_c]
    ADDI = 0xD,     // | 13 |  ADDI reg_a reg_b val   | Adds a register and a value, storing the result in a register [reg_a = reg_b + val]
    SUBI = 0xE,     // | 14 |  SUBI reg_a reg_b val   | Subtracts a register and a value, storing the result in a register [reg_a = reg_b - val]
    MULI = 0xF,     // | 15 |  MULI reg_a reg_b val   | Multiplies a register and a value, storing the result in a register [reg_a = reg_b * val]
    DIVI = 0x10,    // | 16 |  DIVI reg_a reg_b val   | Divides a register and a value, storing the result in a register [reg_a = reg_b / val]
    AND = 0x11,     // | 17 |  AND reg_a reg_b reg_c  | Performs an AND operation on two registers, storing the result in a register [reg_a = reg_b & reg_c]
    OR = 0x12,      // | 18 |  OR reg_a reg_b reg_c   | Performs an OR operation on two registers, storing the result in a register [reg_a = reg_b | reg_c]
    SL = 0x13,      // | 19 |  SL reg_a reg_b reg_c   | Performs an SHIFT LEFT operation on two registers, storing the result in a register [reg_a = reg_b << reg_c]
    SR = 0x14,      // | 20 |  SR reg_a reg_b reg_c   | Performs an SHIFT RIGHT operation on two registers, storing the result in a register [reg_a = reg_b >> reg_c]
    SLI = 0x15,     // | 21 |  SLI reg_a reg_b val    | Performs an SHIFT LEFT operation on a register and value, storing the result in a register [reg_a = reg_b << val]
    SRI = 0x16,     // | 22 |  SRI reg_a reg_b val    | Performs an SHIFT RIGHT operation on a register and value, storing the result in a register [reg_a = reg_b >> val]
    SAL = 0x17,     // | 23 |  SAL reg                | Performs an SHIFT LEFT operation on a register, storing the result in a register [reg <<= 1]
    SAR = 0x18,     // | 24 |  SAR reg                | Performs an SHIFT RIGHT operation on a register, storing the result in a register [reg >>= 1]
    LD = 0x19,      // | 25 |  LD reg mem_addr        | Loads data (uint8_t) from a memory address (uint16_t), storing in a register
    SD = 0x1A,      // | 26 |  SD mem_addr reg        | Stores data (uint8_t) from a register into memory  at the given address (uint16_t)
    LDHW = 0x1B,    // | 27 |  LDHW reg mem_addr      | Loads data (uint16_t) from a memory address (uint16_t), storing in a register
    SDHW = 0x1C,    // | 28 |  SDHW mem_addr reg      | Stores data (uint16_t) from a register into memory  at the given address (uint16_t)
    LDW = 0x1D,     // | 29 |  LDW reg mem_addr       | Loads data (uint32_t) from a memory address (uint16_t), storing in a register
    SDW = 0x1E,     // | 30 |  SDW mem_addr reg       | Stores data (uint32_t) from a register into memory  at the given address (uint16_t)
    JNZ = 0x1F,     // | 31 |  JNZ reg addr           | Jumps to addr if reg is not 0
}

// Convert from a regular opcode integer into an enum
impl<T> From<T> for Instructions where T: Into<i32>{
    fn from(val: T) -> Self{
        let val = val.into();
        match val{
            0x0 => Self::HLT,
            0x1 => Self::PSH,
            0x2 => Self::POP,
            0x3 => Self::SET,
            0x4 => Self::MOV,
            0x5 => Self::IF,
            0x6 => Self::IFN,
            0x7 => Self::JMP,
            0x8 => Self::SLT,
            0x9 => Self::ADD,

            0xA => Self::SUB,
            0xB => Self::MUL,
            0xC => Self::DIV,
            0xD => Self::ADDI,
            0xE => Self::SUBI,
            0xF => Self::MULI,

            0x10 => Self::DIVI,
            0x11 => Self::AND,
            0x12 => Self::OR,
            0x13 => Self::SL,
            0x14 => Self::SR,
            0x15 => Self::SLI,
            0x16 => Self::SRI,
            0x17 => Self::SAL,
            0x18 => Self::SAR,
            0x19 => Self::LD,

            0x1A => Self::SD,
            0x1B => Self::LDHW,
            0x1C => Self::SDHW,
            0x1D => Self::LDW,
            0x1E => Self::SDW,
            0x1F => Self::JNZ,

            _ => panic!("Error - value does not correspond to an instruction!")
        }
    }
}
                        // Instruction, param count length in bytes - used by the VM to determine jumps and different size params (like 32 bit and so on)
pub const OPCODE_TABLE: [(Instructions, u8); 32] = [
    (Instructions::HLT, 0),
    (Instructions::PSH, 4),
    (Instructions::POP, 1),
    (Instructions::SET, 5),
    (Instructions::MOV, 2),
    (Instructions::IF, 7),
    (Instructions::IFN, 7),
    (Instructions::JMP, 2),
    (Instructions::SLT, 3),
    (Instructions::ADD, 3),
    (Instructions::SUB, 3),
    (Instructions::MUL, 3),
    (Instructions::DIV, 3),
    (Instructions::ADDI, 6),
    (Instructions::SUBI, 6),
    (Instructions::MULI, 6),
    (Instructions::DIVI, 6),
    (Instructions::AND, 3),
    (Instructions::OR, 3),
    (Instructions::SL, 3),
    (Instructions::SR, 3),
    (Instructions::SLI, 6),
    (Instructions::SRI, 6),
    (Instructions::SAL, 1),
    (Instructions::SAR, 1),
    (Instructions::LD, 3),
    (Instructions::SD, 3),
    (Instructions::LDHW, 3),
    (Instructions::SDHW, 3),
    (Instructions::LDW, 3),
    (Instructions::SDW, 3),
    (Instructions::JNZ, 3),
];