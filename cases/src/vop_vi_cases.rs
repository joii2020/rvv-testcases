use core::arch::asm;
use core::convert::TryInto;
use rvv_asm::rvv_asm;
use rvv_testcases::misc::{Widening, U256};
use rvv_testcases::runner::{run_template_v_vi, MaskType};

fn expected_op_add(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    match lhs.len() {
        1 => {
            let (r, _) = lhs[0].overflowing_add(imm as u8);
            result[0] = r;
        }
        2 => {
            let (r, _) = u16::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u16);
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let (r, _) = u32::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u32);
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let (r, _) = u64::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u64);
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) = u128::from_le_bytes(lhs.try_into().unwrap()).overflowing_add(imm as u128);
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let (r, _) = U256::from_little_endian(lhs).overflowing_add(x.sign_extend());
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vadd_vi() {
    // test combinations of lmul, sew, avl, etc
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vadd.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vadd.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }
    run_template_v_vi(expected_op_add, op, true, true, "vadd.vi");
}

fn expected_op_sub(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    match lhs.len() {
        1 => {
            let (r, _) = (imm as u8).overflowing_sub(lhs[0]);
            result[0] = r;
        }
        2 => {
            let (r, _) = (imm as u16).overflowing_sub(u16::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let (r, _) = (imm as u32).overflowing_sub(u32::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let (r, _) = (imm as u64).overflowing_sub(u64::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let (r, _) =
                (imm as u128).overflowing_sub(u128::from_le_bytes(lhs.try_into().unwrap()));
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let (r, _) = x
                .sign_extend()
                .overflowing_sub(U256::from_little_endian(lhs));
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vrsub_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vrsub.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vrsub.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_sub, op, true, true, "vrsub.vi");
}

fn expected_op_and(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] & imm as u8;
        }
        2 => {
            let r = imm as u16 & u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = imm as u32 & u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = imm as u64 & u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u128 & u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let r = x.sign_extend() & U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vand_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vand.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vand.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_and, op, true, true, "vand.vi");
}

fn expected_op_or(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] | imm as u8;
        }
        2 => {
            let r = imm as u16 | u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = imm as u32 | u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = imm as u64 | u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u128 | u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let r = x.sign_extend() | U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vor_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vor.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vor.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_or, op, true, true, "vor.vi");
}

fn expected_op_xor(lhs: &[u8], imm: i64, result: &mut [u8]) {
    assert!(lhs.len() == result.len());
    match lhs.len() {
        1 => {
            result[0] = lhs[0] ^ imm as u8;
        }
        2 => {
            let r = imm as u16 ^ u16::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        4 => {
            let r = imm as u32 ^ u32::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        8 => {
            let r = imm as u64 ^ u64::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        16 => {
            let r = imm as u128 ^ u128::from_le_bytes(lhs.try_into().unwrap());
            result.copy_from_slice(&r.to_le_bytes());
        }
        32 => {
            let x = imm as u64;
            let r = x.sign_extend() ^ U256::from_little_endian(lhs);
            r.to_little_endian(result);
        }
        // 64 => {
        //     let (r, _) = U512::from_little_endian(lhs).overflowing_add(U512::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        // 128 => {
        //     let (r, _) = U1024::from_little_endian(lhs).overflowing_add(U1024::from(imm as i64));
        //     r.to_little_endian(result);
        // }
        _ => {
            panic!("Invalid sew");
        }
    }
}
fn test_vxor_vi() {
    fn op(_: &[u8], rhs: &[u8], mask_type: MaskType) {
        let imm = i64::from_le_bytes(rhs.try_into().unwrap());
        unsafe {
            match imm {
                -16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -16");
                    }
                    _ => panic!("Abort"),
                },
                -15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -15");
                    }
                    _ => panic!("Abort"),
                },
                -14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -14");
                    }
                    _ => panic!("Abort"),
                },
                -13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -13");
                    }
                    _ => panic!("Abort"),
                },
                -12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -12");
                    }
                    _ => panic!("Abort"),
                },
                -11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -11");
                    }
                    _ => panic!("Abort"),
                },
                -10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -10");
                    }
                    _ => panic!("Abort"),
                },
                -9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -9");
                    }
                    _ => panic!("Abort"),
                },
                -8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -8");
                    }
                    _ => panic!("Abort"),
                },
                -7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -7");
                    }
                    _ => panic!("Abort"),
                },
                -6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -6");
                    }
                    _ => panic!("Abort"),
                },
                -5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -5");
                    }
                    _ => panic!("Abort"),
                },
                -4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -4");
                    }
                    _ => panic!("Abort"),
                },
                -3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -3");
                    }
                    _ => panic!("Abort"),
                },
                -2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -2");
                    }
                    _ => panic!("Abort"),
                },
                -1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, -1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, -1");
                    }
                    _ => panic!("Abort"),
                },
                0 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 0, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 0");
                    }
                    _ => panic!("Abort"),
                },
                1 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 1, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 1");
                    }
                    _ => panic!("Abort"),
                },
                2 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 2, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 2");
                    }
                    _ => panic!("Abort"),
                },
                3 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 3, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 3");
                    }
                    _ => panic!("Abort"),
                },
                4 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 4, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 4");
                    }
                    _ => panic!("Abort"),
                },
                5 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 5, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 5");
                    }
                    _ => panic!("Abort"),
                },
                6 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 6, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 6");
                    }
                    _ => panic!("Abort"),
                },
                7 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 7, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 7");
                    }
                    _ => panic!("Abort"),
                },
                8 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 8, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 8");
                    }
                    _ => panic!("Abort"),
                },
                9 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 9, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 9");
                    }
                    _ => panic!("Abort"),
                },
                10 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 10, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 10");
                    }
                    _ => panic!("Abort"),
                },
                11 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 11, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 11");
                    }
                    _ => panic!("Abort"),
                },
                12 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 12, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 12");
                    }
                    _ => panic!("Abort"),
                },
                13 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 13, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 13");
                    }
                    _ => panic!("Abort"),
                },
                14 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 14, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 14");
                    }
                    _ => panic!("Abort"),
                },
                15 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 15, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 15");
                    }
                    _ => panic!("Abort"),
                },
                16 => match mask_type {
                    MaskType::Enable => {
                        rvv_asm!("vxor.vi v24, v8, 16, v0.t");
                    }
                    MaskType::Disable => {
                        rvv_asm!("vxor.vi v24, v8, 16");
                    }
                    _ => panic!("Abort"),
                },
                _ => {
                    panic!("Abort");
                }
            }
        }
    }

    run_template_v_vi(expected_op_xor, op, true, true, "vxor.vi");
}

pub fn test_vop_vi() {
    test_vadd_vi();
    test_vrsub_vi();
    test_vand_vi();
    test_vor_vi();
    test_vxor_vi();
}
