use core::fmt;

pub enum OperationCodes {
    AND = 0x0,
    ORR = 0x1,
    EOR = 0x2,
    ADD = 0x3,
    ADC = 0x4,
    CMP = 0x5,
    SUB = 0x6,
    SBC = 0x7,
    MOV = 0x8,
    LSH = 0x9,
    RSH = 0xa,
}

impl TryFrom<u32> for OperationCodes {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == OperationCodes::AND as u32 => Ok(OperationCodes::AND),
            x if x == OperationCodes::ORR as u32 => Ok(OperationCodes::ORR),
            x if x == OperationCodes::EOR as u32 => Ok(OperationCodes::EOR),
            x if x == OperationCodes::ADD as u32 => Ok(OperationCodes::ADD),
            x if x == OperationCodes::ADC as u32 => Ok(OperationCodes::ADC),
            x if x == OperationCodes::CMP as u32 => Ok(OperationCodes::CMP),
            x if x == OperationCodes::SUB as u32 => Ok(OperationCodes::SUB),
            x if x == OperationCodes::SBC as u32 => Ok(OperationCodes::SBC),
            x if x == OperationCodes::MOV as u32 => Ok(OperationCodes::MOV),
            x if x == OperationCodes::LSH as u32 => Ok(OperationCodes::LSH),
            x if x == OperationCodes::RSH as u32 => Ok(OperationCodes::RSH),
            _ => Err(()),
        }
    }
}

impl fmt::Display for OperationCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationCodes::AND => write!(f, "AND"),
            OperationCodes::ORR => write!(f, "ORR"),
            OperationCodes::EOR => write!(f, "EOR"),
            OperationCodes::ADD => write!(f, "ADD"),
            OperationCodes::ADC => write!(f, "ADC"),
            OperationCodes::CMP => write!(f, "CMP"),
            OperationCodes::SUB => write!(f, "SUB"),
            OperationCodes::SBC => write!(f, "SBC"),
            OperationCodes::MOV => write!(f, "MOV"),
            OperationCodes::LSH => write!(f, "LSH"),
            OperationCodes::RSH => write!(f, "RSH"),
        }
    }
}

pub enum BranchingCodes {
    B = 0x8,
    BEQ = 0x9,
    BNE = 0xa,
    BLE = 0xb,
    BGE = 0xc,
    BL = 0xd,
    BG = 0xe,
}

impl TryFrom<u32> for BranchingCodes {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == BranchingCodes::B as u32 => Ok(BranchingCodes::B),
            x if x == BranchingCodes::BEQ as u32 => Ok(BranchingCodes::BEQ),
            x if x == BranchingCodes::BNE as u32 => Ok(BranchingCodes::BNE),
            x if x == BranchingCodes::BLE as u32 => Ok(BranchingCodes::BLE),
            x if x == BranchingCodes::BGE as u32 => Ok(BranchingCodes::BGE),
            x if x == BranchingCodes::BL as u32 => Ok(BranchingCodes::BL),
            x if x == BranchingCodes::BG as u32 => Ok(BranchingCodes::BG),
            _ => Err(()),
        }
    }
}

impl fmt::Display for BranchingCodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BranchingCodes::B => write!(f, "B"),
            BranchingCodes::BEQ => write!(f, "BEQ"),
            BranchingCodes::BNE => write!(f, "BNE"),
            BranchingCodes::BLE => write!(f, "BLE"),
            BranchingCodes::BGE => write!(f, "BGE"),
            BranchingCodes::BL => write!(f, "BL"),
            BranchingCodes::BG => write!(f, "BG"),
        }
    }
}

pub struct Flags {
    pub beq: bool,
    pub bne: bool,
    pub ble: bool,
    pub bge: bool,
    pub bl: bool,
    pub bg: bool,
    pub carry: bool,
}

impl Flags {
    pub fn get_from_bcc(&self, bcc: BranchingCodes) -> bool {
        match bcc {
            BranchingCodes::BEQ => self.beq,
            BranchingCodes::BNE => self.bne,
            BranchingCodes::BLE => self.ble,
            BranchingCodes::BGE => self.bge,
            BranchingCodes::BL => self.bl,
            BranchingCodes::BG => self.bg,
            _ => false
        }
    }

    pub fn current_flag_states(&self) -> String {
        let mut states = String::from("Current flag states.\n");
        states.push_str(&format!("BEQ   = {}\n", self.beq));
        states.push_str(&format!("BNE   = {}\n", self.bne));
        states.push_str(&format!("BLE   = {}\n", self.ble));
        states.push_str(&format!("BGE   = {}\n", self.bge));
        states.push_str(&format!("BL    = {}\n", self.bl));
        states.push_str(&format!("BG    = {}\n", self.bg));
        states.push_str(&format!("CARRY = {}", self.carry));
        states
    }
}