use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 581 (imm)
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_rsc(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        ::util::assert_shift(&arm.operands());
        assert!(arm.operands().len() == 3);
        assert!(false == arm.writeback);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        let (shifted, _) = self.op_value(&arm.operands()[2]);
        let (result, carry, overflow) = ::arith::add_with_carry(!(self.get_reg(n)), shifted, self.get_carry());

        if d == 15 {
            assert!(false == arm.update_flags);
            return Some(result);
        }

        self.set_reg(d, result);

        let raw: u32 = self.mem.read(insn.address as usize);
        if ::util::get_bit(raw, 20) == 1 { // bug fixed in capstone-next?
        //if arm.update_flags {
            self.set_cpsr_bit(::cpu::cpsr::CPSR_N, ::util::get_bit(result, 31));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_Z, ::util::is_zero(result));
            self.set_cpsr_bit(::cpu::cpsr::CPSR_C, carry);
            self.set_cpsr_bit(::cpu::cpsr::CPSR_V, overflow);
        }
        None
    }
}
