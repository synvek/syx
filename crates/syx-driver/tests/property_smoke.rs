//! C-09 property smoke harness — minimal identity check for `syx prop` /
//! `cargo test --test property_smoke`. Not full property/proptestest GA.

#[test]
fn property_smoke_wrapping_identity() {
    for n in 0u64..100 {
        assert_eq!(n.wrapping_add(1).wrapping_sub(1), n);
    }
}
