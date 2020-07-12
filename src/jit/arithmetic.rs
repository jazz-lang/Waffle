use super::*;
use add_generator::*;
use mathic::*;

impl<'a> JIT<'a> {
    pub fn emit_op_div(&mut self, op: &Ins) {
        if let Ins::Div(op1, op2, dest) = op {
            let left = T0;
            let right = T1;
            let result = left;
            let scratch = T2;
            let scratch_fp = FT2;

            self.emit_get_virtual_register(*op1, left);
            self.emit_get_virtual_register(*op2, right);
            let mut gen = div_generator::DivGenerator::new(
                result, left, right, FT0, FT1, scratch, scratch_fp,
            );

            gen.generate_fast_path(self);

            if gen.did_emit_fast_path {
                gen.end_jump_list
                    .iter()
                    .for_each(|item| item.link(&mut self.masm));
                self.emit_put_virtual_register(*dest, result);
                self.add_slow_cases(&gen.slow_path_jump_list);
            }
        }
    }

    pub fn emit_op_add(&mut self, op: &Ins) {
        match op {
            Ins::Add(src1, src2, dest) => {
                let meta = self.code_block.metadata(self.bytecode_index as _);
                let math_ic = self.code_block.add_jit_addic(&meta.arith_profile);
                self.ins_to_mathic
                    .insert(op as *const Ins, math_ic as *mut MathIC<_> as *mut u8);
                self.emit_mathic_fast_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0 as *mut _,
                    operations::operation_value_add as *const u8,
                );
            }
            _ => op_unreachable!(),
        }
    }

    pub fn emit_slow_op_add(
        &mut self,
        op: &Ins,
        slow_cases: &mut std::iter::Peekable<std::slice::Iter<'_, SlowCaseEntry>>,
    ) {
        self.link_all_slow_cases(slow_cases);
        match op {
            Ins::Add(src1, src2, dest) => {
                let ic = *self.ins_to_mathic.get(&(op as *const Ins)).unwrap();
                let math_ic = unsafe { &mut *(ic as *mut MathIC<AddGenerator>) };
                self.emit_mathic_slow_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0xdead as *const _,
                    operations::operation_value_add_optimize as *const _,
                );
            }
            _ => op_unreachable!(),
        }
    }
    pub fn emit_op_sub(&mut self, op: &Ins) {
        match op {
            Ins::Sub(src1, src2, dest) => {
                let meta = self.code_block.metadata(self.bytecode_index as _);
                let math_ic = self.code_block.add_jit_subic(&meta.arith_profile);
                self.ins_to_mathic
                    .insert(op as *const Ins, math_ic as *mut MathIC<_> as *mut u8);
                self.emit_mathic_fast_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0 as *mut _,
                    operations::operation_value_sub as *const u8,
                );
            }
            _ => op_unreachable!(),
        }
    }

    pub fn emit_slow_op_sub(
        &mut self,
        op: &Ins,
        slow_cases: &mut std::iter::Peekable<std::slice::Iter<'_, SlowCaseEntry>>,
    ) {
        self.link_all_slow_cases(slow_cases);
        match op {
            Ins::Sub(src1, src2, dest) => {
                let ic = *self.ins_to_mathic.get(&(op as *const Ins)).unwrap();
                let math_ic = unsafe { &mut *(ic as *mut MathIC<sub_generator::SubGenerator>) };
                self.emit_mathic_slow_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0xdead as *const _,
                    operations::operation_value_sub_optimize as *const _,
                );
            }
            _ => op_unreachable!(),
        }
    }
    pub fn emit_op_mul(&mut self, op: &Ins) {
        match op {
            Ins::Mul(src1, src2, dest) => {
                let meta = self.code_block.metadata(self.bytecode_index as _);
                let math_ic = self.code_block.add_jit_mulic(&meta.arith_profile);
                self.ins_to_mathic
                    .insert(op as *const Ins, math_ic as *mut MathIC<_> as *mut u8);
                self.emit_mathic_fast_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0 as *mut _,
                    operations::operation_value_mul as *const u8,
                );
            }
            _ => op_unreachable!(),
        }
    }

    pub fn emit_slow_op_mul(
        &mut self,
        op: &Ins,
        slow_cases: &mut std::iter::Peekable<std::slice::Iter<'_, SlowCaseEntry>>,
    ) {
        self.link_all_slow_cases(slow_cases);
        match op {
            Ins::Mul(src1, src2, dest) => {
                let ic = *self.ins_to_mathic.get(&(op as *const Ins)).unwrap();
                let math_ic = unsafe { &mut *(ic as *mut MathIC<mul_generator::MulGenerator>) };
                self.emit_mathic_slow_bin(
                    math_ic,
                    op,
                    *src1,
                    *src2,
                    *dest,
                    0xdead as *const _,
                    operations::operation_value_mul_optimize as *const _,
                );
            }
            _ => op_unreachable!(),
        }
    }
    pub fn emit_mathic_slow_bin<GEN: MathICGenerator + BinaryMathICGenerator>(
        &mut self,
        math_ic: &mut MathIC<GEN>,
        ins: &Ins,
        src1: virtual_register::VirtualRegister,
        src2: virtual_register::VirtualRegister,
        dest: virtual_register::VirtualRegister,
        _profiled_fn: *const u8, // TODO: Type info
        repatch_fn: *const u8,
    ) {
        log::debug!("[JIT Arithmetic] Emit slow MathIC case");
        let label = self.masm.label();
        self.ins_to_mathic_state
            .get_mut(&(ins as *const Ins))
            .map(|item| {
                item.slow_path_start = label;
            });
        let left_reg = T1;
        let right_reg = T2;
        let result_reg = T0;
        let scratch_gpr = T3;
        let scratch_fpr = FT2;

        let generator = GEN::new(
            result_reg,
            left_reg,
            right_reg,
            FT0,
            FT1,
            scratch_gpr,
            scratch_fpr,
        );
        math_ic.generator = Some(generator);
        self.emit_get_virtual_register(src1, left_reg);
        self.emit_get_virtual_register(src2, right_reg);
        let slow_path_call = {
            self.masm.prepare_call_with_arg_count(4);
            self.masm.pass_reg_as_arg(right_reg, 2);
            self.masm.pass_reg_as_arg(left_reg, 1);
            self.masm.pass_ptr_as_arg(math_ic as *mut _ as usize, 3);
            self.masm
                .pass_ptr_as_arg(crate::get_vm() as *mut _ as usize, 0); // TODO: Put VM pointer as first argument
            self.update_top_frame();
            let call = self.masm.call_ptr_repatch_argc(repatch_fn, 3);
            self.masm.move_rr(RET0, result_reg);
            call
        };
        self.ins_to_mathic_state
            .get_mut(&(ins as *const Ins))
            .map(|item| {
                item.slow_path_call = slow_path_call;
            });
        self.emit_put_virtual_register(dest, result_reg);
        let state = self
            .ins_to_mathic_state
            .get_mut(&(ins as *const Ins))
            .unwrap() as *mut MathICGenerationState;
        let ic = *self.ins_to_mathic.get(&(ins as *const Ins)).unwrap();
        self.masm.add_link_task(Box::new(move |link_buffer| {
            let state = unsafe { &mut *state };
            let math_ic = unsafe { &mut *(ic as *mut MathIC<GEN>) };
            math_ic.finalize_inline_code(state, link_buffer);
        }));
    }
    pub fn emit_mathic_fast_bin<GEN: MathICGenerator + BinaryMathICGenerator>(
        &mut self,
        math_ic: &mut MathIC<GEN>,
        ins: &Ins,
        src1: virtual_register::VirtualRegister,
        src2: virtual_register::VirtualRegister,
        dest: virtual_register::VirtualRegister,
        _profiled_fn: *const u8, // TODO: Type info
        non_profiled_fn: *const u8,
    ) {
        log::debug!("[JIT Arithmetic] Emit fast MathIC case");
        let left_reg = T1;
        let right_reg = T2;
        let result_reg = T0;
        let scratch_gpr = T3;
        let scratch_fpr = FT2;

        let generator = GEN::new(
            result_reg,
            left_reg,
            right_reg,
            FT0,
            FT1,
            scratch_gpr,
            scratch_fpr,
        );
        math_ic.generator = Some(generator);

        self.emit_get_virtual_register(src1, left_reg);
        self.emit_get_virtual_register(src2, right_reg);
        self.ins_to_mathic_state
            .insert(ins, MathICGenerationState::default());
        let state = self
            .ins_to_mathic_state
            .get_mut(&(ins as *const Ins))
            .unwrap() as *mut MathICGenerationState;
        let generated_inline = math_ic.generate_inline(self, unsafe { &mut *state }, true);
        if !generated_inline {
            // cannot generate inline code based on type info, invoke `profiled_fn` if profiling is enabled or `non_profiled_fn` if disabled.
            self.masm.prepare_call_with_arg_count(3);
            self.masm.pass_reg_as_arg(right_reg, 2);
            self.masm.pass_reg_as_arg(left_reg, 1);
            self.masm
                .pass_ptr_as_arg(crate::get_vm() as *mut _ as usize, 0); // TODO: Put VM pointer as first argument
            self.update_top_frame();
            self.masm.call_ptr(non_profiled_fn);
            self.masm.move_rr(RET0, result_reg);
        } else {
            // inline code generated, now we can generate slow path at end of the function.
            for j in unsafe { (&*state).slow_path_jumps.jumps.iter() } {
                self.add_slow_case(*j);
            }
        }
        self.emit_put_virtual_register(dest, result_reg);
    }
}
