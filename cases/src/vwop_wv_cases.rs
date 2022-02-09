use core::arch::asm;
use rvv_asm::rvv_asm;
use rvv_testcases::intrinsic::vwop_wv;
use rvv_testcases::misc::{avl_iterator, U256, U512};
use rvv_testcases::runner::{run_vop_vv, WideningCategory};

fn expected_op_add(lhs: &[u8], rhs: &[u8], result: &mut [u8]) {
    assert_eq!(lhs.len(), 2 * rhs.len());
    assert_eq!(lhs.len(), result.len());

    match lhs.len() {
        64 => {
            let l = U512::from_little_endian(lhs);
            let r: U512 = U256::from_little_endian(rhs).into();

            let (r, _) = l.overflowing_add(r);
            r.to_little_endian(result);
        }
        n => {
            panic!("Invalid sew: {}", n);
        }
    }
}

pub fn test_vwop_wv() {
    fn add(lhs: &[u8], rhs: &[u8], result: &mut [u8], sew: u64, lmul: i64, avl: u64) {
        vwop_wv(lhs, rhs, result, sew, avl, lmul, || unsafe {
            rvv_asm!("vwaddu.wv v21, v1, v11");
        });
    }
    let sew = 256u64;
    for lmul in [-2, 1, 4, 8] {
        for avl in avl_iterator(sew, 4) {
            run_vop_vv(
                sew,
                lmul,
                avl,
                expected_op_add,
                add,
                WideningCategory::VdVs2,
                "vwaddu.wv",
            );
        }
    }
}
