
fn main() {
    use syx_stdlib::HostFnId;
    let all = HostFnId::all();
    println!("len={}", all.len());
    println!("173={:?}", all.get(173).map(|h| h.name()));
    println!("0={:?}", all[0].name());
    let abs = all.iter().position(|h| matches!(h, HostFnId::MathFloatAbs)).unwrap();
    println!("MathFloatAbs_idx={}", abs);
    let fr = all.iter().position(|h| matches!(h, HostFnId::FileRead)).unwrap();
    println!("FileRead_idx={}", fr);
}
