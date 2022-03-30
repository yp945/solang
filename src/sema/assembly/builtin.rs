use phf::{phf_map, phf_set};

pub struct AssemblyBuiltinPrototype {
    pub name: &'static str,
    pub no_args: u8,
    pub no_returns: u8,
    pub doc: &'static str,
    pub ty: AssemblyBuiltInFunction,
    pub stops_execution: bool,
}

// The enums declaration order should match that of the static vector containing the builtins
#[derive(Clone, Debug, PartialEq, Copy)]
#[repr(u8)]
pub enum AssemblyBuiltInFunction {
    Stop = 0,
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    SDiv = 5,
    Mod = 6,
    SMod = 7,
    Exp = 8,
    Not = 9,
    Lt = 10,
    Gt = 11,
    Slt = 12,
    Sgt = 13,
    Eq = 14,
    IsZero = 15,
    And = 16,
    Or = 17,
    Xor = 18,
    Byte = 19,
    Shl = 20,
    Shr = 21,
    Sar = 22,
    AddMod = 23,
    MulMod = 24,
    SignExtend = 25,
    Keccak256 = 26,
    Pc = 27,
    Pop = 28,
    MLoad = 29,
    MStore = 30,
    MStore8 = 31,
    SLoad = 32,
    SStore = 33,
    MSize = 34,
    Gas = 35,
    Address = 36,
    Balance = 37,
    SelfBalance = 38,
    Caller = 39,
    CallValue = 40,
    CallDataLoad = 41,
    CallDataSize = 42,
    CallDataCopy = 43,
    CodeSize = 44,
    CodeCopy = 45,
    ExtCodeSize = 46,
    ExtCodeCopy = 47,
    ReturnDataSize = 48,
    ReturnDataCopy = 49,
    ExtCodeHash = 50,
    Create = 51,
    Create2 = 52,
    Call = 53,
    CallCode = 54,
    DelegateCall = 55,
    StaticCall = 56,
    Return = 57,
    Revert = 58,
    SelfDestruct = 59,
    Invalid = 60,
    Log0 = 61,
    Log1 = 62,
    Log2 = 63,
    Log3 = 64,
    Log4 = 65,
    ChainId = 66,
    BaseFee = 67,
    Origin = 68,
    GasPrice = 69,
    BlockHash = 70,
    CoinBase = 71,
    Timestamp = 72,
    Number = 73,
    Difficulty = 74,
    GasLimit = 75,
}

// These are functions that do high level stuff in a contract and are not yet implemented.
static UNSUPPORTED_BUILTINS: phf::Set<&'static str> = phf_set! {
    "datasize", "dataoffset", "datacopy", "setimmutable", "loadimmutable",
    "linkersymbol", "memoryguard"
};

/// Checks if bultin function is unsupported
pub(crate) fn assembly_unsupported_builtin(name: &str) -> bool {
    UNSUPPORTED_BUILTINS.contains(name)
}

static BUILTIN_ASSEMBLY_FUNCTIONS: phf::Map<&'static str, AssemblyBuiltInFunction> = phf_map! {
    "stop" => AssemblyBuiltInFunction::Stop,
    "add" => AssemblyBuiltInFunction::Add,
    "sub" => AssemblyBuiltInFunction::Sub,
    "mul" => AssemblyBuiltInFunction::Mul,
    "div" => AssemblyBuiltInFunction::Div,
    "sdiv" => AssemblyBuiltInFunction::SDiv,
    "mod" => AssemblyBuiltInFunction::Mod,
    "smod" => AssemblyBuiltInFunction::SMod,
    "exp" => AssemblyBuiltInFunction::Exp,
    "not" => AssemblyBuiltInFunction::Not,
    "lt" => AssemblyBuiltInFunction::Lt,
    "gt" => AssemblyBuiltInFunction::Gt,
    "slt" => AssemblyBuiltInFunction::Slt,
    "sgt" => AssemblyBuiltInFunction::Sgt,
    "eq" => AssemblyBuiltInFunction::Eq,
    "iszero" => AssemblyBuiltInFunction::IsZero,
    "and" => AssemblyBuiltInFunction::And,
    "or" => AssemblyBuiltInFunction::Or,
    "xor" => AssemblyBuiltInFunction::Xor,
    "byte" => AssemblyBuiltInFunction::Byte,
    "shl" => AssemblyBuiltInFunction::Shl,
    "shr" => AssemblyBuiltInFunction::Shr,
    "sar" => AssemblyBuiltInFunction::Sar,
    "addmod" => AssemblyBuiltInFunction::AddMod,
    "mulmod" => AssemblyBuiltInFunction::MulMod,
    "signextend" => AssemblyBuiltInFunction::SignExtend,
    "keccak256" => AssemblyBuiltInFunction::Keccak256,
    "pc" => AssemblyBuiltInFunction::Pc,
    "pop" => AssemblyBuiltInFunction::Pop,
    "mload" => AssemblyBuiltInFunction::MLoad,
    "mstore" => AssemblyBuiltInFunction::MStore,
    "mstore8" => AssemblyBuiltInFunction::MStore8,
    "sload" => AssemblyBuiltInFunction::SLoad,
    "sstore" => AssemblyBuiltInFunction::SStore,
    "msize" => AssemblyBuiltInFunction::MSize,
    "gas" => AssemblyBuiltInFunction::Gas,
    "address" => AssemblyBuiltInFunction::Address,
    "balance" => AssemblyBuiltInFunction::Balance,
    "selfbalance" => AssemblyBuiltInFunction::SelfBalance,
    "caller" => AssemblyBuiltInFunction::Caller,
    "callvalue" => AssemblyBuiltInFunction::CallValue,
    "calldataload" => AssemblyBuiltInFunction::CallDataLoad,
    "calldatasize" => AssemblyBuiltInFunction::CallDataSize,
    "calldatacopy" => AssemblyBuiltInFunction::CallDataCopy,
    "codesize" => AssemblyBuiltInFunction::CodeSize,
    "codecopy" => AssemblyBuiltInFunction::CodeCopy,
    "extcodesize" => AssemblyBuiltInFunction::ExtCodeSize,
    "extcodecopy" => AssemblyBuiltInFunction::ExtCodeCopy,
    "returndatasize" => AssemblyBuiltInFunction::ReturnDataSize,
    "returndatacopy" => AssemblyBuiltInFunction::ReturnDataCopy,
    "extcodehash" => AssemblyBuiltInFunction::ExtCodeHash,
    "create" => AssemblyBuiltInFunction::Create,
    "create2" => AssemblyBuiltInFunction::Create2,
    "call" => AssemblyBuiltInFunction::Call,
    "callcode" => AssemblyBuiltInFunction::CallCode,
    "delegatecall" => AssemblyBuiltInFunction::DelegateCall,
    "staticcall" => AssemblyBuiltInFunction::StaticCall,
    "return" => AssemblyBuiltInFunction::Return,
    "revert" => AssemblyBuiltInFunction::Revert,
    "selfdestruct" => AssemblyBuiltInFunction::SelfDestruct,
    "invalid" => AssemblyBuiltInFunction::Invalid,
    "log0" => AssemblyBuiltInFunction::Log0,
    "log1" => AssemblyBuiltInFunction::Log1,
    "log2" => AssemblyBuiltInFunction::Log2,
    "log3" => AssemblyBuiltInFunction::Log3,
    "log4" => AssemblyBuiltInFunction::Log4,
    "chainid" => AssemblyBuiltInFunction::ChainId,
    "basefee" => AssemblyBuiltInFunction::BaseFee,
    "origin" => AssemblyBuiltInFunction::Origin,
    "gasprice" => AssemblyBuiltInFunction::GasPrice,
    "blockhash" => AssemblyBuiltInFunction::BlockHash,
    "coinbase" => AssemblyBuiltInFunction::CoinBase,
    "timestamp" => AssemblyBuiltInFunction::Timestamp,
    "number" => AssemblyBuiltInFunction::Number,
    "difficulty" => AssemblyBuiltInFunction::Difficulty,
    "gaslimit" => AssemblyBuiltInFunction::GasLimit,
};

/// Retrieved the builtin function type from an identifier name
pub fn parse_builtin_keyword(keyword: &str) -> Option<&AssemblyBuiltInFunction> {
    BUILTIN_ASSEMBLY_FUNCTIONS.get(keyword)
}

impl AssemblyBuiltInFunction {
    /// Retrieve the prototype from the enum type
    pub(crate) fn get_prototype_info(self) -> &'static AssemblyBuiltinPrototype {
        let index = self as usize;
        &ASSEMBLY_BUILTIN[index]
    }
}

impl ToString for AssemblyBuiltInFunction {
    fn to_string(&self) -> String {
        let prototype = self.get_prototype_info();
        prototype.name.to_owned()
    }
}

// Assembly built-in functions.
// Descriptions copied and slightly modified from: https://docs.soliditylang.org/en/v0.8.12/yul.html
static ASSEMBLY_BUILTIN: [AssemblyBuiltinPrototype; 76] =
    [
        AssemblyBuiltinPrototype{
            name: "stop",
            no_args: 0,
            no_returns: 0,
            doc: "Stop execution",
            ty: AssemblyBuiltInFunction::Stop,
            stops_execution: true,
        },
        AssemblyBuiltinPrototype{
            name: "add",
            no_args: 2,
            no_returns: 1,
            doc: "add(x, y) returns x + y",
            ty: AssemblyBuiltInFunction::Add,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sub",
            no_args: 2,
            no_returns: 1,
            doc: "sub(x, y) returns x - y",
            ty: AssemblyBuiltInFunction::Sub,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mul",
            no_args: 2,
            no_returns: 1,
            doc: "mul(x, y) returns x*y",
            ty: AssemblyBuiltInFunction::Mul,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "div",
            no_args: 2,
            no_returns: 1,
            doc: "div(x, y) returns x/y or 0 if y == 0",
            ty: AssemblyBuiltInFunction::Div,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sdiv",
            no_args: 2,
            no_returns: 1,
            doc: "sdiv(x, y) returns x/y or 0 if y==0. Used for signed numbers in two's complement",
            ty: AssemblyBuiltInFunction::SDiv,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mod",
            no_args: 2,
            no_returns: 1,
            doc: "mod(x, y) returns x % y or 0 if y == 0",
            ty: AssemblyBuiltInFunction::Mod,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "smod",
            no_args: 2,
            no_returns: 1,
            doc: "smod(x, y) returns x % y or 0 if y == 0. Used for signed numbers in two's complement",
            ty: AssemblyBuiltInFunction::SMod,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "exp",
            no_args: 2,
            no_returns: 1,
            doc: "exp(x, y) returns x to the power of y",
            ty: AssemblyBuiltInFunction::Exp,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "not",
            no_args: 1,
            no_returns: 1,
            doc: "not(x): bitwise \"not\" of x (every bit is negated)",
            ty: AssemblyBuiltInFunction::Not,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "lt",
            no_args: 2,
            no_returns: 1,
            doc: "lt(x, y) returns 1 if x < y, 0 otherwise",
            ty: AssemblyBuiltInFunction::Lt,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "gt",
            no_args: 2,
            no_returns: 1,
            doc: "gt(x, y) returns 1 if x > y, 0 otherwise",
            ty: AssemblyBuiltInFunction::Gt,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "slt",
            no_args: 2,
            no_returns: 1,
            doc: "slt(x, y) returns 1 if x > y, 0 otherwise. Used for signed numbers in two's complement",
            ty: AssemblyBuiltInFunction::Slt,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sgt",
            no_args: 2,
            no_returns: 1,
            doc: "sgt(x, y) returns 1 if x > y, 0 otherwise. Used for signed numbers in two's complement",
            ty: AssemblyBuiltInFunction::Sgt,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "eq",
            no_args: 2,
            no_returns: 1,
            doc: "eq(x, y) returns 1 if x == y, 0 otherwise",
            ty: AssemblyBuiltInFunction::Eq,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "iszero",
            no_args: 1,
            no_returns: 1,
            doc: "iszero(x) returns 1 if x == 0, 0 otherwise",
            ty: AssemblyBuiltInFunction::IsZero,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "and",
            no_args: 2,
            no_returns: 1,
            doc: "and(x, y) returns the bitwise \"and\" between x and y",
            ty: AssemblyBuiltInFunction::And,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "or",
            no_args: 2,
            no_returns: 1,
            doc: "or(x, y) returns the bitwise \"or\" between x and y",
            ty: AssemblyBuiltInFunction::Or,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "xor",
            no_args: 2,
            no_returns: 1,
            doc: "xor(x, y) returns the bitwise \"xor\" between x and y",
            ty: AssemblyBuiltInFunction::Xor,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "byte",
            no_args: 2,
            no_returns: 1,
            doc: "byte(n, x) returns the nth byte of x, where the most significant byt is the 0th",
            ty: AssemblyBuiltInFunction::Byte,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "shl",
            no_args: 2,
            no_returns: 1,
            doc: "shl(x, y) returns the logical shift left of y by x bits",
            ty: AssemblyBuiltInFunction::Shl,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "shr",
            no_args: 2,
            no_returns: 1,
            doc: "shr(x, y) returns the logical shift right of y by x bits",
            ty: AssemblyBuiltInFunction::Shr,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sar",
            no_args: 2,
            no_returns: 1,
            doc: "signed arithmetic shift right y by x bits",
            ty: AssemblyBuiltInFunction::Sar,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "addmod",
            no_args: 3,
            no_returns: 1,
            doc: "addmod(x, y, m) returns (x + y) % m or 0 if m == 0",
            ty: AssemblyBuiltInFunction::AddMod,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mulmod",
            no_args: 3,
            no_returns: 1,
            doc: "mulmod(x, y, m) returns (x * y) % m or 0 if m == 0",
            ty: AssemblyBuiltInFunction::MulMod,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "signextend",
            no_args: 2,
            no_returns: 1,
            doc: "signextend(i, x) sign extends from (i*8+7)th bit counting from least significant",
            ty: AssemblyBuiltInFunction::SignExtend,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "keccak256",
            no_args: 2,
            no_returns: 1,
            doc: "keccak256(p, n) performs keccak(mem[p...(p+n)])",
            ty: AssemblyBuiltInFunction::Keccak256,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "pc",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the current position in code, i.e. the program counter",
            ty: AssemblyBuiltInFunction::Pc,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "pop",
            no_args: 1,
            no_returns: 0,
            doc: "pop(x) discard value x",
            ty: AssemblyBuiltInFunction::Pop,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mload",
            no_args: 1,
            no_returns: 1,
            doc: "mload(p) returns mem[p...(p+32)]",
            ty: AssemblyBuiltInFunction::MLoad,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mstore",
            no_args: 2,
            no_returns: 0,
            doc: "mstore(p, v) stores v into mem[p...(p+32)]",
            ty: AssemblyBuiltInFunction::MStore,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "mstore8",
            no_args: 2,
            no_returns: 0,
            doc: "mstore8(p, v) stores (v & 0xff) into mem[p] (modified a single byte of v)",
            ty: AssemblyBuiltInFunction::MStore8,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sload",
            no_args: 1,
            no_returns: 1,
            doc: "sload(p) returns storage[p], i.e. memory on contract's storage",
            ty: AssemblyBuiltInFunction::SLoad,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "sstore",
            no_args: 2,
            no_returns: 0,
            doc: "sstore(p) stores v into storage[p]",
            ty: AssemblyBuiltInFunction::SStore,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "msize",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the size of memory, i.e largest accessed memory index",
            ty: AssemblyBuiltInFunction::MSize,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "gas",
            no_args: 0,
            no_returns: 1,
            doc: "Returns gas still available to execution",
            ty: AssemblyBuiltInFunction::Gas,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "address",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the address of the current contract / execution context",
            ty: AssemblyBuiltInFunction::Address,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "balance",
            no_args: 2,
            no_returns: 1,
            doc: "balance(a) returns the wei balance at address a",
            ty: AssemblyBuiltInFunction::Balance,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "selfbalance",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the wei balance at the address of the current contract / execution context",
            ty: AssemblyBuiltInFunction::SelfBalance,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "caller",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the call sender",
            ty: AssemblyBuiltInFunction::Caller,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "callvalue",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the wei sent together with the current call",
            ty: AssemblyBuiltInFunction::CallValue,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "calldataload",
            no_args: 1,
            no_returns: 1,
            doc: "calldataload(p) returns call data starting from position p (32 bytes)",
            ty: AssemblyBuiltInFunction::CallDataLoad,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "calldatasize",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the size of call data in bytes",
            ty: AssemblyBuiltInFunction::CallDataSize,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "calldatacopy",
            no_args: 3,
            no_returns: 0,
            doc: "calldatacopy(t, f, s) copies s bytes from calldata at position f to mem at position t",
            ty: AssemblyBuiltInFunction::CallDataCopy,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "codesize",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the size of the current contract / execution context",
            ty: AssemblyBuiltInFunction::CodeSize,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "codecopy",
            no_args: 3,
            no_returns: 0,
            doc: "codecopy(t, f, s) copies s bytes from code at position f to mem at position t",
            ty: AssemblyBuiltInFunction::CodeCopy,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "extcodesize",
            no_args: 1,
            no_returns: 1,
            doc: "extcodesize(a) returns the size of the code at address a",
            ty: AssemblyBuiltInFunction::ExtCodeSize,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "extcodecopy",
            no_args: 4,
            no_returns: 0,
            doc: "extcodecopy(a, t, f, s) copies s bytes from code located at address a at position f to mem at position t",
            ty: AssemblyBuiltInFunction::ExtCodeCopy,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "returndatasize",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the size of the last returndata",
            ty: AssemblyBuiltInFunction::ReturnDataSize,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "returndatacopy",
            no_args: 3,
            no_returns: 0,
            doc: "returndatacopy(t, f, s) copy s bytes from return data at position f to mem at position t",
            ty: AssemblyBuiltInFunction::ReturnDataCopy,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "extcodehash",
            no_args: 1,
            no_returns: 1,
            doc: "extcodehash(a) returns the code hash of address a",
            ty: AssemblyBuiltInFunction::ExtCodeHash,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "create",
            no_args: 3,
            no_returns: 1,
            doc: "create(v, p, n) creates new contract with code mem[p..(p+n)] and sends v wei. It returns the new address or 0 on error",
            ty: AssemblyBuiltInFunction::Create,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "create2",
            no_args: 4,
            no_returns: 1,
            doc: "create2(v, p, n, s) new contract with code mem[p...(p+n)] at address keccak256(0xff . this . s . keccak256(mem[p...(p+n)]) and sends v wei.\n 0xff is a 1 byte value, 'this' is the current contract's address as a 20 byte value and 's' is a big endian 256-bit value. it returns 0 on error.",
            ty: AssemblyBuiltInFunction::Create2,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "call",
            no_args: 7,
            no_returns: 1,
            doc: "call(g, a, v, in, insize, out, outsize) calls contract at address a with input mem[in...(in+insize)] providing f cas and v wei and outputs area mem[out...(out+outsize)]. It returns 0 on error and 1 on success",
            ty: AssemblyBuiltInFunction::Call,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "callcode",
            no_args: 7,
            no_returns: 1,
            doc: "Identical to call(g, a, v, in, insize, out, outsize), but only use the code from a and stay in the context of the current contract otherwise",
            ty: AssemblyBuiltInFunction::CallCode,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "delegatecall",
            no_args: 6,
            no_returns: 1,
            doc: "Identical to 'callcode' but also keep caller and callvalue",
            ty: AssemblyBuiltInFunction::DelegateCall,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "staticcall",
            no_args: 6,
            no_returns: 1,
            doc: "Identical to call(g, a, 0, in, insize, out, outsize), but do not allow state modifications",
            ty: AssemblyBuiltInFunction::StaticCall,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "return",
            no_args: 2,
            no_returns: 0,
            doc: "return(p, s) ends execution and returns data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Return,
            stops_execution: true,
        },
        AssemblyBuiltinPrototype{
            name: "revert",
            no_args: 2,
            no_returns: 0,
            doc: "revert(p, s) ends execution, reverts state changes and returns data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Revert,
            stops_execution: true,
        },
        AssemblyBuiltinPrototype{
            name: "selfdestruct",
            no_args: 1,
            no_returns: 0,
            doc: "selfdestruct(a) ends execution, destroy current contract and sends funds to a",
            ty: AssemblyBuiltInFunction::SelfDestruct,
            stops_execution: true,
        },
        AssemblyBuiltinPrototype{
            name: "invalid",
            no_args: 0,
            no_returns: 0,
            doc: "Ends execution with invalid instruction",
            ty: AssemblyBuiltInFunction::Invalid,
            stops_execution: true,
        },
        AssemblyBuiltinPrototype{
            name: "log0",
            no_args: 2,
            no_returns: 0,
            doc: "log(p, s): log without topics and data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Log0,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "log1",
            no_args: 3,
            no_returns: 0,
            doc: "log1(p, s, t1): log with topic t1 and data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Log1,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "log2",
            no_args: 4,
            no_returns: 0,
            doc: "log2(p, s, t1, t2): log with topics t1, t2 and data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Log2,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "log3",
            no_args: 5,
            no_returns: 0,
            doc: "log3(p, s, t1, t2, t3): log with topics t1, t2, t3 and data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Log3,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "log4",
            no_args: 6,
            no_returns: 0,
            doc: "log4(p, s, t1, t2, t3, t4): log with topics t1, t2, t3, t4 with data mem[p...(p+s)]",
            ty: AssemblyBuiltInFunction::Log4,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "chainid",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the ID of the executing chain",
            ty: AssemblyBuiltInFunction::ChainId,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "basefee",
            no_args: 0,
            no_returns: 1,
            doc: "Return the current block's base fee",
            ty: AssemblyBuiltInFunction::BaseFee,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "origin",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the transaction sender",
            ty: AssemblyBuiltInFunction::Origin,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "gasprice",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the gas price of the transaction",
            ty: AssemblyBuiltInFunction::GasPrice,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "blockhash",
            no_args: 1,
            no_returns: 1,
            doc: "blockhash(b) return the hash of block #b - only valid for the last 256 executing block excluding current",
            ty: AssemblyBuiltInFunction::BlockHash,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "coinbase",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the current mining beneficiary",
            ty: AssemblyBuiltInFunction::CoinBase,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "timestamp",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the timestamp of the current block in seconds since the epoch",
            ty: AssemblyBuiltInFunction::Timestamp,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "number",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the current block's number",
            ty: AssemblyBuiltInFunction::Number,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "difficulty",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the difficulty of the current block",
            ty: AssemblyBuiltInFunction::Difficulty,
            stops_execution: false,
        },
        AssemblyBuiltinPrototype{
            name: "gaslimit",
            no_args: 0,
            no_returns: 1,
            doc: "Returns the current block's gas limit",
            ty: AssemblyBuiltInFunction::GasLimit,
            stops_execution: false,
        },
    ];

#[test]
fn test_builtin_indexes() {
    for item in &ASSEMBLY_BUILTIN {
        let name = item.name;
        let ty = item.ty;

        let parsed_ty = parse_builtin_keyword(name).unwrap();
        assert_eq!(ty, *parsed_ty);
        assert_eq!(name, parsed_ty.get_prototype_info().name);
    }
}