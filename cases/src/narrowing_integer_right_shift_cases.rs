use core::arch::asm;

use alloc::boxed::Box;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::{vop_wv, vop_wx};
use rvv_testcases::misc::{avl_iterator, U256, U512};
use rvv_testcases::runner::{run_vop_vv, run_vop_vx, ExpectedOp, WideningCategory};

fn expected_op(lhs: &[u8], x: u64, result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);

    match result.len() {
        32 => {
            // The low lg2(2*SEW) bits of the shift-amount value are used
            let shift_amount = x & 0b111111111;
            let l = U512::from_little_endian(lhs);
            let r = l >> shift_amount;
            let r2: U256 = r.into();
            r2.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

fn expected_op2(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), result.len() * 2);
    assert_eq!(rhs.len(), result.len());

    match result.len() {
        32 => {
            // The low lg2(2*SEW) bits of the shift-amount value are used
            let l = U512::from_little_endian(lhs);
            let r = U256::from_little_endian(rhs);
            let shift_amount = r.low_u64() & 0b111111111;

            let res = l >> shift_amount;
            let res2: U256 = res.into();
            res2.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_narrowing_integer_right_shift() {
    fn srl(lhs: &[u8], x: u64, result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_wx(lhs, x, result, sew, avl, lmul, |_| unsafe {
            rvv_asm!("mv t0, {}", "vnsrl.wx v21, v1, t0", in (reg) x);
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vx(
                256,
                lmul,
                avl,
                expected_op,
                srl,
                WideningCategory::Vs2Only,
                "vnsrl.wx",
            );
        }
    }

    fn srl2(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vnsrl.wv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                256,
                lmul,
                avl,
                ExpectedOp::Normal(Box::new(expected_op2)),
                srl2,
                WideningCategory::Vs2Only,
                "vnsrl.wv",
            );
        }
    }
}