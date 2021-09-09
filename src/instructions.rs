#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instructions{
                    // ______________________________________________________________________________
                    // |VAL |  EXAMPLE USAGE                    | Description
                    // |____|___________________________________|____________________________________
    HLT = 0x0,      // | 0  |  HLT                              | Halts the program
    PSH = 0x1,      // | 1  |  PSH val                          | Push val to stack
    POP = 0x2,      // | 2  |  POP reg                          | Remove from stack and reg to the value
    SET = 0x3,      // | 3  |  SET reg val                      | Set a register
    MOV = 0x4,      // | 4  |  MOV reg_a reg_b                  | Moves value from register a to register b
    SLT = 0x5,      // | 5  |  SLT reg_a reg_b reg_c            | Set reg_a to 1 if reg_b < reg_c

    // Arithmetic

    ADD = 0x6,      // | 6  |  ADD reg_a reg_b reg_c            | Adds two registers, storing the result in a register [reg_a = reg_b + reg_c]
    SUB = 0x7,      // | 7  |  SUB reg_a reg_b reg_c            | Subtracts two registers, storing the result in a register [reg_a = reg_b - reg_c]
    MUL = 0x8,      // | 8  |  MUL reg_a reg_b reg_c            | Multiplies two registers, storing the result in a register [reg_a = reg_b * reg_c]
    DIV = 0x9,      // | 9  |  DIV reg_a reg_b reg_c            | Divides two registers, storing the result in a register [reg_a = reg_b / reg_c]


    ADDI = 0xA,     // | 10 |  ADDI reg_a reg_b val             | Adds a register and a value, storing the result in a register [reg_a = reg_b + val]
    SUBI = 0xB,     // | 11 |  SUBI reg_a reg_b val             | Subtracts a register and a value, storing the result in a register [reg_a = reg_b - val]
    MULI = 0xC,     // | 12 |  MULI reg_a reg_b val             | Multiplies a register and a value, storing the result in a register [reg_a = reg_b * val]
    DIVI = 0xD,     // | 13 |  DIVI reg_a reg_b val             | Divides a register and a value, storing the result in a register [reg_a = reg_b / val]


    AND = 0xE,      // | 14 |  AND reg_a reg_b reg_c            | Performs an AND operation on two registers, storing the result in a register [reg_a = reg_b & reg_c]
    OR = 0xF,       // | 15 |  OR reg_a reg_b reg_c             | Performs an OR operation on two registers, storing the result in a register [reg_a = reg_b | reg_c]
    SL = 0x10,      // | 16 |  SL reg_a reg_b reg_c             | Performs an SHIFT LEFT operation on two registers, storing the result in a register [reg_a = reg_b << reg_c]
    SR = 0x11,      // | 17 |  SR reg_a reg_b reg_c             | Performs an SHIFT RIGHT operation on two registers, storing the result in a register [reg_a = reg_b >> reg_c]
    SLI = 0x12,     // | 18 |  SLI reg_a reg_b val              | Performs an SHIFT LEFT operation on a register and value, storing the result in a register [reg_a = reg_b << val]
    SRI = 0x13,     // | 19 |  SRI reg_a reg_b val              | Performs an SHIFT RIGHT operation on a register and value, storing the result in a register [reg_a = reg_b >> val]
    SAL = 0x14,     // | 20 |  SAL reg                          | Performs an SHIFT LEFT operation on a register, storing the result in a register [reg <<= 1]
    SAR = 0x15,     // | 21 |  SAR reg                          | Performs an SHIFT RIGHT operation on a register, storing the result in a register [reg >>= 1]

    // Memory Instructions

    LD = 0x16,      // | 22 |  LD reg mem_addr                  | Loads data (uint8_t) from a memory address (uint16_t), storing in a register
    SD = 0x17,      // | 23 |  SD mem_addr reg                  | Stores data (uint8_t) from a register into memory  at the given address (uint16_t)
    LDHW = 0x18,    // | 24 |  LDHW reg mem_addr                | Loads data (uint16_t) from a memory address (uint16_t), storing in a register
    SDHW = 0x19,    // | 25 |  SDHW mem_addr reg                | Stores data (uint16_t) from a register into memory  at the given address (uint16_t)
    LDW = 0x1A,     // | 26 |  LDW reg mem_addr                 | Loads data (uint32_t) from a memory address (uint16_t), storing in a register
    SDW = 0x1B,     // | 27 |  SDW mem_addr reg                 | Stores data (uint32_t) from a register into memory  at the given address (uint16_t)

    // Branching instructions

    IF = 0x1C,      // | 28 |  IF reg val addr                  | If reg == val, goto addr
    IFN = 0x1D,     // | 29 |  IFN reg val addr                 | If reg != val, goto addr
    JMP = 0x1E,     // | 30 |  JMP addr                         | Relative jump to addr
    JNZ = 0x1F,     // | 31 |  JNZ reg addr                     | Jumps to addr if reg is not 0

    IFR = 0x20,     // | 32 |  IFR reg val rel_addr             | Relative jump to rel_addr if reg == val
    IFNR = 0x21,    // | 33 |  IFNR reg val rel_addr            | Relative jump to rel_addr if reg != val
    JMPR = 0x22,    // | 34 |  JMPR rel_addr                    | Relative jump to addr
    JNZR = 0x23,    // | 35 |  JNZR reg rel_addr                | Relative jump to addr if reg is not 0


    RET = 0x24,     // | 36 |  RET                              | Return from a jump


    // _______________________________________________________________________________________________
    // *Floating point instructions - these use a different set of registers to perform calculations*
    //
    // We won't have any bit operations as it would mess up the floating point calculation
    // _______________________________________________________________________________________________


    SETF = 0x25,    // | 37 |  SETF f_reg val                   | Set a fp reg to val (also an f) (FP operation)
    MOVF = 0x26,    // | 38 |  MOVF f_reg_a f_reg_b             | Move f_reg_a to f_reg_b (FP operation)
    
    // These instructions allow you to move data between integer and floating point registers

    MOVFI = 0x27,   // | 39 |  MOVFI f_reg_a reg_b              | Move f_reg_a to reg_b (warning - this will round the float to the nearest integer) (FP operation)
    MOVIF = 0x28,   // | 40 |  MOVIF reg_a f_reg_b              | Move reg_a to f_reg_b (FP operation)

    // Arithemtic instructions

    ADDF = 0x29,    // | 41 |  ADDF f_reg_a f_reg_b f_reg_c     | add f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    SUBF = 0x2A,    // | 42 |  SUBF f_reg_a f_reg_b f_reg_c     | subtract f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    MULF = 0x2B,    // | 43 |  MULF f_reg_a f_reg_b f_reg_c     | multiply f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)
    DIVF = 0x2C,    // | 44 |  DIVF f_reg_a f_reg_b f_reg_c     | divide f_reg_b to f_reg_c, storing the result in f_reg_a (FP operation)

    ADDFI = 0x2D,   // | 45 |  ADDFI f_reg_a f_reg_b value      | add f_reg_b to value, storing the result in f_reg_a (FP operation)
    SUBFI = 0x2E,   // | 46 |  SUBFI f_reg_a f_reg_b value      | subtract f_reg_b by value, storing the result in f_reg_a (FP operation)
    MULFI = 0x2F,   // | 47 |  MULFI f_reg_a f_reg_b value      | multiply f_reg_b by value, storing the result in f_reg_a (FP operation)
    DIVFI = 0x30,   // | 48 |  DIVFI f_reg_a f_reg_b value      | divide f_reg_b by value, storing the result in f_reg_a (FP operation)

    LDWF = 0x31,    // | 49 |  LDWF f_reg mem_addr              | Loads data (f32) from a memory address (uint16_t), storing in a register (FP operation)
    SDWF = 0x32,    // | 50 |  SDWF mem_addr f_reg              | Stores data (f32) from a register into memory  at the given address (uint16_t) (FP operation)

    // Branching instructions

    IFF = 0x33,     // | 51 |  IFF f_reg val addr               | If reg == val, goto addr (FP operation)
    IFNF = 0x34,    // | 52 |  IFNF f_reg val addr              | If reg != val, goto addr (FP operation)
    JNZF = 0x35,    // | 53 |  JNZF f_reg addr                  | Jumps to addr if reg is not 0 (FP operation)

    IFRF = 0x36,    // | 54 |  IFRF f_reg val rel_addr          | Relative jump to rel_addr if reg == val (FP operation)
    IFNRF = 0x37,   // | 55 |  IFNRF f_reg val rel_addr         | Relative jump to rel_addr if reg != val (FP operation)
    JNZRF = 0x38,   // | 56 |  JNZRF f_reg rel_addr             | Relative jump to addr if reg is not 0 (FP operation)

    // Equality testing

    EQ = 0x39,      // | 57 |  EQ reg_a reg_b reg_c             | set reg_a to 1 if reg_b == reg_c, else 0
    NEQ = 0x3A,     // | 58 |  NEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b != reg_c, else 0
    LEQ = 0x3B,     // | 59 |  LEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b <= reg_c, else 0
    GEQ = 0x3C,     // | 60 |  GEQ reg_a reg_b reg_c            | set reg_a to 1 if reg_b >= reg_c, else 0
    LT = 0x3D,      // | 61 |  LT reg_a reg_b reg_c             | set reg_a to 1 if reg_b < reg_c, else 0
    GT = 0x3E,      // | 62 |  GT reg_a reg_b reg_c             | set reg_a to 1 if reg_b > reg_c, else 0

    EQF = 0x3F,     // | 63 |  EQF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b == reg_c, else 0 (FP operation)
    NEQF = 0x40,    // | 64 |  NEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b != reg_c, else 0 (FP operation)
    LEQF = 0x41,    // | 65 |  LEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b <= reg_c, else 0 (FP operation)
    GEQF = 0x42,    // | 66 |  GEQF f_reg_a f_reg_b f_reg_c     | set reg_a to 1 if reg_b >= reg_c, else 0 (FP operation)
    LTF = 0x43,     // | 67 |  LTF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b < reg_c, else 0 (FP operation)
    GTF = 0x44,     // | 68 |  GTF f_reg_a f_reg_b f_reg_c      | set reg_a to 1 if reg_b > reg_c, else 0 (FP operation)
    
    
    PSHR = 0x45,    // | 69  |  PSH val                         | Push the register's value to stack

    // Floating point specific instructions

    REC = 0x46,    // | 70  |  REC f_reg_a f_reg_b                  | stores the reciprocal of reg_b into reg_a
    SQRT = 0x47,   // | 71  |  SQRT f_reg_a f_reg_b                 | stores the square root of reg_b into reg_a
    RND = 0x48,    // | 72  |  RND f_reg_a f_reg_b                  | stores the rounded value of reg_b into reg_a

    SIN = 0x49,    // | 73  |  SIN f_reg_a f_reg_b                  | stores the sine of reg_b into reg_a
    COS = 0x4A,    // | 74  |  SIN f_reg_a f_reg_b                  | stores the cosine of reg_b into reg_a
    TAN = 0x4B,    // | 75  |  TAN f_reg_a f_reg_b                  | stores the tangent of reg_b into reg_a

    ASIN = 0x4C,   // | 76  |  ASIN f_reg_a f_reg_b                 | stores the arcsine of reg_b into reg_a
    ACOS = 0x4D,   // | 77  |  ASIN f_reg_a f_reg_b                 | stores the arccosine of reg_b into reg_a
    ATAN = 0x4E,   // | 78  |  ATAN f_reg_a f_reg_b                 | stores the arctangent of reg_b into reg_a

    // Exceptions!
    
    SEX = 0x4F,   // | 79  |  SEX flag addr                         | If the exception defined by the flag is triggered, go to the handler (addr)


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

            0x24 => Self::RET,

            0x25 => Self::SETF,

            0x26 => Self::MOVF,
            0x27 => Self::MOVFI,
            0x28 => Self::MOVIF,

            0x29 => Self::ADDF,
            0x2A => Self::SUBF,
            0x2B => Self::MULF,
            0x2C => Self::DIVF,

            0x2D => Self::ADDFI,
            0x2E => Self::SUBFI,
            0x2F => Self::MULFI,
            0x30 => Self::DIVFI,

            0x31 => Self::LDWF,
            0x32 => Self::SDWF,

            0x33 => Self::IFF,
            0x34 => Self::IFNF,
            0x35 => Self::IFRF,
            0x36 => Self::IFNRF,

            0x37 => Self::JNZF,
            0x38 => Self::JNZRF,

            0x39 => Self::EQ,
            0x3A => Self::NEQ,
            0x3B => Self::LEQ,
            0x3C => Self::GEQ,
            0x3D => Self::LT,
            0x3E => Self::GT,

            0x3F => Self::EQF,
            0x40 => Self::NEQF,
            0x41 => Self::LEQF,
            0x42 => Self::GEQF,
            0x43 => Self::LTF,
            0x44 => Self::GTF,

            0x45 => Self::PSHR,

            0x46 => Self::REC,
            0x47 => Self::SQRT,
            0x48 => Self::RND,

            0x49 => Self::SIN,
            0x4A => Self::COS,
            0x4B => Self::TAN,

            0x4C => Self::ASIN,
            0x4D => Self::ACOS,
            0x4E => Self::ATAN,

            0x4F => Self::SEX,

            _ => panic!("Error - value does not correspond to an instruction!")
        }
    }
}
                        // Instruction, param count length in bytes - used by the VM to determine jumps and different size params (like 32 bit and so on)
pub const OPCODE_TABLE: [(Instructions, u8); 80] = [
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

    (Instructions::RET, 0),

    (Instructions::SETF, 5),

    (Instructions::MOVF, 2),
    (Instructions::MOVFI, 2),
    (Instructions::MOVIF, 2),

    
    (Instructions::ADDF, 3),
    (Instructions::SUBF, 3),
    (Instructions::MULF, 3),
    (Instructions::DIVF, 3),

    (Instructions::ADDFI, 6),
    (Instructions::SUBFI, 6),
    (Instructions::MULFI, 6),
    (Instructions::DIVFI, 6),

    (Instructions::LDWF, 3),
    (Instructions::SDWF, 3),

    (Instructions::IFF, 7),
    (Instructions::IFNF, 7),
    (Instructions::JNZF, 3),

    (Instructions::IFRF, 6),
    (Instructions::IFNRF, 6),
    (Instructions::JNZRF, 2),

    (Instructions::EQ, 3),
    (Instructions::NEQ, 3),
    (Instructions::LEQ, 3),
    (Instructions::GEQ, 3),
    (Instructions::LT, 3),
    (Instructions::GT, 3),

    (Instructions::EQF, 3),
    (Instructions::NEQF, 3),
    (Instructions::LEQF, 3),
    (Instructions::GEQF, 3),
    (Instructions::LTF, 3),
    (Instructions::GTF, 3),

    (Instructions::PSHR, 1),

    (Instructions::REC, 2),
    (Instructions::SQRT, 2),
    (Instructions::RND, 2),

    (Instructions::SIN, 2),
    (Instructions::COS, 2),
    (Instructions::TAN, 2),

    (Instructions::ASIN, 2),
    (Instructions::ACOS, 2),
    (Instructions::ATAN, 2),

    (Instructions::SEX, 3),
];