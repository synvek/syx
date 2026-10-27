//! Host plugin smoke — ed25519 verify and echo fixture.
// was: plugin_smoke

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};
use syx_host_plugin::{verify_ed25519_signature, verify_signature, PluginManifest};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

#[test]
fn plugin_signed_load_smoke() {
    let manifest = PluginManifest {
        id: "echo".into(),
        api_version: 1,
        library: "libecho.so".into(),
        hosts: vec![],
        min_syx_version: None,
        capabilities: vec!["crypto".into()],
        signature: Some("syx1:stub:test".into()),
    };
    assert!(verify_signature(&manifest).is_ok());
}

#[test]
fn plugin_bad_sig_verify() {
    std::env::set_var("SYX_HOST_PLUGIN_VERIFY", "1");
    let manifest = PluginManifest {
        id: "echo".into(),
        api_version: 1,
        library: "libecho.so".into(),
        hosts: vec![],
        min_syx_version: None,
        capabilities: vec![],
        signature: None,
    };
    assert!(verify_signature(&manifest).is_err());
    std::env::remove_var("SYX_HOST_PLUGIN_VERIFY");
}

#[test]
fn plugin_ed25519_roundtrip() {
    use ed25519_dalek::SigningKey;
    let sk = SigningKey::from_bytes(&[7u8; 32]);
    let manifest = PluginManifest {
        id: "t".into(),
        api_version: 1,
        library: "libt.so".into(),
        hosts: vec![],
        min_syx_version: None,
        capabilities: vec![],
        signature: None,
    };
    let checksum = manifest.checksum();
    let sig = syx_pkg::sign_package_checksum(&checksum, &sk);
    assert!(verify_ed25519_signature(&checksum, &sig));
}

#[test]
fn plugin_echo_vm() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("common/plugin_echo.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    let (code, _lines) = run_file_printed(&path, &options).expect("run");
    assert_eq!(code, 0);
}

#[test]
fn plugin_v2probe_sum_vm() {
    syx_host_plugin::register_v2probe_sum_in_process();
    let path = fixture("common/plugin_v2probe.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile v2probe");
    let (code, lines) = run_file_printed(&path, &options).expect("run v2probe");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("6"),
        "v2probe sum 1+2+3 must print 6, got {lines:?}"
    );
}

#[test]
fn plugin_v2probe_dot_vm() {
    syx_host_plugin::register_v2probe_dot_in_process();
    let path = fixture("common/plugin_v2probe_dot.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile v2probe_dot");
    let (code, lines) = run_file_printed(&path, &options).expect("run v2probe_dot");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("11"),
        "v2probe dot [1,2]·[3,4] must print 11, got {lines:?}"
    );
}

#[test]
fn plugin_v2probe_grow_vm() {
    syx_host_plugin::register_v2probe_grow_in_process();
    let path = fixture("common/plugin_v2probe_grow.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile v2probe_grow");
    let (code, lines) = run_file_printed(&path, &options).expect("run v2probe_grow");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("1") && joined.contains("2") && joined.contains("3"),
        "v2probe grow must echo 1 2 3, got {lines:?}"
    );
}

#[test]
fn plugin_v2probe_handle_vm() {
    syx_host_plugin::register_v2probe_pin_in_process();
    syx_host_plugin::register_v2probe_sumh_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_v2probe_handle.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile v2probe_handle");
    let (code, lines) = run_file_printed(&path, &options).expect("run v2probe_handle");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("6"),
        "v2probe handle pin/sumh 1+2+3 must print 6, got {lines:?}"
    );
}

#[test]
fn plugin_faer_mul_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_mul_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_faer_mul.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile plugin_faer_mul");
    let (code, lines) = run_file_printed(&path, &options).expect("run plugin_faer_mul");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("1")
            && joined.contains("2")
            && joined.contains("3")
            && joined.contains("4"),
        "faer mul A×I must print 1 2 3 4, got {lines:?}"
    );
}

#[test]
fn plugin_faer_binop_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_add_in_process();
    syx_host_plugin::register_faer_sub_in_process();
    syx_host_plugin::register_faer_hadamard_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_faer_binop.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile plugin_faer_binop");
    let (code, lines) = run_file_printed(&path, &options).expect("run plugin_faer_binop");
    assert_eq!(code, 0);
    let nums: Vec<&str> = lines.iter().map(|s| s.trim()).collect();
    assert_eq!(
        nums,
        [
            "11", "22", "33", "44", "-9", "-18", "-27", "-36", "10", "40", "90", "160"
        ],
        "faer add/sub/hadamard must print 11 22 33 44 / -9 -18 -27 -36 / 10 40 90 160, got {lines:?}"
    );
}


#[test]
fn plugin_pad_reflect_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_zeros_in_process();
    syx_host_plugin::register_faer_mul_in_process();
    syx_host_plugin::register_faer_solve_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_pad_reflect_smoke.sy");
    let options = DriverOptions::default();
    // run_file_printed compiles; skip a second full Linalg lex pass.
    let (code, lines) = run_file_printed(&path, &options).expect("run pad_reflect");
    assert_eq!(code, 0);
    let nums: Vec<&str> = lines.iter().map(|s| s.trim()).collect();
    eprintln!("pad reflect debug nums={nums:?}");
    assert_eq!(
        nums,
        [
            "3", "4", "1", "2", "3", "4", "1", "2", "3", "4", "1", "2", "2", "1", "2", "4", "3",
            "4", "1", "2", "1", "3", "4", "3"
        ],
        "pad reflect nails got {lines:?}"
    );
}

#[test]
fn plugin_pad_replicate_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_zeros_in_process();
    syx_host_plugin::register_faer_mul_in_process();
    syx_host_plugin::register_faer_solve_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_pad_replicate_smoke.sy");
    let options = DriverOptions::default();
    let (code, lines) = run_file_printed(&path, &options).expect("run pad_replicate");
    assert_eq!(code, 0);
    let nums: Vec<&str> = lines.iter().map(|s| s.trim()).collect();
    eprintln!("pad replicate debug nums={nums:?}");
    assert_eq!(
        nums,
        [
            "1", "2", "1", "2", "3", "4", "1", "2", "3", "4", "3", "4", "1", "1", "2", "3", "3",
            "4", "1", "2", "2", "3", "4", "4"
        ],
        "pad replicate nails got {lines:?}"
    );
}

#[test]
fn plugin_pad_circular_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_zeros_in_process();
    syx_host_plugin::register_faer_mul_in_process();
    syx_host_plugin::register_faer_solve_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_pad_circular_smoke.sy");
    let options = DriverOptions::default();
    let (code, lines) = run_file_printed(&path, &options).expect("run pad_circular");
    assert_eq!(code, 0);
    let nums: Vec<&str> = lines.iter().map(|s| s.trim()).collect();
    eprintln!("pad circular debug nums={nums:?}");
    assert_eq!(
        nums,
        [
            "5", "6", "1", "2", "3", "4", "5", "6", "1", "2", "3", "4", "5", "6", "1", "2", "3",
            "1", "2", "3", "6", "4", "5", "6", "1", "2", "3", "1", "4", "5", "6", "4"
        ],
        "pad circular nails got {lines:?}"
    );
}

#[test]
fn plugin_mat_mul_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_zeros_in_process();
    syx_host_plugin::register_faer_mul_in_process();
    syx_host_plugin::register_faer_add_in_process();
    syx_host_plugin::register_faer_sub_in_process();
    syx_host_plugin::register_faer_hadamard_in_process();
    syx_host_plugin::register_faer_solve_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_mat_mul.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile plugin_mat_mul");
    let (code, lines) = run_file_printed(&path, &options).expect("run plugin_mat_mul");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("1")
            && joined.contains("2")
            && joined.contains("3")
            && joined.contains("4"),
        "Mat * I must print 1 2 3 4, got {lines:?}"
    );
    let nums: Vec<&str> = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| {
            *s == "0"
                || *s == "1"
                || *s == "2"
                || *s == "0.25"
                || *s == "0.5"
                || *s == "1.5"
                || *s == "2.5"
                || *s == "3.5"
                || *s == "3"
                || *s == "4"
                || *s == "5"
                || *s == "6"
                || *s == "7"
                || *s == "8"
                || *s == "9"
                || *s == "10"
                || *s == "11"
                || *s == "12"
                || *s == "13"
                || *s == "14"
                || *s == "15"
                || *s == "16"
                || *s == "17"
                || *s == "18"
                || *s == "20"
                || *s == "22"
                || *s == "24"
                || *s == "27"
                || *s == "30"
                || *s == "33"
                || *s == "36"
                || *s == "40"
                || *s == "42"
                || *s == "44"
                || *s == "64"
                || *s == "100"
                || *s == "1000"
                || *s == "-1"
                || *s == "-2"
                || *s == "-3"
                || *s == "-4"
                || *s == "-0.5"
                || *s == "-1.5"
                || *s == "-9"
                || *s == "-18"
                || *s == "-27"
                || *s == "-36"
                || s.starts_with("5.477")
                || s.starts_with("3.741")
                || s.starts_with("1.870")
                || s.starts_with("0.91287")
                || s.starts_with("0.08712")
                || s.starts_with("0.18257")
                || s.starts_with("0.36514")
                || s.starts_with("0.54772")
                || s.starts_with("0.73029")
                || s.starts_with("-1.34164")
                || s.starts_with("-0.44721")
                || s.starts_with("0.44721")
                || s.starts_with("1.34164")
                || s.starts_with("0.33333")
                || s.starts_with("0.66666")
                || s.starts_with("2.71828")
                || s.starts_with("7.38905")
                || s.starts_with("20.08553")
                || s.starts_with("0.69314")
                || s.starts_with("1.38629")
                || s.starts_with("2.07944")
                || s.starts_with("1.31326")
                || s.starts_with("2.12692")
                || s.starts_with("3.04858")
                || s.starts_with("0.73105")
                || s.starts_with("0.88079")
                || s.starts_with("0.95257")
                || s.starts_with("0.84270")
                || s.starts_with("-0.84270")
                || s.starts_with("0.99532")
                || s.starts_with("0.15729")
                || s.starts_with("1.84270")
                || s.starts_with("0.00467")
                || s.starts_with("0.49999")
                || s.starts_with("-0.63212")
                || s.starts_with("0.84134")
                || s.starts_with("-0.15865")
                || s.starts_with("1.95449")
                || s.starts_with("0.76159")
                || s.starts_with("-0.76159")
                || s.starts_with("0.96402")
                || s.starts_with("-0.26894")
                || s.starts_with("1.76159")
                || s.starts_with("-1.11133")
                || s.starts_with("1.05070")
                || s.starts_with("2.10140")
                || s.starts_with("0.86509")
                || s.starts_with("-0.30340")
                || s.starts_with("1.94395")
                || *s == "0.1"
                || *s == "-0.1"
                || *s == "0.3"
                || *s == "0.7"
                || *s == "0.9"
                || *s == "2.5"
                || *s == "1.5"
                || *s == "3.5"
        })
        .collect();
    assert!(
        nums.len() >= 322
            && nums[0] == "1"
            && nums[1] == "2"
            && nums[2] == "3"
            && nums[3] == "4"
            && nums[4] == "1"
            && nums[5] == "0"
            && nums[6] == "0"
            && nums[7] == "1"
            && nums[8] == "2"
            && nums[9] == "4"
            && nums[10] == "6"
            && nums[11] == "8"
            && nums[12] == "1"
            && nums[13] == "2"
            && nums[14] == "3"
            && nums[15] == "4"
            && nums[16] == "0"
            && nums[17] == "0"
            && nums[18] == "0"
            && nums[19] == "0"
            && nums[20] == "2"
            && nums[21] == "2"
            && nums[22] == "2"
            && nums[23] == "2"
            && nums[24] == "1"
            && nums[25] == "2"
            && nums[26] == "2"
            && nums[27] == "0"
            && nums[28] == "0"
            && nums[29] == "2"
            && nums[30] == "2"
            && nums[31] == "1"
            && nums[32] == "2"
            && nums[33] == "1"
            && nums[34] == "3"
            && nums[35] == "2"
            && nums[36] == "4"
            && nums[37] == "1"
            && nums[38] == "2"
            && nums[39] == "3"
            && nums[40] == "4"
            && nums[41] == "1"
            && nums[42] == "2"
            && nums[43] == "3"
            && nums[44] == "4"
            && nums[45] == "1"
            && nums[46] == "2"
            && nums[47] == "3"
            && nums[48] == "4"
            && nums[49] == "1"
            && nums[50] == "2"
            && nums[51] == "3"
            && nums[52] == "4"
            && nums[53] == "1"
            && nums[54] == "1"
            && nums[55] == "1"
            && nums[56] == "1"
            && nums[57] == "4"
            && nums[58] == "0"
            && nums[59] == "1"
            && nums[60] == "0"
            && nums[61] == "0"
            && nums[62] == "0"
            && nums[63] == "1"
            && nums[64] == "0"
            && nums[65] == "0"
            && nums[66] == "0"
            && nums[67] == "1"
            && nums[68] == "1"
            && nums[69] == "1"
            && nums[70] == "1"
            && nums[71] == "1"
            && nums[72] == "1"
            && nums[73] == "1"
            && nums[74] == "6"
            && nums[75] == "0"
            && nums[76] == "0"
            && nums[77] == "0"
            && nums[78] == "0"
            && nums[79] == "1"
            && nums[80] == "4"
            && nums[81] == "2"
            && nums[82] == "5"
            && nums[83] == "3"
            && nums[84] == "6"
            && nums[85] == "1"
            && nums[86] == "2"
            && nums[87] == "3"
            && nums[88] == "4"
            && nums[89] == "5"
            && nums[90] == "6"
            && nums[91] == "1"
            && nums[92] == "2"
            && nums[93] == "3"
            && nums[94] == "4"
            && nums[95] == "5"
            && nums[96] == "6"
            && nums[97] == "2"
            && nums[98] == "4"
            && nums[99] == "6"
            && nums[100] == "8"
            && nums[101] == "1"
            && nums[102] == "2"
            && nums[103] == "3"
            && nums[104] == "4"
            && nums[105] == "11"
            && nums[106] == "22"
            && nums[107] == "33"
            && nums[108] == "44"
            && nums[109] == "1"
            && nums[110] == "2"
            && nums[111] == "3"
            && nums[112] == "4"
            && nums[113] == "10"
            && nums[114] == "20"
            && nums[115] == "30"
            && nums[116] == "40"
            && nums[117] == "-9"
            && nums[118] == "-18"
            && nums[119] == "-27"
            && nums[120] == "-36"
            && nums[121] == "1"
            && nums[122] == "2"
            && nums[123] == "3"
            && nums[124] == "4"
            && nums[125] == "-1"
            && nums[126] == "-2"
            && nums[127] == "-3"
            && nums[128] == "-4"
            && nums[129] == "1"
            && nums[130] == "2"
            && nums[131] == "3"
            && nums[132] == "4"
            && nums[133] == "1"
            && nums[134] == "4"
            && nums[135] == "9"
            && nums[136] == "16"
            && nums[137] == "1"
            && nums[138] == "2"
            && nums[139] == "3"
            && nums[140] == "4"
            && nums[141] == "7"
            && nums[142] == "7"
            && nums[143] == "7"
            && nums[144] == "7"
            && nums[145] == "7"
            && nums[146] == "7"
            && nums[147] == "42"
            && nums[148] == "1"
            && nums[149] == "2.5"
            && nums[150] == "1"
            && nums[151] == "9"
            && nums[152] == "3"
            && nums[153] == "4"
            && nums[154] == "1"
            && nums[155] == "2"
            && nums[156] == "3"
            && nums[157] == "4"
            && nums[158] == "3"
            && nums[159] == "4"
            && nums[160] == "1"
            && nums[161] == "2"
            && nums[162] == "1"
            && nums[163] == "3"
            && nums[164] == "2"
            && nums[165] == "1"
            && nums[166] == "1"
            && nums[167] == "4"
            && nums[168] == "5"
            && nums[169] == "1"
            && nums[170] == "2"
            && nums[171] == "3"
            && nums[172] == "4"
            && nums[173] == "1"
            && nums[174] == "2"
            && nums[175] == "3"
            && nums[176] == "4"
            && nums[177] == "1"
            && nums[178] == "4"
            && nums[179] == "1"
            && nums[180] == "2"
            && nums[181] == "3"
            && nums[182] == "4"
            && nums[183] == "2"
            && nums[184] == "2"
            && nums[185] == "1"
            && nums[186] == "2"
            && nums[187] == "3"
            && nums[188] == "4"
            && nums[189] == "1"
            && nums[190] == "4"
            && nums[191] == "1"
            && nums[192] == "2"
            && nums[193] == "10"
            && nums[194] == "20"
            && nums[195] == "3"
            && nums[196] == "4"
            && nums[197] == "30"
            && nums[198] == "40"
            && nums[199] == "2"
            && nums[200] == "4"
            && nums[201] == "1"
            && nums[202] == "2"
            && nums[203] == "3"
            && nums[204] == "4"
            && nums[205] == "10"
            && nums[206] == "20"
            && nums[207] == "30"
            && nums[208] == "40"
            && nums[209] == "1"
            && nums[210] == "2"
            && nums[211] == "3"
            && nums[212] == "4"
            && nums[213] == "10"
            && nums[214] == "20"
            && nums[215] == "30"
            && nums[216] == "40"
            && nums[217] == "4"
            && nums[218] == "2"
            && nums[219] == "1"
            && nums[220] == "0"
            && nums[221] == "0"
            && nums[222] == "4"
            && nums[223] == "2"
            && nums[224] == "2"
            && nums[225] == "1"
            && nums[226] == "4"
            && nums[227] == "0"
            && nums[228] == "3"
            && nums[229].starts_with("5.477")
            && nums[230] == "1"
            && nums[231] == "2"
            && nums[232] == "1"
            && nums[233] == "2"
            && nums[234] == "3"
            && nums[235] == "4"
            && nums[236] == "3"
            && nums[237] == "4"
            && nums[238] == "2"
            && nums[239] == "4"
            && nums[240] == "1"
            && nums[241] == "2"
            && nums[242] == "3"
            && nums[243] == "4"
            && nums[244] == "4"
            && nums[245] == "4"
            && nums[246] == "1"
            && nums[247] == "1"
            && nums[248] == "4"
            && nums[249] == "4"
            && nums[250] == "0"
            && nums[251] == "0"
            && nums[252] == "1"
            && nums[253] == "0"
            && nums[254] == "3"
            && nums[255] == "4"
            && nums[256] == "1"
            && nums[257] == "2"
            && nums[258] == "0"
            && nums[259] == "4"
            && nums[260] == "3"
            && nums[261] == "4"
            && nums[262] == "1"
            && nums[263] == "2"
            && nums[264] == "2"
            && nums[265] == "1"
            && nums[266] == "4"
            && nums[267] == "3"
            && nums[268] == "2"
            && nums[269] == "4"
            && nums[270] == "1"
            && nums[271] == "3"
            && nums[272] == "2"
            && nums[273] == "2"
            && nums[274] == "2"
            && nums[275] == "3"
            && nums[276] == "2"
            && nums[277] == "1"
            && nums[278] == "4"
            && nums[279] == "3"
            && nums[280] == "2"
            && nums[281] == "1"
            && nums[282] == "3"
            && nums[283] == "4"
            && nums[284] == "1"
            && nums[285] == "2"
            && nums[286] == "2"
            && nums[287] == "1"
            && nums[288] == "4"
            && nums[289] == "3"
            && nums[290] == "1"
            && nums[291] == "2"
            && nums[292] == "3"
            && nums[293] == "4"
            && nums[294] == "3"
            && nums[295] == "1"
            && nums[296] == "2"
            && nums[297] == "6"
            && nums[298] == "4"
            && nums[299] == "5"
            && nums[300] == "2"
            && nums[301] == "3"
            && nums[302] == "-1"
            && nums[303] == "1"
            && nums[304] == "-1"
            && nums[305] == "0"
            && nums[306] == "1"
            && nums[307] == "2"
            && nums[308] == "3"
            && nums[309] == "4"
            && nums[310] == "1"
            && nums[311] == "4"
            && nums[312] == "9"
            && nums[313] == "16"
            && nums[314] == "2"
            && nums[315] == "2"
            && nums[316] == "3"
            && nums[317] == "3"
            && nums[318] == "1"
            && nums[319] == "2"
            && nums[320] == "3"
            && nums[321] == "4"
            && nums.len() >= 604
            && nums[322] == "1"
            && nums[323] == "2"
            && nums[324] == "3"
            && nums[325] == "4"
            && nums[326] == "11"
            && nums[327] == "12"
            && nums[328] == "13"
            && nums[329] == "14"
            && nums[330] == "1"
            && nums[331] == "2"
            && nums[332] == "3"
            && nums[333] == "4"
            && nums[334] == "0"
            && nums[335] == "1"
            && nums[336] == "2"
            && nums[337] == "3"
            && nums[338] == "1"
            && nums[339] == "2"
            && nums[340] == "3"
            && nums[341] == "4"
            && nums[342] == "1"
            && nums[343] == "2"
            && nums[344] == "3"
            && nums[345] == "4"
            && nums[346] == "2"
            && nums[347] == "3"
            && nums[348] == "4"
            && nums[349] == "5"
            && nums[350] == "1"
            && nums[351] == "2"
            && nums[352] == "2"
            && nums[353] == "3"
            && nums[354] == "1"
            && nums[355] == "-2"
            && nums[356] == "3"
            && nums[357] == "-4"
            && nums[358] == "1"
            && nums[359] == "2"
            && nums[360] == "3"
            && nums[361] == "4"
            && nums[362] == "2"
            && nums[363] == "4"
            && nums[364] == "3"
            && nums[365] == "4"
            && nums[366] == "1"
            && nums[367] == "2"
            && nums[368] == "2"
            && nums[369] == "3"
            && nums[370] == "-1"
            && nums[371] == "2"
            && nums[372] == "-3"
            && nums[373] == "4"
            && nums[374] == "1"
            && nums[375] == "0.5"
            && nums[376] == "1"
            && nums[377] == "0.5"
            && nums[378] == "1"
            && nums[379] == "2"
            && nums[380] == "3"
            && nums[381] == "4"
            && nums[382] == "5"
            && nums[383] == "13"
            && nums[384] == "17"
            && nums[385] == "0"
            && nums[386] == "4"
            && nums[387] == "9"
            && nums[388] == "8"
            && nums[389] == "2"
            && nums[390] == "2"
            && nums[391] == "1"
            && nums[392] == "1"
            && nums[393] == "3"
            && nums[394] == "2"
            && nums[395] == "1"
            && nums[396] == "1"
            && nums[397] == "3"
            && nums[398] == "1"
            && nums[399] == "2"
            && nums[400] == "3"
            && nums[401] == "4"
            && nums[402] == "3"
            && nums[403] == "5"
            && nums[404] == "7"
            && nums[405] == "9"
            && nums[406] == "1"
            && nums[407] == "2"
            && nums[408] == "3"
            && nums[409] == "4"
            && nums[410] == "1"
            && nums[411] == "2"
            && nums[412] == "4"
            && nums[413] == "8"
            && nums[414] == "0"
            && nums[415] == "1"
            && nums[416] == "2"
            && nums[417] == "3"
            && nums[418] == "2"
            && nums[419] == "3"
            && nums[420] == "1"
            && nums[421] == "4"
            && nums[422] == "0"
            && nums[423] == "1"
            && nums[424] == "2"
            && nums[425] == "3"
            && nums[426] == "1"
            && nums[427] == "1"
            && nums[428] == "2"
            && nums[429] == "6"
            && nums[430] == "1"
            && nums[431] == "3"
            && nums[432] == "5"
            && nums[433] == "7"
            && nums[434] == "1"
            && nums[435] == "2"
            && nums[436] == "3"
            && nums[437] == "4"
            && nums[438] == "4"
            && nums[439] == "1"
            && nums[440] == "3"
            && nums[441] == "6"
            && nums[442] == "10"
            && nums[443] == "1"
            && nums[444] == "2"
            && nums[445] == "6"
            && nums[446] == "24"
            && nums[447] == "1"
            && nums[448] == "2"
            && nums[449] == "3"
            && nums[450] == "4"
            && nums[451] == "1"
            && nums[452] == "2"
            && nums[453] == "3"
            && nums[454] == "4"
            && nums[455] == "2"
            && nums[456] == "3"
            && nums[457] == "4"
            && nums[458] == "6"
            && nums[459] == "8"
            && nums[460] == "1"
            && nums[461] == "1"
            && nums[462] == "1"
            && nums[463] == "1"
            && nums[464] == "1"
            && nums[465] == "2"
            && nums[466] == "3"
            && nums[467] == "4"
            && nums[468] == "1"
            && nums[469] == "0"
            && nums[470] == "1"
            && nums[471] == "0"
            && nums[472] == "1"
            && nums[473] == "0"
            && nums[474] == "1"
            && nums[475] == "0"
            && nums[476] == "0"
            && nums[477] == "1"
            && nums[478] == "0"
            && nums[479] == "1"
            && nums[480] == "0"
            && nums[481] == "0"
            && nums[482] == "0"
            && nums[483] == "1"
            && nums[484] == "0"
            && nums[485] == "1"
            && nums[486] == "0"
            && nums[487] == "1"
            && nums[488] == "1"
            && nums[489] == "2"
            && nums[490] == "3"
            && nums[491] == "4"
            && nums[492] == "1"
            && nums[493] == "1"
            && nums[494] == "1"
            && nums[495] == "0"
            && nums[496] == "0"
            && nums[497] == "0"
            && nums[498] == "0"
            && nums[499] == "0"
            && nums[500] == "1"
            && nums[501] == "2"
            && nums[502] == "3"
            && nums[503] == "4"
            && nums[504] == "7"
            && nums[505] == "7"
            && nums[506] == "7"
            && nums[507] == "7"
            && nums[508] == "1"
            && nums[509] == "9"
            && nums[510] == "3"
            && nums[511] == "9"
            && nums[512] == "0"
            && nums[513] == "1"
            && nums[514] == "0"
            && nums[515] == "1"
            && nums[516] == "1"
            && nums[517] == "0"
            && nums[518] == "0"
            && nums[519] == "0"
            && nums[520] == "1"
            && nums[521] == "1"
            && nums[522] == "1"
            && nums[523] == "0"
            && nums[524] == "1"
            && nums[525] == "0"
            && nums[526] == "1"
            && nums[527] == "2"
            && nums[528] == "3"
            && nums[529] == "4"
            && nums[530] == "0"
            && nums[531] == "1"
            && nums[532] == "1"
            && nums[533] == "0"
            && nums[534] == "2"
            && nums[535] == "1"
            && nums[536] == "9"
            && nums[537] == "2"
            && nums[538] == "9"
            && nums[539] == "4"
            && nums[540] == "1"
            && nums[541] == "2"
            && nums[542] == "3"
            && nums[543] == "4"
            && nums[544] == "3"
            && nums[545] == "7"
            && nums[546] == "4"
            && nums[547] == "6"
            && nums[548] == "1"
            && nums[549] == "3"
            && nums[550] == "3"
            && nums[551] == "4"
            && nums[552] == "1"
            && nums[553] == "2"
            && nums[554] == "3"
            && nums[555] == "4"
            && nums[556] == "1.5"
            && nums[557] == "3.5"
            && nums[558] == "3"
            && nums[559] == "8"
            && nums[560] == "2"
            && nums[561] == "4"
            && nums[562] == "1"
            && nums[563] == "2"
            && nums[564] == "1"
            && nums[565] == "2"
            && nums[566] == "3"
            && nums[567] == "4"
            && nums[568] == "2"
            && nums[569] == "3"
            && nums[570] == "2"
            && nums[571] == "12"
            && nums[572] == "0"
            && nums[573] == "0"
            && nums[574] == "1"
            && nums[575] == "1"
            && nums[576] == "1"
            && nums[577] == "2"
            && nums[578] == "3"
            && nums[579] == "4"
            && nums[580] == "0"
            && nums[581] == "0"
            && nums[582] == "1"
            && nums[583] == "1"
            && nums[584] == "0.5"
            && nums[585] == "0.5"
            && nums[586] == "1"
            && nums[587] == "1"
            && nums[588] == "1"
            && nums[589] == "2"
            && nums[590] == "3"
            && nums[591] == "4"
            && nums[592] == "0.25"
            && nums[593] == "0.25"
            && nums[594] == "1"
            && nums[595] == "1"
            && nums[596] == "1"
            && nums[597] == "1"
            && nums[598] == "2"
            && nums[599] == "2"
            && nums[600] == "1"
            && nums[601] == "2"
            && nums[602] == "3"
            && nums[603] == "4"
            && nums.len() >= 624
            && nums[604] == "1"
            && nums[605] == "3"
            && nums[606] == "3"
            && nums[607] == "7"
            && nums[608] == "1"
            && nums[609] == "2"
            && nums[610] == "4"
            && nums[611] == "6"
            && nums[612] == "1"
            && nums[613] == "2"
            && nums[614] == "3"
            && nums[615] == "12"
            && nums[616] == "1"
            && nums[617] == "2"
            && nums[618] == "3"
            && nums[619] == "8"
            && nums[620] == "1"
            && nums[621] == "2"
            && nums[622] == "3"
            && nums[623] == "4"
            && nums.len() >= 644
            && nums[624] == "1"
            && nums[625] == "1"
            && nums[626] == "3"
            && nums[627] == "1"
            && nums[628] == "1"
            && nums[629] == "2"
            && nums[630] == "2"
            && nums[631] == "2"
            && nums[632] == "1"
            && nums[633] == "1"
            && nums[634] == "3"
            && nums[635] == "3"
            && nums[636] == "1"
            && nums[637] == "2"
            && nums[638] == "1"
            && nums[639] == "2"
            && nums[640] == "1"
            && nums[641] == "2"
            && nums[642] == "3"
            && nums[643] == "4"
            && nums.len() >= 664
            && nums[644] == "1"
            && nums[645] == "2"
            && nums[646] == "3"
            && nums[647] == "4"
            && nums[648] == "1"
            && nums[649] == "2"
            && nums[650] == "3"
            && nums[651] == "4"
            && nums[652] == "1"
            && nums[653] == "1"
            && nums[654] == "1"
            && nums[655] == "1"
            && nums[656] == "1"
            && nums[657] == "2"
            && nums[658] == "3"
            && nums[659] == "4"
            && nums[660] == "1"
            && nums[661] == "2"
            && nums[662] == "3"
            && nums[663] == "4"
            && nums.len() >= 684
            && nums[664] == "2"
            && nums[665] == "2"
            && nums[666] == "3"
            && nums[667] == "4"
            && nums[668] == "1"
            && nums[669] == "2"
            && nums[670] == "3"
            && nums[671] == "3"
            && nums[672] == "0"
            && nums[673] == "2"
            && nums[674] == "0"
            && nums[675] == "4"
            && nums[676] == "0"
            && nums[677] == "0"
            && nums[678] == "3"
            && nums[679] == "4"
            && nums[680] == "1"
            && nums[681] == "2"
            && nums[682] == "3"
            && nums[683] == "4"
            && nums.len() >= 704
            && nums[684] == "-1"
            && nums[685] == "2"
            && nums[686] == "-2"
            && nums[687] == "4"
            && nums[688] == "-1"
            && nums[689] == "1"
            && nums[690] == "-1"
            && nums[691] == "1"
            && nums[692] == "0"
            && nums[693] == "1"
            && nums[694] == "0"
            && nums[695] == "1"
            && nums[696] == "6"
            && nums[697] == "0"
            && nums[698] == "6"
            && nums[699] == "2"
            && nums[700] == "1"
            && nums[701] == "2"
            && nums[702] == "3"
            && nums[703] == "4"
            && nums.len() >= 920
            && nums[704] == "0"
            && nums[705] == "1"
            && nums[706] == "2"
            && nums[707] == "3"
            && nums[708] == "0"
            && nums[709] == "1"
            && nums[710] == "4"
            && nums[711] == "9"
            && nums[712] == "6"
            && nums[713] == "14"
            && nums[714] == "1"
            && nums[715] == "2"
            && nums[716] == "3"
            && nums[717] == "4"
            && nums[718] == "3"
            && nums[719] == "3"
            && nums[720] == "1"
            && nums[721] == "0"
            && nums[722] == "0"
            && nums[723] == "0"
            && nums[724] == "0"
            && nums[725] == "0"
            && nums[726] == "1"
            && nums[727] == "1"
            && nums[728] == "1"
            && nums[729] == "2"
            && nums[730] == "3"
            && nums[731] == "4"
            && nums[732] == "1"
            && nums[733] == "1"
            && nums[734] == "0"
            && nums[735] == "0"
            && nums[736] == "1"
            && nums[737] == "0"
            && nums[738] == "1"
            && nums[739] == "1"
            && nums[740] == "1"
            && nums[741] == "1"
            && nums[742] == "0"
            && nums[743] == "0"
            && nums[744] == "0"
            && nums[745] == "0"
            && nums[746] == "1"
            && nums[747] == "1"
            && nums[748] == "1"
            && nums[749] == "2"
            && nums[750] == "3"
            && nums[751] == "4"
            && nums[752] == "1"
            && nums[753] == "2"
            && nums[754] == "3"
            && nums[755] == "4"
            && nums[756] == "1"
            && nums[757] == "3"
            && nums[758] == "2"
            && nums[759] == "4"
            && nums[760] == "1"
            && nums[761] == "2"
            && nums[762] == "3"
            && nums[763] == "4"
            && nums[764] == "0"
            && nums[765] == "0"
            && nums[766] == "1"
            && nums[767] == "2"
            && nums[768] == "3"
            && nums[769] == "4"
            && nums[770] == "1"
            && nums[771] == "2"
            && nums[772] == "3"
            && nums[773] == "4"
            && nums[774] == "0"
            && nums[775] == "0"
            && nums[776] == "0"
            && nums[777] == "1"
            && nums[778] == "2"
            && nums[779] == "0"
            && nums[780] == "3"
            && nums[781] == "4"
            && nums[782] == "1"
            && nums[783] == "2"
            && nums[784] == "0"
            && nums[785] == "3"
            && nums[786] == "4"
            && nums[787] == "0"
            && nums[788] == "1"
            && nums[789] == "2"
            && nums[790] == "3"
            && nums[791] == "4"
            && nums[792] == "9"
            && nums[793] == "9"
            && nums[794] == "1"
            && nums[795] == "2"
            && nums[796] == "3"
            && nums[797] == "4"
            && nums[798] == "1"
            && nums[799] == "2"
            && nums[800] == "3"
            && nums[801] == "4"
            && nums[802] == "9"
            && nums[803] == "9"
            && nums[804] == "9"
            && nums[805] == "1"
            && nums[806] == "2"
            && nums[807] == "9"
            && nums[808] == "3"
            && nums[809] == "4"
            && nums[810] == "1"
            && nums[811] == "2"
            && nums[812] == "9"
            && nums[813] == "3"
            && nums[814] == "4"
            && nums[815] == "9"
            && nums[816] == "1"
            && nums[817] == "2"
            && nums[818] == "3"
            && nums[819] == "4"
            && nums[820] == "3"
            && nums[821] == "4"
            && nums[822] == "1"
            && nums[823] == "2"
            && nums[824] == "3"
            && nums[825] == "4"
            && nums[826] == "1"
            && nums[827] == "2"
            && nums[828] == "3"
            && nums[829] == "4"
            && nums[830] == "1"
            && nums[831] == "2"
            && nums[832] == "2"
            && nums[833] == "1"
            && nums[834] == "2"
            && nums[835] == "4"
            && nums[836] == "3"
            && nums[837] == "4"
            && nums[838] == "1"
            && nums[839] == "2"
            && nums[840] == "1"
            && nums[841] == "3"
            && nums[842] == "4"
            && nums[843] == "3"
            && nums[844] == "1"
            && nums[845] == "2"
            && nums[846] == "3"
            && nums[847] == "4"
            && nums[848] == "1"
            && nums[849] == "2"
            && nums[850] == "1"
            && nums[851] == "2"
            && nums[852] == "3"
            && nums[853] == "4"
            && nums[854] == "1"
            && nums[855] == "2"
            && nums[856] == "3"
            && nums[857] == "4"
            && nums[858] == "3"
            && nums[859] == "4"
            && nums[860] == "1"
            && nums[861] == "1"
            && nums[862] == "2"
            && nums[863] == "3"
            && nums[864] == "3"
            && nums[865] == "4"
            && nums[866] == "1"
            && nums[867] == "2"
            && nums[868] == "2"
            && nums[869] == "3"
            && nums[870] == "4"
            && nums[871] == "4"
            && nums[872] == "1"
            && nums[873] == "2"
            && nums[874] == "3"
            && nums[875] == "4"
            && nums[876] == "5"
            && nums[877] == "6"
            && nums[878] == "1"
            && nums[879] == "2"
            && nums[880] == "3"
            && nums[881] == "4"
            && nums[882] == "5"
            && nums[883] == "6"
            && nums[884] == "1"
            && nums[885] == "2"
            && nums[886] == "3"
            && nums[887] == "4"
            && nums[888] == "5"
            && nums[889] == "6"
            && nums[890] == "1"
            && nums[891] == "2"
            && nums[892] == "3"
            && nums[893] == "1"
            && nums[894] == "2"
            && nums[895] == "3"
            && nums[896] == "6"
            && nums[897] == "4"
            && nums[898] == "5"
            && nums[899] == "6"
            && nums[900] == "1"
            && nums[901] == "2"
            && nums[902] == "3"
            && nums[903] == "1"
            && nums[904] == "4"
            && nums[905] == "5"
            && nums[906] == "6"
            && nums[907] == "4"
            && nums[908] == "1"
            && nums[909] == "2"
            && nums[910] == "3"
            && nums[911] == "4"
            && nums[912] == "5"
            && nums[913] == "6"
            && nums[914] == "1"
            && nums[915] == "2"
            && nums[916] == "3"
            && nums[917] == "4"
            && nums[918] == "5"
            && nums[919] == "6"
            && nums.len() >= 928
            && nums[920] == "1.5"
            && nums[921] == "3.5"
            && nums[922].starts_with("3.741")
            && nums[923].starts_with("1.870")
            && nums[924] == "1"
            && nums[925] == "2"
            && nums[926] == "3"
            && nums[927] == "4"
            && nums.len() >= 936
            && nums[928] == "1"
            && nums[929] == "0"
            && nums[930].starts_with("0.91287")
            && nums[931].starts_with("0.08712")
            && nums[932] == "1"
            && nums[933] == "2"
            && nums[934] == "3"
            && nums[935] == "4"
            && nums.len() >= 956
            && nums[936].starts_with("0.18257")
            && nums[937].starts_with("0.36514")
            && nums[938].starts_with("0.54772")
            && nums[939].starts_with("0.73029")
            && nums[940] == "-1.5"
            && nums[941] == "-0.5"
            && nums[942] == "0.5"
            && nums[943] == "1.5"
            && nums[944].starts_with("-1.34164")
            && nums[945].starts_with("-0.44721")
            && nums[946].starts_with("0.44721")
            && nums[947].starts_with("1.34164")
            && nums[948] == "0"
            && nums[949].starts_with("0.33333")
            && nums[950].starts_with("0.66666")
            && nums[951] == "1"
            && nums[952] == "1"
            && nums[953] == "2"
            && nums[954] == "3"
            && nums[955] == "4"
            && nums.len() >= 976
            && nums[956] == "1"
            && nums[957].starts_with("2.71828")
            && nums[958].starts_with("7.38905")
            && nums[959].starts_with("20.08553")
            && nums[960] == "0"
            && nums[961].starts_with("0.69314")
            && nums[962].starts_with("1.38629")
            && nums[963].starts_with("2.07944")
            && nums[964].starts_with("0.69314")
            && nums[965].starts_with("1.31326")
            && nums[966].starts_with("2.12692")
            && nums[967].starts_with("3.04858")
            && nums[968] == "0.5"
            && nums[969].starts_with("0.73105")
            && nums[970].starts_with("0.88079")
            && nums[971].starts_with("0.95257")
            && nums[972] == "1"
            && nums[973] == "2"
            && nums[974] == "3"
            && nums[975] == "4"
            && nums.len() >= 996
            && nums[976] == "0"
            && nums[977] == "1"
            && nums[978] == "0"
            && (nums[979] == "0.5" || nums[979].starts_with("0.49999"))
            && nums[980] == "1"
            && nums[981] == "-1"
            && nums[982] == "1"
            && nums[983] == "-1"
            && nums[984] == "0"
            && nums[985].starts_with("0.84270")
            && nums[986].starts_with("-0.84270")
            && nums[987].starts_with("0.99532")
            && nums[988] == "1"
            && nums[989].starts_with("0.15729")
            && nums[990].starts_with("1.84270")
            && nums[991].starts_with("0.00467")
            && nums[992] == "1"
            && nums[993] == "2"
            && nums[994] == "3"
            && nums[995] == "4"
            && nums.len() >= 1016
            && nums[996].starts_with("-0.63212")
            && nums[997] == "0"
            && nums[998] == "1"
            && nums[999] == "2"
            && nums[1000] == "0"
            && nums[1001].starts_with("0.84134")
            && nums[1002].starts_with("-0.15865")
            && nums[1003].starts_with("1.95449")
            && nums[1004] == "0"
            && nums[1005].starts_with("0.76159")
            && nums[1006].starts_with("-0.76159")
            && nums[1007].starts_with("0.96402")
            && nums[1008] == "0"
            && nums[1009].starts_with("0.73105")
            && nums[1010].starts_with("-0.26894")
            && nums[1011].starts_with("1.76159")
            && nums[1012] == "1"
            && nums[1013] == "2"
            && nums[1014] == "3"
            && nums[1015] == "4"
            && nums.len() >= 1036
            && nums[1016].starts_with("-1.11133")
            && nums[1017] == "0"
            && nums[1018].starts_with("1.05070")
            && nums[1019].starts_with("2.10140")
            && nums[1020].starts_with("-0.63212")
            && nums[1021] == "0"
            && nums[1022] == "1"
            && nums[1023] == "2"
            && nums[1024] == "0"
            && nums[1025].starts_with("0.86509")
            && nums[1026].starts_with("-0.30340")
            && nums[1027].starts_with("1.94395")
            && nums[1028] == "0"
            && nums[1029] == "0.5"
            && nums[1030] == "-0.5"
            && nums[1031].starts_with("0.66666")
            && nums[1032] == "1"
            && nums[1033] == "2"
            && nums[1034] == "3"
            && nums[1035] == "4"
            && nums.len() >= 1056
            && nums[1036] == "1"
            && nums[1037] == "-1"
            && nums[1038] == "0"
            && nums[1039] == "0"
            && nums[1040] == "2"
            && nums[1041] == "-2"
            && nums[1042] == "0"
            && nums[1043] == "0"
            && nums[1044] == "-0.1"
            && nums[1045] == "0"
            && nums[1046] == "1"
            && nums[1047] == "2"
            && nums[1048] == "0.5"
            && nums[1049] == "0.7"
            && nums[1050] == "0.3"
            && nums[1051] == "0.9"
            && nums[1052] == "1"
            && nums[1053] == "2"
            && nums[1054] == "3"
            && nums[1055] == "4"
            && nums.len() >= 1084
            && nums[1056] == "2.5"
            && nums[1057] == "2.5"
            && nums[1058] == "1"
            && nums[1059] == "2"
            && nums[1060] == "3"
            && nums[1061] == "1.5"
            && nums[1062] == "3.5"
            && nums[1063] == "1"
            && nums[1064] == "2"
            && nums[1065] == "3"
            && nums[1066] == "4"
            && nums[1067] == "2"
            && nums[1068] == "3"
            && nums[1069] == "1"
            && nums[1070] == "2"
            && nums[1071] == "1.5"
            && nums[1072] == "3.5"
            && nums[1073] == "1"
            && nums[1074] == "2"
            && nums[1075] == "3"
            && nums[1076] == "4"
            && nums[1077] == "0.5"
            && nums[1078] == "1"
            && nums[1079].starts_with("0.88079")
            && nums[1080] == "1"
            && nums[1081] == "2"
            && nums[1082] == "3"
            && nums[1083] == "4",
        "W1102..W1118 nails plus glu, got {lines:?}"
    );
}

#[test]
fn plugin_faer_solve_vm() {
    syx_host_plugin::register_faer_from_in_process();
    syx_host_plugin::register_faer_solve_in_process();
    syx_host_plugin::register_faer_to_in_process();
    syx_host_plugin::register_v2probe_drop_in_process();
    let path = fixture("common/plugin_faer_solve.sy");
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile plugin_faer_solve");
    let (code, lines) = run_file_printed(&path, &options).expect("run plugin_faer_solve");
    assert_eq!(code, 0);
    let joined = lines.join("\n");
    assert!(
        joined.contains("1") && joined.contains("2"),
        "faer solve 2I x = [2,4] must print 1 2, got {lines:?}"
    );
}

#[test]
fn plugin_h002_compile_validate() {
    assert!(syx_host_plugin::validate_compile_plugin_lock().is_ok());
}

#[test]
fn plugin_trust_chain_documented() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("ed25519"));
}
