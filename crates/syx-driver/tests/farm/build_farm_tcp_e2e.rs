//! Phase 100: farm TCP frame protocol E2E — GET/PUT length-prefixed; worker get/put hit.

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use syx_driver::build_syx::{parse_build_syx, run_build_tasks, BuildTaskContext};
use syx_driver::{fetch_cache_remote, spawn_farm_tcp_node, DriverOptions};
use syx_pkg::SyxManifest;

use crate::billing_env_lock::billing_env_lock;
use crate::lease_test_harness::with_lease_auth;

#[test]
fn p101_farm_tcp_e2e_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-frame-lease-v1:ttl-err-on-expired"));
}

#[test]
fn p100_build_farm_tcp_e2e_get_put_hit() {
    let _billing = billing_env_lock();
    with_lease_auth(|| {
        std::env::remove_var("SYX_BUILD_FARM_URL");
        std::env::remove_var("SYX_FARM_CACHE_URL");
        std::env::remove_var("SYX_BUILD_FARM_QUEUE");
        std::env::remove_var("SYX_FARM_BILLING_PATH");
        std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");

        let payload = b"p100-queue-job-hit".to_vec();

        // 3 farm nodes pre-seeded for multi-node GET hit.
        let nodes: Vec<_> = (0..3)
            .map(|i| {
                let key = format!("tenant/node-{i}");
                let mut store = HashMap::new();
                store.insert(key, format!("node-{i}").into_bytes());
                spawn_farm_tcp_node(store, 1)
            })
            .collect();
        thread::sleep(Duration::from_millis(30));

        for (i, (addr, _)) in nodes.iter().enumerate() {
            let url = format!("tcp://{addr}/tenant/node-{i}");
            let bytes = fetch_cache_remote(&url).expect("tcp frame GET");
            assert_eq!(bytes, format!("node-{i}").as_bytes());
        }
        for (_, h) in nodes {
            h.join().ok();
        }

        // Queue serves GET with payload; farm accepts PUT then subsequent GET must hit equal bytes.
        let mut queue_store = HashMap::new();
        queue_store.insert("tenant/job.bin".into(), payload.clone());
        let (queue_addr, queue_handle) = spawn_farm_tcp_node(queue_store, 1);

        let (farm_addr, farm_handle) = spawn_farm_tcp_node(HashMap::new(), 2);
        thread::sleep(Duration::from_millis(20));

        let dir = std::env::temp_dir().join(format!("syx_p100_farm_frames_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let build_syx = format!(
            r#"build {{
  task farm_worker {{
    target = "farm_worker"
    queue = "tcp://{queue_addr}/tenant/job.bin"
    farm = "tcp://{farm_addr}/tenant/node-a"
  }}
}}"#
        );
        std::fs::write(dir.join("build.syb"), build_syx).unwrap();
        std::fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        std::fs::write(dir.join("main.sy"), "func main() { print(0) }").unwrap();

        let plan = parse_build_syx(&dir.join("build.syb")).unwrap();
        assert!(plan.farm_url.as_deref().unwrap().starts_with("tcp://"));
        assert!(plan.farm_queue_url.as_deref().unwrap().starts_with("tcp://"));
        let mut opts = DriverOptions::default();
        let mut manifest = SyxManifest::default();
        let mut ctx = BuildTaskContext::new(&dir, &mut opts, &mut manifest);
        run_build_tasks(&plan, &mut ctx).expect("farm_worker TCP frame get/put E2E");
        assert!(ctx.executed.iter().any(|t| t == "farm_worker"));

        queue_handle.join().ok();

        // Real cache hit: GET from farm must equal queue payload.
        let farm_url = format!("tcp://{farm_addr}/tenant/node-a");
        let hit = fetch_cache_remote(&farm_url).expect("farm GET after worker PUT");
        assert_eq!(hit, payload, "worker must produce real cache hit (bytes equal)");
        farm_handle.join().ok();
        let _ = std::fs::remove_dir_all(&dir);
    });
}
