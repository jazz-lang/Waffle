#[derive(Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum OpV {
    Star(u8),
    Ldar(u8),
    /// acc = arguments[idx]
    LdaArg(u16),
    /// acc = arguments
    LdaArguments,
    Mov(u8,u8),
    Add(u8,u32),
    Sub(u8,u32),
    Div(u8,u32),
    Mul(u8,u32),
    Mod(u8,u32),
    UShr(u8,u32),
    Shr(u8,u32),
    Shl(u8,u32),
    BitwiseOr(u8,u32),
    BitwiseAnd(u8,u32),
    BitwiseXor(u8,u32),
    LdaUndefined,
    LdaInt,
    LdaTrue,
    LdaFalse,
    LdaK(u16),
    LdaNull,
    LdaGlobal(u16),
    StaGlobal(u16),
    /// PolyIC Opcode
    LdaGlobalDirect(u16),
    // PolyIC Opcode
    StaGlobalDirect(u16),
    LdaById(u8,u16,u32),
    StaById(u8,u16,u32),
    LdaByVal(u8,u8),
    StaByVal(u8,u8),
    LdaByIdx(u8,u32,u32),
    StaByIdx(u8,u32,u32),
    LdaOwnProperty(u8,u16,u32),
    StaOwnProperty(u8,u16,u32),
    LdaProtoProperty(u8,u16,u32),
    LdaChainProperty(u8,u16,u32),
    LdaOwnIdx(u8,u32,u32),
    StaOwnIdx(u8,u32,u32),
    LdaChainIdx(u8,u32,u32),
    LdaProtoIdx(u8,u32,u32),
    LdaSlowById(u8,u16,u32),
    StaSlowById(u8,u16,u32),
    LdaSlowByIdx(u8,u32,u32),
    StaSlowByIdx(u8,u32,u32),
    PushA,
    PopA,
    PushR(u8),
    PopR(u8),
    LdaThis,
    Call(u8,u16),
    Throw,
    CatchSetup(u16),
    /// Hint for interpreter and JIT that current block is loop header.
    /// 
    /// This opcode may use OSR to enter JITed code of hot loop.
    LoopHint(u32),
    BrC(u16,u16),
    Br(u16),
    Return,
    Greater(u8,u32),
    GreaterEqual(u8,u32),
    Less(u8,u32),
    LessEqual(u8,u32),
    Equal(u8,u32),
    NotEqual(u8,u32),

    CloseEnv(u16),
    // acc = new Object()
    NewObject,
    // acc = new Array(N)
    NewArray(u32),
    // acc = new acc (stack[-1]..stack[argc])
    Construct(u16),
    
    LdaUpvalue(u16),
    StaUpvalue(u16),
    /// Safepoint is used to check for GC or for OSR entry. 
    /// 
    /// We try to JIT compile functions in another separate goroutine and 
    /// if function is compiled we check at safepoint and try to enter JITed code.
    Safepoint(u32),
    Debug(u32),
}

pub const OPCODE_COUNT: usize = 45;

//One opcode should be <= 12 bytes.
const _: [bool; 0] = [false; (!(std::mem::size_of::<OpV>() <= 12)) as usize];

pub enum Op {
    Star = 0,
    Ldar,
    Mov,
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    UShr,
    Shr,
    Shl,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    LdaUndefined,
    LdaInt,
    LdaTrue,
    LdaFalse,
    LdaK,
    LdaNull,
    LdaGlobal,
    StaGlobal,
    LdaById,
    StaById,
    LdaByVal,
    StaByVal,
    LdaByIdx,
    StaByIdx,
    LdaOwnProperty,
    StaOwnProperty,
    LdaProtoProperty,
    StaProtoProperty,
    LdaChainProperty,
    StaChainProperty,
    LdaOwnId,
    StaOwnId,
    LdaChainId,
    StaChainId,
    LdaProtoId,
    StaProtoId,
    LdaSlowById,
    StaSlowById,
    LdaSlowByIdx,
    StaSlowByIdx,
    PushA,
    PopA,
    PushR,
    PopR,
    LdaThis,
    Call,
    Throw,
    CatchSetup,
    LoopHint,
    BrC,
    Br,
    Return,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,

    CloseEnv,
    NewObject,
    NewArray,
    Construct,
    LdaUpvalue,
    StaUpvalue,
    Safepoint,
    Debug,
}
