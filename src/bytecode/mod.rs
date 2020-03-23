macro_rules! opcodes {
    ($m: ident) => {
        $m!(
            OP_LOAD_CONST,
            0,
            "load_const",
            "Load constant value (tring,float number etc) to register"
        );
        $m!(OP_LOAD_INT, 1, "load_int", "Load integer value to register");
        $m!(
            OP_LOAD_UPVALUE,
            2,
            "load_upvalue",
            "Load upvalue to register"
        );
        $m!(
            OP_LOAD_BY_ID,
            3,
            "load_by_id",
            "Load value from object by using constant identifier"
        );
        $m!(
            OP_LOAD_BY_INDEX,
            4,
            "load_by_index",
            "Load value from object where key is int32"
        );
        $m!(
            OP_LOAD_STATIC_BY_ID,
            5,
            "load_static",
            "Load static variable"
        );
        $m!(OP_STORE_UPVALUE, 6, "store_upvalue", "Store upvalue");
        $m!(
            OP_STORE_BY_ID,
            7,
            "store_by_id",
            "Store value in object where key is constant identifier"
        );
        $m!(
            OP_STORE_BY_INDEX,
            8,
            "store_by_index",
            "Store value in object where key is int32"
        );
        $m!(
            OP_STORE_STATIC_BY_ID,
            9,
            "store_static",
            "Store static variable"
        );
        $m!(OP_LOAD_TRUE, 10, "load_true", "Load true");
        $m!(OP_LOAD_FALSE, 11, "load_false", "Load false");
        $m!(OP_LOAD_NIL, 12, "load_nil", "Load nil");
        $m!(OP_LOAD_UNDEF, 13, "load_undef", "Load undefined");
        $m!(
            OP_LOAD_STACK,
            14,
            "load_stack",
            "Load value from stack, runtime panic if index is out of bounds"
        );
        $m!(OP_STORE_STACK, 15, "store_stack", "Store value at stack");
        $m!(OP_PUSH, 16, "push", "Push value onto stack");
        $m!(OP_POP, 17, "pop", "Pop value from stack");
        $m!(OP_CONDITIONAL, 18, "cond", "Conditional goto");
        $m!(OP_GOTO, 19, "goto", "Goto target");
        $m!(OP_MAKEENV, 20, "makeenv", "Close upvalues in closure");
        $m!(OP_RET, 21, "ret", "Return from function");
        $m!(OP_CALL, 22, "call", "Invoke function");
        $m!(
            OP_VIRT_CALL,
            23,
            "virtcall",
            "Invoke some method on function"
        );
        $m!(OP_TCALL, 24, "tailcall", "Tail recursion");
        $m!(OP_NEW, 25, "new", "Create new object");
        $m!(OP_SAFEPOINT, 26, "safepoint", "GC safepoint");
        $m!(OP_ADD, 27, "add");
        $m!(OP_SUB, 28, "sub");
        $m!(OP_DIV, 29, "div");
        $m!(OP_MUL, 30, "mul");
        $m!(OP_MOD, 31, "mod");
        $m!(OP_SHL, 32, "shl");
        $m!(OP_SHR, 33, "shr");
        $m!(OP_USHR, 34, "unsigned shr");
        $m!(OP_EQ, 35, "eq");
        $m!(OP_GT, 36, "gt");
        $m!(OP_LT, 37, "lt");
        $m!(OP_LE, 38, "le");
        $m!(OP_GE, 39, "ge");
        $m!(OP_NEQ, 40, "neq");
        $m!(OP_NOT, 41, "not");
        $m!(OP_NEG, 42, "neg");
        $m!(OP_PLUS, 43, "plus");
        $m!(OP_LOAD_THIS, 44, "load_this");
        $m!(OP_STORE_THIS, 45, "store_this");
        $m!(OP_YIELD, 46, "yield");
        $m!(OP_POPCNT, 47, "popcnt", "Pop N elements from stack");
        $m!(
            OP_LOAD_BY_VALUE,
            48,
            "load_by_value",
            "Load value from object where key is some value"
        );
        $m!(
            OP_STORE_BY_VALUE,
            49,
            "store_by_value",
            "Store value in object where key is some value"
        );
    };
}

macro_rules! declare {
    ($op: ident,$x: expr,$_xx: expr,$_x2: literal) => {
        #[doc($_x2)]
        pub const $op: u32 = $x;
    };
    ($op: ident,$x: expr,$_xx: expr) => {
        pub const $op: u32 = $x;
    };
}

opcodes!(declare);
