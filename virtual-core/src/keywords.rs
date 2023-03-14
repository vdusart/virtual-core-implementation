pub enum OperationCodes  {
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
    RSH = 0xa
}

impl TryFrom<u32> for OperationCodes {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == OperationCodes::ADD as u32 => Ok(OperationCodes::ADD),
            x if x == OperationCodes::SUB as u32 => Ok(OperationCodes::SUB),
            x if x == OperationCodes::MOV as u32 => Ok(OperationCodes::MOV),
            _ => Err(()),
        }
    }
}