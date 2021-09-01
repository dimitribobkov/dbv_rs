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
    SLT = 0x5,      // | 5  |  SLT reg_a reg_b reg_c  | Set reg_a to 1 if reg_b < reg_c


    ADD = 0x6,      // | 6  |  ADD reg_a reg_b reg_c  | Adds two registers, storing the result in a register [reg_a = reg_b + reg_c]
    SUB = 0x7,      // | 7  |  SUB reg_a reg_b reg_c  | Subtracts two registers, storing the result in a register [reg_a = reg_b - reg_c]
    MUL = 0x8,      // | 8  |  MUL reg_a reg_b reg_c  | Multiplies two registers, storing the result in a register [reg_a = reg_b * reg_c]
    DIV = 0x9,      // | 9  |  DIV reg_a reg_b reg_c  | Divides two registers, storing the result in a register [reg_a = reg_b / reg_c]


    ADDI = 0xA,     // | 10 |  ADDI reg_a reg_b val   | Adds a register and a value, storing the result in a register [reg_a = reg_b + val]
    SUBI = 0xB,     // | 11 |  SUBI reg_a reg_b val   | Subtracts a register and a value, storing the result in a register [reg_a = reg_b - val]
    MULI = 0xC,     // | 12 |  MULI reg_a reg_b val   | Multiplies a register and a value, storing the result in a register [reg_a = reg_b * val]
    DIVI = 0xD,     // | 13 |  DIVI reg_a reg_b val   | Divides a register and a value, storing the result in a register [reg_a = reg_b / val]


    AND = 0xE,      // | 14 |  AND reg_a reg_b reg_c  | Performs an AND operation on two registers, storing the result in a register [reg_a = reg_b & reg_c]
    OR = 0xF,       // | 15 |  OR reg_a reg_b reg_c   | Performs an OR operation on two registers, storing the result in a register [reg_a = reg_b | reg_c]
    SL = 0x10,      // | 16 |  SL reg_a reg_b reg_c   | Performs an SHIFT LEFT operation on two registers, storing the result in a register [reg_a = reg_b << reg_c]
    SR = 0x11,      // | 17 |  SR reg_a reg_b reg_c   | Performs an SHIFT RIGHT operation on two registers, storing the result in a register [reg_a = reg_b >> reg_c]
    SLI = 0x12,     // | 18 |  SLI reg_a reg_b val    | Performs an SHIFT LEFT operation on a register and value, storing the result in a register [reg_a = reg_b << val]
    SRI = 0x13,     // | 19 |  SRI reg_a reg_b val    | Performs an SHIFT RIGHT operation on a register and value, storing the result in a register [reg_a = reg_b >> val]
    SAL = 0x14,     // | 20 |  SAL reg                | Performs an SHIFT LEFT operation on a register, storing the result in a register [reg <<= 1]
    SAR = 0x15,     // | 21 |  SAR reg                | Performs an SHIFT RIGHT operation on a register, storing the result in a register [reg >>= 1]


    LD = 0x16,      // | 22 |  LD reg mem_addr        | Loads data (uint8_t) from a memory address (uint16_t), storing in a register
    SD = 0x17,      // | 23 |  SD mem_addr reg        | Stores data (uint8_t) from a register into memory  at the given address (uint16_t)
    LDHW = 0x18,    // | 24 |  LDHW reg mem_addr      | Loads data (uint16_t) from a memory address (uint16_t), storing in a register
    SDHW = 0x19,    // | 25 |  SDHW mem_addr reg      | Stores data (uint16_t) from a register into memory  at the given address (uint16_t)
    LDW = 0x1A,     // | 26 |  LDW reg mem_addr       | Loads data (uint32_t) from a memory address (uint16_t), storing in a register
    SDW = 0x1B,     // | 27 |  SDW mem_addr reg       | Stores data (uint32_t) from a register into memory  at the given address (uint16_t)


    IF = 0x1C,      // | 28 |  IF reg val addr        | If reg == val, goto addr
    IFN = 0x1D,     // | 29 |  IFN reg val addr       | If reg != val, goto addr
    JMP = 0x1E,     // | 30 |  JMP addr               | Relative jump to addr
    JNZ = 0x1F,     // | 31 |  JNZ reg addr           | Jumps to addr if reg is not 0

    IFR = 0x20,     // | 32 |  IFR reg val rel_addr   | Relative jump to rel_addr if reg == val
    IFNR = 0x21,    // | 32 |  IFNR reg val rel_addr  | Relative jump to rel_addr if reg != val
    JMPR = 0x22,    // | 33 |  JMPR rel_addr          | Relative jump to addr
    JNZR = 0x23,    // | 34 |  JNZR reg rel_addr      | Relative jump to addr if reg is not 0
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
            0x5 => Self::SLT,

            0x6 => Self::ADD,
            0x7 => Self::SUB,
            0x8 => Self::MUL,
            0x9 => Self::DIV,
            0xA => Self::ADDI,
            0xB => Self::SUBI,
            0xC => Self::MULI,

            0xD => Self::DIVI,
            0xE => Self::AND,
            0xF => Self::OR,
            0x10 => Self::SL,
            0x11 => Self::SR,
            0x12 => Self::SLI,
            0x13 => Self::SRI,
            0x14 => Self::SAL,
            0x15 => Self::SAR,
            0x16 => Self::LD,

            0x17 => Self::SD,
            0x18 => Self::LDHW,
            0x19 => Self::SDHW,
            0x1A => Self::LDW,
            0x1B => Self::SDW,

            0x1C => Self::IF,
            0x1D => Self::IFN,
            0x1E => Self::JMP,
            0x1F => Self::JNZ,

            0x20 => Self::IFR,
            0x21 => Self::IFNR,
            0x22 => Self::JMPR,
            0x23 => Self::JNZR,

            _ => panic!("Error - value does not correspond to an instruction!")
        }
    }
}
                        // Instruction, param count length in bytes - used by the VM to determine jumps and different size params (like 32 bit and so on)
pub const OPCODE_TABLE: [(Instructions, u8); 36] = [
    (Instructions::HLT, 0),
    (Instructions::PSH, 4),
    (Instructions::POP, 1),
    (Instructions::SET, 5),
    (Instructions::MOV, 2),
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

    (Instructions::IF, 7),
    (Instructions::IFN, 7),
    (Instructions::JMP, 2),
    (Instructions::JNZ, 3),

    (Instructions::IFR, 6),
    (Instructions::IFNR, 6),
    (Instructions::JMPR, 1),
    (Instructions::JNZR, 2),
];