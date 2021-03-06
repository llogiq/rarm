use capstone::ffi::*;
use capstone::ffi::detail::*;

/* p. 756
*/
impl ::cpu::core::CPU {
    pub unsafe fn exec_ubfx(&mut self, insn: &Insn) -> Option<u32> {
        let arm = insn.detail().unwrap().data_arm();
        if !self.cond_passed(arm.cc) {
            return None;
        }

        assert!(arm.operands().len() == 4);
        assert!(arm.operands()[0].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[1].ty == ARMOpType::ARM_OP_REG);
        assert!(arm.operands()[2].ty == ARMOpType::ARM_OP_IMM);
        assert!(arm.operands()[3].ty == ARMOpType::ARM_OP_IMM);
        assert!(false == arm.writeback);
        assert!(false == arm.update_flags);

        let d = ::util::reg_num(arm.operands()[0].data());
        let n = ::util::reg_num(arm.operands()[1].data());
        assert!(d != 15 && n != 15);
        let lsbit = ::util::imm_to_u32(arm.operands()[2].data());
        let width = ::util::imm_to_u32(arm.operands()[3].data());

        let msbit = lsbit + width - 1;
        assert!(msbit <= 31);

        let rn = self.get_reg(n);
        self.set_reg(d, ::util::get_bits(rn, (lsbit..msbit)));

        None
    }
}
