//! SYX driver cache fingerprint / invalidation digests.
//! Extracted in S2-I215 (zero semantics).

use crate::build_syx;
use crate::embedded_std;
use crate::{load_resolved_packages, DriverOptions, ModuleGraph};
use std::path::Path;
use std::sync::OnceLock;
use syx_diagnostics::SyxResult;
use syx_pkg::{load_manifest, SyxManifest};

pub(crate) fn cache_version_tag() -> String {
    // S3-I52: prefer build-time bake — zero runtime heap_v100 / HostFnId::all first-touch.
    if let Some(baked) = option_env!("SYX_CACHE_VERSION_TAG") {
        return baked.to_string();
    }
    static TAG: OnceLock<String> = OnceLock::new();
    TAG.get_or_init(compute_cache_version_tag).clone()
}

pub(crate) fn compute_cache_version_tag() -> String {
    // Historical tags retained for invalidation contract tests: syx-cache-v38 syx-cache-v39 syx-cache-v44 syx-cache-v58 syx-cache-v59 syx-cache-v60 syx-cache-v61 syx-cache-v62 syx-cache-v63 syx-cache-v64 syx-cache-v65 syx-cache-v66 syx-cache-v67 syx-cache-v68 syx-cache-v69 syx-cache-v70 syx-cache-v71 syx-cache-v72 syx-cache-v73 syx-cache-v74 syx-cache-v75 syx-cache-v76 syx-cache-v78 syx-cache-v79 syx-cache-v80 syx-cache-v81 syx-cache-v82 syx-cache-v83 syx-cache-v84 syx-cache-v85 syx-cache-v86 syx-cache-v87 syx-cache-v88 syx-cache-v89 syx-cache-v90 syx-cache-v91 syx-cache-v92 syx-cache-v93 syx-cache-v94 syx-cache-v95 syx-cache-v96 syx-cache-v97 syx-cache-v98 syx-cache-v99 syx-cache-v100 syx-cache-v101 syx-cache-v102 syx-cache-v103 syx-cache-v104 syx-cache-v105 syx-cache-v106 syx-cache-v107 syx-cache-v108 syx-cache-v109 syx-cache-v110
    format!(
        "syx-cache-v1:{}:{}:{}",
        env!("CARGO_PKG_VERSION"),
        rustc_version_tag(),
        heap_v100_phase128_track_c_wasm_digest()
    )
}

/// Phase 131 Track A: Macro M21 decl-param subst MVP + Service DI v12 execution pointcut MVP.
/// Phase 131 Track B: SynUI v8 (textArea/list Signal bind + spacer + Web/Mobile surface deepen).
/// Phase 131 Track C: SIMD16 i8x16 abs MVP.
/// Phase 131 Track D: farm hsm-digest-bound + dist peer-announce fanout.
/// Phase 131 DX: audit 6050/3450, fill EXCLUDE, LSP semantic v33, driver CR370, stdlib 109.0 pin.
/// Phase 130 Track A: Macro M20 token expr MVP + Service DI v11 pointcut-name MVP.
/// Phase 130 Track B: SynUI v7 (image/scrollView + picker Signal bind + Web language surface MVP).
/// Phase 130 Track C: SIMD16 i8x16 unsigned min/max MVP.
/// Phase 130 Track D: farm hsm-sign-verify + dist peer-announce ack.
/// Phase 130 DX: audit 5900/3400, fill EXCLUDE, LSP semantic v32, driver CR360, stdlib 108.0 pin.
/// Phase 129 Track A: Macro M19 Codable L2 associated-enum + Service DI v10 bean-param autowire.
/// Phase 129 Track B: SynUI v6 (textArea/list + progress Number bind + desktop layout GA).
/// Phase 129 Track C: SIMD16 i8x16 load/store MVP.
/// Phase 129 Track D: farm hsm-sign-bound + dist peer-announce sticky.
/// Phase 129 DX: audit 5750/3350, fill EXCLUDE, LSP semantic v31, driver CR350, stdlib 107.0 pin.
/// Phase 128 Track A: Macro M18 + Service DI v9 (lazy + cycle-proxy).
/// Phase 128 Track B: SynUI v5 (toggle/radio bind + progress/picker + mobile unretire).
/// Phase 128 Track D: farm hsm-ref-bound + dist sticky-multi-snapshot.
/// Phase 128 DX: audit 5600/3300, fill EXCLUDE, LSP semantic v30, driver CR340, stdlib 106.0 pin.
/// Phase 127 Track B: SynUI v4 (signal bind + desktop onClick wire + switch/radio).
/// Phase 127 Track C: SIMD16 saturating add/sub MVP + inherited compare/extract.
/// Phase 128 Track C: SIMD16 i8x16 shuffle MVP + inherited sat/compare.
/// Phase 128 Track C (+ folded v97_b…v100_b): invalidation bytes preserved in tip order.
/// Phase 128 Track C (+ folded v52…v97_a kept tips): invalidation bytes preserved in tip order.
/// Phase 128 Track C (+ folded v104_dx/v106_dx/v110_a kept tips): invalidation bytes preserved in tip order.
/// Phase 128 Track C (+ folded foundation×12): invalidation bytes preserved in tip order.
/// Phase 128 Track C (+ folded track_l): invalidation bytes preserved in tip order.
pub(crate) fn heap_v100_phase128_track_c_wasm_digest() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(host_feature_tag().as_bytes());
    hasher.update(witness_table_digest().as_bytes());
    hasher.update(vtable_layout_digest().as_bytes());
    hasher.update(wasm_aot_tier_digest().as_bytes());
    hasher.update(
        b"native-llvm-v5:tier2ABC+MakeIterator+IteratorNext+DictGet+DictSet+Actor+Task+Try+Simd",
    );
    hasher.update(b"aot-fixtures-v1:hot_loop,hello,comparable_sort,inheritance_dispatch,protocol_iterator,main");
    hasher.update(b"sorted-v1:ArrSorted+CallWitness+tier2A=ObjectGet,ArraySet,ValueEq,RefEq");
    hasher.update(b"tier2b-jit-llvm-v1:MakeClosure,CallClosure,AwaitPoll,AwaitSuspend+cranelift-jit-module-v1+llvm-tier2-emit-v1");
    hasher.update(b"tier2c-simd-browser-v1:Actor,Task,Try,SimdGuest+browser-prelude-v2");
    hasher.update(b"iterator-witness-v2:MutRefCell-wrap-order+native-llvm-tier2c-v1:MakeIterator,IteratorNext,DictGet,Actor,Task,Try,Simd");
    hasher.update(b"gc-arc-layout-v1:ObjectRef-retain-release+vm-mark-sweep+browser-prelude-v3");
    hasher.update(b"unified-heap-v2:ManagedHeap+collect_global+vm-align");
    hasher.update(b"native-exec-op-v1:ObjectRefRetain,Release,HeapCollect-llvm-direct");
    hasher.update(b"heap-v3-mir-run-slots:ManagedHeap+heap_collect+actor-roots");
    hasher.update(b"llvm-tier3-zero-exec-op-v1:tier2A-D-direct-rt-no-exec-op-ir");
    hasher.update(b"heap-v4-compact-relocate-v1:ManagedHeap+HeapSlot+compact_global");
    hasher.update(b"inkwell-prod-ir-builder-v1:phase75-default-inkwell-text-opt-out");
    hasher.update(b"vm-heap-v5-slot-parity:ArrayBuild+HeapCompact+HeapSlot");
    hasher.update(b"inkwell-crate-v2:phase76-ModuleBuilder-tier1-2A");
    hasher.update(b"cache-remote-push-v1:SYX_CACHE_REMOTE_PUSH+multi-node");
    hasher.update(b"inkwell-llvm-v3-real-module:tier-2B-D+HeapCompact");
    hasher.update(b"heap-v6-host-materialize:ArraySet+Dict+simd-store");
    hasher.update(b"rt-zero-exec-v2:full-tier-direct-stub");
    hasher.update(b"lock-v11-plugin-cdn:unload-epoch+marketplace+farm-queue");
    hasher.update(b"inkwell-module-v4-real-object:tier-2B-D+object-emit");
    hasher.update(b"heap-v7-relocating-gc:extern-ref-rewrite+HeapRelocate");
    hasher.update(b"rt-zero-mir-v3-runtime:native-run-exec-count-zero");
    hasher.update(b"lock-v12-marketplace-enforce:epoch+rotation+farm-worker");
    hasher.update(b"heap-v8-dict-string-stdout:DictGet-Set-four-end+string-inline");
    hasher.update(b"inkwell-object-aot-v5-real:non-stub-ELF+heap-compact-call");
    hasher.update(b"rt-dispatch-table-v4:RT_DISPATCH_TABLE-hot-path");
    hasher.update(b"lock-v13-tenant-farm:rotation-cron+worker-pool+lease-ttl");
    hasher.update(b"actor-ipc-v1:heap-roots+dist-actor-process");
    hasher.update(b"heap-v9-generational-gc:young-old+write-barrier+string-full-inline");
    hasher.update(b"inkwell-llvm-v6-real-object:write_to_memory_buffer-non-stub");
    hasher.update(b"rt-native-stack-v5:NATIVE_STACK_DISPATCH-zero-mir-exec-hot");
    hasher.update(b"lock-v14-hsm-cdn-billing:hsm_key_ref+global_cdn_sla_tier+billing_meter_id");
    hasher.update(b"dist-actor-cluster-v2:multi-region+cross-process-mailbox");
    hasher.update(b"browser-tier2d-v1:closure+async-hard");
    hasher.update(b"heap-v10-bridge:promotion+card-table+heap_root_bridge+string-offheap-mvp");
    hasher.update(b"inkwell-llvm-v7-link:write_object+cc-link-smoke");
    hasher.update(b"rt-native-stack-v6-full:NATIVE_STACK_DISPATCH_V6-zero-mir-exec-release");
    hasher.update(b"lock-v15-hsm-auto:hsm_rotation_cron+billing_settlement+marketplace_epoch");
    hasher.update(b"dist-actor-cdn-v3:global_cdn+failover+health_probe");
    hasher.update(b"browser-tier2e-v1:SIMD110+actor-hard");
    hasher.update(b"heap-v11-bridge-prod:shared-arena+slot-remap+spawn-hard");
    hasher.update(b"heapslot-writeback-v1:materialize_receiver+mir-run-write-back");
    hasher.update(b"rt-native-stack-v7-real:NATIVE_STACK_DISPATCH_V7-zero-internal-exec");
    hasher.update(b"string-offheap-full:utf8-simd+64b-threshold");
    hasher.update(b"browser-tier2f-v1:full-main-guest+heap-compact");
    hasher.update(b"dist-failover-v4:cross-continent-auto-failover");
    hasher.update(b"lock-v16-marketplace:publish_epoch+extension_signing_key");
    hasher.update(b"mono-witness-v2:disk-cache-mir-backfill");
    hasher.update(b"stdlib-v1-host-table-extension:phase83-45-hosts");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"stdlib-parity-v1:phase84-four-end-host-matrix");
    hasher.update(b"grapheme-cluster-v1:unicode-segmentation-index");
    hasher.update(b"reflect-v2-metadata:no-fallback-witness");
    hasher.update(b"browser-tier2g-v1:stdlib-prelude-hard-run");
    hasher.update(b"borrow-prover-exec-v1:coq-lean-subprocess");
    hasher.update(b"dlopen-default-v1:lockfile-auto-load");
    hasher.update(b"test-runner-v1:tap-filter-aggregate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"stdlib-native-parity-v1:phase85-four-end-host-matrix");
    hasher.update(b"hof-closure-v1:sortBy-flatMap-execute-closure");
    hasher.update(b"reflect-lang-v1:reflect-self-type-metadata");
    hasher.update(b"net-v2-h2-compress:http2-gzip-zstd-accept-encoding");
    hasher.update(b"browser-tier2h-v1:playwright-stdlib-prelude-hard");
    hasher.update(b"dist-winpipe-rustls-v1:winapi-named-pipe-rustls-e2e");
    hasher.update(b"borrow-replay-v1:coq-lean-proof-artifact-mandatory");
    hasher.update(b"test-runner-v2:registry-invoke-tap-aggregate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"p85-honest-closure-v1:sortBy-SYX-comparator-four-end");
    hasher.update(b"reflect-runtime-v1:reflect-self-dynamic-dispatch");
    hasher.update(b"net-v3-h2-zstd:http2-loopback-accept-encoding");
    hasher.update(b"browser-tier2i-v1:playwright-wasm-stdout-mandatory");
    hasher.update(b"dist-prod-e2e-v1:winpipe-rustls-cross-process");
    hasher.update(b"borrow-proof-mandatory-v1:coq-lean-compile-exit0");
    hasher.update(b"test-runner-v3:registry-invoke-tap-exit");
    hasher.update(b"codable-l3-v1:json-encode-decode-synthesis");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-llvm-phi-v1:cfg-ssa-phi-emit");
    hasher.update(b"native-async-resume-v1:await-suspend-driver-loop");
    hasher.update(b"llvm-stub-elimination-v1:real-wire-tier-reduce");
    hasher.update(b"reflect-runtime-cast-v1:as-downcast-dynamic-dispatch");
    hasher.update(b"codable-l4-v1:keyed-container-synthesis");
    hasher.update(b"hof-generic-witness-v1:sortBy-flatMap-generic");
    hasher.update(b"net-grpc-v1:unary-loopback");
    hasher.update(b"net-h2-alpn-default-v1:http2-default-on");
    hasher.update(b"browser-tier2j-v1:playwright-rust-gate-mandatory");
    hasher.update(b"dist-prod-hard-v1:cross-process-e2e");
    hasher.update(b"borrow-mandatory-v2:coq-lean-compile-exit0");
    hasher.update(b"plugin-marketplace-hard-v1:dlopen-poison-reinstall");
    hasher.update(b"file-watch-os-default-v1:inotify-kqueue-winapi");
    hasher.update(b"select-async-v1:select-await-interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-llvm-stack-v1:callhost-stack-sync-main");
    hasher.update(b"llvm-stub-ga-v1:noop-stub-tier-reduce");
    hasher.update(b"reflect-cast-ga-v1:is-as-existential-four-end");
    hasher.update(b"hof-mir-wasm-parity-v1:closure-cell-generic-witness");
    hasher.update(b"kvo-full-chain-v1:observe-willSet-didSet-didChange");
    hasher.update(b"codable-l5-proto-v1:reflect-protobuf-wire");
    hasher.update(b"net-grpc-stream-v1:client-server-streaming-loopback");
    hasher.update(b"net-tls-wss-hard-v1:rustls-persistent-wss-frame");
    hasher.update(b"browser-tier2k-v1:playwright-rust-mandatory");
    hasher.update(b"dist-prod-ga-v1:winpipe-unix-tcp-tls-cross-process");
    hasher.update(b"borrow-mandatory-v3:coq-lean-compile-exit0");
    hasher.update(b"select-async-ga-v1:select-await-interleave");
    hasher.update(b"sql-mvp-v1:sqlite-loopback-query");
    hasher.update(b"xml-mvp-v1:pull-parser-dom-subset");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gc-relocating-v12-v1:heap-slot-rewrite-native-roots");
    hasher.update(b"native-llvm-full-ga-v1:sync-main-zero-mir-entry");
    hasher.update(b"hof-wasm-tier2-closure-v1:call-closure-guest-emit");
    hasher.update(b"reflect-cast-quad-v1:is-as-existential-twelve-tests");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    hasher.update(b"orm-mvp-v1:reflect-table-roundtrip");
    hasher.update(b"xml-xpath-v1:subset-three-expressions");
    hasher.update(b"codable-l6-schema-v1:wire-schema-registry");
    hasher.update(b"farm-multitenant-prod-v1:lease-ttl-three-node-hit");
    hasher.update(b"plugin-cdn-prod-v1:cdn-route-poison-reinstall");
    hasher.update(b"gui-mvp-v1:window-eventloop-canvas");
    hasher.update(b"browser-tier2l-v1:playwright-gui-wasm-smoke");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-aot-tier3-v1:zero-exec-op-direct-emit");
    hasher.update(b"native-llvm-direct-v1:sync-main-direct-call");
    hasher.update(b"reflect-cast-hard-v1:twelve-tests-four-end");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness");
    hasher.update(b"gui-winit-v1:window-event-canvas-headless");
    hasher.update(b"browser-tier2m-v1:playwright-gui-interaction");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"codable-l7-evolution-v1:schema-field-evolution");
    hasher.update(b"farm-lease-enforce-v1:ttl-reject-tcp-tenant");
    hasher.update(b"gc-generational-v13-v1:young-old-compact");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-aot-zero-mir-exec-v1:sync-zero-mir-exec-histogram");
    hasher.update(b"native-llvm-user-fn-v1:multi-fn-zero-syx-rt-call-fn");
    hasher.update(b"reflect-cast-quad-v1:twelve-scenarios-four-end-stdout");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness-mandatory");
    hasher.update(b"gui-winit-real-v1:softbuffer-pixel-headless");
    hasher.update(b"browser-tier2n-v1:playwright-gui-interaction");
    hasher.update(b"codable-l8-migration-v1:field-add-remove-rename");
    hasher.update(b"orm-reflect-where-v1:reflect-compound-and");
    hasher.update(b"xml-xpath-full-v1:eight-expressions-hard");
    hasher.update(b"farm-tcp-cache-v1:fetch-cache-remote-tcp-tenant");
    hasher.update(b"farm-billing-enforce-v1:settlement-excess-exit-nonzero");
    hasher.update(b"gc-generational-v13-real-v1:young-old-promote-compact");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-aot-zero-run-op-v1:sync-zero-run-op-histogram");
    hasher.update(b"native-llvm-async-v1:await-poll-suspend-direct-emit");
    hasher.update(b"reflect-cast-strict-quad-v1:twelve-scenarios-four-end-mandatory");
    hasher.update(b"hof-witness-quad-v1:generic-sortBy-witness-mandatory");
    hasher.update(b"gui-winit-softbuffer-v1:softbuffer-pixel-readback-headless");
    hasher.update(b"gui-mobile-mvp-v1:touch-event-stub");
    hasher.update(b"browser-tier2o-v1:playwright-gui-wasm-interaction");
    hasher.update(b"codable-l9-protobuf-v1:schema-migration-apply-protobuf");
    hasher.update(b"orm-reflect-compound-v1:reflect-compound-and-parameterized");
    hasher.update(b"xml-xpath-strict-v1:eight-expressions-hard");
    hasher.update(b"farm-tcp-prod-v1:farm-worker-tcp-tenant-three-node");
    hasher.update(b"farm-billing-farm-path-v1:settlement-excess-exit-nonzero");
    hasher.update(b"gc-generational-v13-quad-v1:young-old-promote-compact-stdout");
    hasher.update(b"sql-migrations-v1:prepare-bind-migrate");
    hasher.update(b"orm-query-ga-v1:findWhere-update-delete");
    hasher.update(b"sql-rusqlite-v1:embedded-engine-loopback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-build-profile-v1:clang-rt-link-debug-release");
    hasher.update(b"native-llvm-string-v1:const-concat-tostr-four-end");
    hasher.update(b"gui-winit-real-v2:visible-window-softbuffer-pixel");
    hasher.update(b"wasm-aot-full-opcode-v1:zero-mir-exec-all-fixtures");
    hasher.update(b"reflect-cast-fixture-quad-v1:twelve-scenarios-mandatory");
    hasher.update(b"browser-tier2p-v1:playwright-gui-run-mandatory");
    hasher.update(b"codable-l10-protobuf-v1:real-wire-breaking");
    hasher.update(b"orm-bind-compound-v1:parameterized-values-bound");
    hasher.update(b"gc-churn-quad-v1:churn-relocate-four-end-stdout");
    hasher.update(b"farm-tcp-hard-v1:worker-default-tcp-mutex");
    hasher.update(b"native-async-resume-v1:behavioral-stdout-resume");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-llvm-locals-v1:alloca-load-store-ssa");
    hasher.update(b"native-string-exact-v1:hello-syx-exact-stdout");
    hasher.update(b"gui-winit-real-v3:os-window-softbuffer-eventloop");
    hasher.update(b"wasm-aot-zero-fallback-v1:mir-exec-and-run-op-zero-real-programs");
    hasher.update(b"browser-tier2q-v1:playwright-gui-ci-mandatory");
    hasher.update(b"codable-l11-protobuf-v1:pure-wire-breaking-l10");
    hasher.update(b"orm-rusqlite-bind-v1:compound-dict-executed");
    hasher.update(b"gc-churn-quad-v2:churn-stdout-four-end-mandatory");
    hasher.update(b"farm-tcp-hard-v2:worker-default-mutex-billing");
    hasher.update(b"native-lto-v1:thin-lto-opt-in-release");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-gui-callhost-v1:stack-abi-guidemo-strict-parity");
    hasher.update(b"native-string-exact-v1:hello-syx-exact-stdout");
    hasher.update(b"gui-winit-isolated-v1:syx-gui-dlopen-softbuffer-pixel");
    hasher.update(b"wasm-aot-zero-mir-exec-real-v1:main-guidemo-mir-exec-run-op-zero");
    hasher.update(b"browser-tier2r-v1:playwright-test-gui-ci-mandatory");
    hasher.update(b"codable-l12-prost-v1:schema-registry-breaking-l11");
    hasher.update(b"orm-prepare-bind-v1:rusqlite-row-assert");
    hasher.update(b"gc-churn-host-v1:managed-heap-churn-stdout-quad");
    hasher.update(b"farm-tcp-worker-v1:build-syx-default-hardened-tcp");
    hasher.update(b"native-async-resume-v2:behavioral-stdout-order");
    hasher.update(b"llvm-link-guidemo-v1:clang-rt-link-exit-zero");
    hasher.update(b"native-llvm-locals-v2:rt-load-store-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-dlopen-prod-v1:syx-gui-dylib-xvfb-pixel-guidemo-full");
    hasher.update(b"native-guidemo-full-v1:examples-guidemo-ui-import-strict-parity");
    hasher.update(b"wasm-runtime-zero-v1:main-guidemo-wasm-run-smoke");
    hasher.update(b"browser-tier2s-v1:playwright-tier2s-pixel-click-ci-mandatory");
    hasher.update(b"codable-l13-prost-real-v1:prost-message-breaking-l12");
    hasher.update(b"gc-churn-syx-quad-v1:host-program-four-end-stdout");
    hasher.update(b"farm-tcp-e2e-v1:build-syx-worker-no-env");
    hasher.update(b"native-async-resume-v3:multi-await-stdout-order");
    hasher.update(b"llvm-link-run-guidemo-v1:clang-rt-run-exit-zero");
    hasher.update(b"native-llvm-locals-v3:rt-load-store-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-dlopen-default-v1:dylib-unconditional-fallback-hashmap");
    hasher.update(b"native-guidemo-import-ui-v1:examples-guidemo-ui-strict-parity-15plus");
    hasher.update(b"wasm-cold-zero-v1:concat-tostr-tier1-mir-exec-counter-zero");
    hasher.update(b"browser-tier2t-v1:playwright-multistep-gui-ci-mandatory");
    hasher.update(b"codable-l14-evolution-v1:prost-schema-breaking-l13");
    hasher.update(b"gc-churn-stress-v1:multi-churn-stdout-quad");
    hasher.update(b"farm-tcp-multinode-v1:build-syx-3-node-no-env");
    hasher.update(b"native-async-resume-v4:nested-await-stdout-order");
    hasher.update(b"llvm-multi-link-run-v1:guidemo-main-fixtures-clang-rt-exit-zero");
    hasher.update(b"native-llvm-locals-v4:alloca-ssa-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-call-return-stack-v1:package-ui-call-mir-stack-abi");
    hasher.update(b"native-guidemo-import-ui-v2:examples-guidemo-15-line-nil-free");
    hasher.update(b"native-llvm-locals-v5:alloca-ssa-value-ga");
    hasher.update(b"wasm-guidemo-zero-v1:main-guidemo-mir-exec-run-op-zero");
    hasher.update(b"browser-tier2u-v1:playwright-guidemo-subset-hard-ci");
    hasher.update(b"codable-l14-surface-v1:hostfn-packages-l14-names");
    hasher.update(b"farm-tcp-3node-v2:build-syx-tcp-listeners-no-env");
    hasher.update(b"llvm-line-parity-v1:guidemo-main-stdout-line-equal");
    hasher.update(b"llvm-utf8-cstr-v1:byte-escape-not-codepoint");
    hasher.update(b"gui-visible-hard-v1:pixel-and-window-count-nonzero");
    hasher.update(b"driver-gate-behavioral-v1:phase98-120-plus-40-compile-run");
    hasher.update(b"mir-licm-stack-order-v1:no-const-hoist-call-args");
    hasher.update(b"llvm-gc-preamble-main-only-v1:nested-call-stack-preserve");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"native-llvm-locals-v6:alloca-value-single-source-ga");
    hasher.update(b"wasm-hard-zero-v1:no-catchall-no-rename-main-guidemo");
    hasher.update(b"gui-line-hard-v1:guidemo-main-stdout-equal");
    hasher.update(b"gui-visible-nonzero-v1:window-and-pixel");
    hasher.update(b"browser-tier2u-hard-v2:no-ci-fallback-no-spec-skip");
    hasher.update(b"farm-tcp-queue-v1:build-syx-3tcp-farm-and-queue");
    hasher.update(b"codable-l14-only-v1:no-l9-surface");
    hasher.update(b"audit-align-v1:ci-script-adr-1250-1850");
    hasher.update(b"lsp-semantic-v1:phase99-12-plus-real");
    hasher.update(b"driver-gate-cr50-v1:phase99-120-plus-50-compile-run");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-widgets-bitmap-v1:label-button-visible");
    hasher.update(b"gui-visible-dylib-hard-v1:no-silent-fallback");
    hasher.update(b"gui-cli-ship-dylib-v1:target-dir-copy");
    hasher.update(b"native-llvm-locals-v7:no-hydrate-handle-ga");
    hasher.update(b"wasm-guest-control-v1:no-mir-exec-jump-conststr");
    hasher.update(b"farm-tcp-frames-v1:get-put-hit");
    hasher.update(b"codable-l14-host-prune-v1:no-l5-l13-public");
    hasher.update(b"audit-align-v1:ci-script-adr-1400-1900");
    hasher.update(b"lsp-semantic-v2:phase100-16-plus-real");
    hasher.update(b"driver-gate-cr60-v1:phase100-compile-run");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-interactive-v1:hit-onclick-layout");
    hasher.update(b"gui-font-ascii-full-v1:printable-20-7e");
    hasher.update(b"wasm-concat-exact-v1:hello-syx-quad");
    hasher.update(b"wasm-allowlist-thin-v1:closure-return-guest");
    hasher.update(b"native-llvm-locals-v8:frame-slab-no-immortal-ga");
    hasher.update(b"farm-frame-lease-v1:ttl-err-on-expired");
    hasher.update(b"codable-hostfn-prune-v2:no-l5-l13-enum");
    hasher.update(b"lang-deinit-auto-v1:scope-exit-call");
    hasher.update(b"audit-align-v1:ci-script-adr-1550-1950");
    hasher.update(b"lsp-semantic-v3:phase101-18-plus-real");
    hasher.update(b"driver-gate-cr70-v1:phase101-real-behavioral");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-callback-ga-v1:invoke-onclick-textfield");
    hasher.update(b"gui-hstack-mvp-v1:layout-axis");
    hasher.update(b"browser-tier2w-v1:click-invoke-field");
    hasher.update(b"wasm-actor-task-guest-v1:no-mir-exec-actor-task");
    hasher.update(b"wasm-call-thin-v1:fn-host-guest");
    hasher.update(b"lang-resultbuilder-ga-v1:multistmt-nested");
    hasher.update(b"codable-host-body-prune-v1:no-l5-l13-impl");
    hasher.update(b"lang-deinit-arc-v1:refcount-zero-call");
    hasher.update(b"audit-align-v1:ci-script-adr-1700-2000");
    hasher.update(b"lsp-semantic-v4:phase102-20-plus-real");
    hasher.update(b"driver-gate-cr80-v1:phase102-real-behavioral");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-textfield-focus-v1:caret-tab-cycle");
    hasher.update(b"gui-hstack-ga-v1:spacing-align");
    hasher.update(b"gui-onclick-hard-v1:no-swallow-err");
    hasher.update(b"browser-tier2w-ci-v1:playwright-job");
    hasher.update(b"wasm-closure-guest-v1:no-mir-exec-closure-fnindex");
    hasher.update(b"native-llvm-locals-v9:frame-value-ptr-ga");
    hasher.update(b"plugin-dlclose-v1:unload-reload");
    hasher.update(b"lang-resultbuilder-v2:build-optional");
    hasher.update(b"select-await-interleave-v2:case-await-order");
    hasher.update(b"lang-deinit-weak-v1:weak-nil-on-drop");
    hasher.update(b"audit-align-v1:ci-script-adr-1850-2050");
    hasher.update(b"lsp-semantic-v5:phase103-20-plus-real");
    hasher.update(b"driver-gate-cr90-v1:phase103-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:81.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ttf-subset-v1:fontdue-cjk");
    hasher.update(b"gui-ime-preedit-v1:composition-draw");
    hasher.update(b"gui-doctor-coverage-v1:phase105");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd-full-v1:aot-ops-complete");
    hasher.update(b"wasm-no-mir-exec-catchall-v1:phase105");
    hasher.update(b"wasm-allowlist-empty-v1:phase104");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"select-default-stack-v1:sugar-matches-host");
    hasher.update(b"select-multi-case-v1:sugar-matches-host");
    hasher.update(b"reflect-witness-mandatory-v1:no-synthesize-true");
    hasher.update(b"reflect-specialization-ga-v1:miss-is-error");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-failclosed-v1:no-auth-is-err");
    hasher.update(b"native-llvm-locals-v10-e2e:guidemo-line-parity");
    hasher.update(b"plugin-unload-hard-v1:fixture-required");
    hasher.update(b"native-llvm-locals-v10:no-tls-slab");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2150-2150");
    hasher.update(b"audit-exclude-probes-v1:phase105");
    hasher.update(b"lsp-semantic-v7:phase105-24-plus-real");
    hasher.update(b"driver-gate-cr110-v1:phase105-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:83.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ttf-expand-v1:cjk64-plus");
    hasher.update(b"gui-ime-highlight-v1:preedit-invert");
    hasher.update(b"gui-doctor-cap-v1:phase106");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd-lt-shuffle-v1:aot-complete");
    hasher.update(b"wasm-ga-no-mir-exec-env-v1:phase106");
    hasher.update(b"wasm-simd-full-v1:aot-ops-complete");
    hasher.update(b"wasm-no-mir-exec-catchall-v1:phase105");
    hasher.update(b"wasm-allowlist-empty-v1:phase104");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"select-send-case-v1:sugar-matches-host");
    hasher.update(b"select-multi-await-v1:case-await-order");
    hasher.update(b"reflect-semantic-unified-v1:instantiations-first");
    hasher.update(b"select-multi-case-v1:sugar-matches-host");
    hasher.update(b"reflect-specialization-ga-v1:miss-is-error");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(crate::farm_lease_auth::farm_lease_server_digest().as_bytes());
    hasher.update(crate::farm_lease_auth::farm_lease_failclosed_digest().as_bytes());
    hasher.update(crate::farm_lease_auth::farm_lease_auth_digest().as_bytes());
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2300-2200");
    hasher.update(b"audit-exclude-gate0-v1:phase106");
    hasher.update(b"lsp-semantic-v8:phase106-24-plus-real");
    hasher.update(b"driver-gate-cr120-v1:phase106-real-behavioral");
    hasher.update(b"driver-gate-cr130-v1:phase107-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:84.0");
    hasher.update(b"lsp-semantic-v9:phase107-24-plus-real");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-harfbuzz-v1:shape-mvp");
    hasher.update(b"gui-font-fallback-v1:system-cjk");
    hasher.update(b"gui-doctor-real-v1:phase107");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-expand-v1:div-eq-lt-shuffle");
    hasher.update(b"llvm-declare-call-v1:rt-hygiene");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"reflect-witness-honest-v1:no-synthesize");
    hasher.update(b"select-fair-v1:round-robin");
    hasher.update(b"select-mixed-unified-v1:tagged-array-poll");
    // Historical Phase 107 surface tag retained for grep/invalidation continuity.
    hasher.update(b"select-mixed-poll-v1:send-recv-array");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2450-2250");
    hasher.update(b"audit-exclude-fill-v1:phase107");
    hasher.update(b"lsp-semantic-v9:phase107-24-plus-real");
    hasher.update(b"driver-gate-cr130-v1:phase107-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:85.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-egui-mvp-v1:winit-softbuffer");
    hasher.update(b"gui-pump-reentrancy-v1:dirty-defer");
    hasher.update(b"gui-doctor-egui-v1:phase108");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-bitwise-v1:gt-and-or-xor");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"select-mixed-unified-v1:tagged-array-poll");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-ha-v2:process-isolated");
    hasher.update(b"dist-actor-mp-v2:no-inproc-fallback-ga");
    // Phase 109 Track D seeds (cache v81 supersedes; keep present for soft gates).
    hasher.update(b"dist-region-anycast-v1:tcp-health");
    hasher.update(b"select-mixed-await-v1:interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2600-2300");
    hasher.update(b"audit-exclude-arith-v1:phase108");
    hasher.update(b"lsp-semantic-v10:phase108-24-plus-real");
    hasher.update(b"driver-gate-cr140-v1:phase108-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:86.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-egui-wgpu-v1:winit-present");
    hasher.update(b"gui-hb-paint-v1:string-shape");
    hasher.update(b"gui-emoji-v1:color-mvp");
    hasher.update(b"gui-vertical-rl-v1:label-mvp");
    hasher.update(b"gui-doctor-wgpu-v1:phase109");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v2:div-load-store-ext");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-ha-v3:os-process");
    hasher.update(b"farm-lease-raft-v1:quorum-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-region-anycast-v1:tcp-health");
    hasher.update(b"select-mixed-await-v1:interleave");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2750-2350");
    hasher.update(b"audit-exclude-cat-v1:phase109");
    hasher.update(b"lsp-semantic-v11:phase109-24-plus-real");
    hasher.update(b"driver-gate-cr150-v1:phase109-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:87.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-wgpu-composite-v1:hb-emoji-tex");
    hasher.update(b"gui-emoji-v2:cbdt-or-colr");
    hasher.update(b"gui-vertical-rl-v2:layout");
    hasher.update(b"gui-doctor-live-v1:phase110");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v3:spill-thin-shr");
    hasher.update(b"wasm-simd128-div-assembled-v1:honest");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-raft-v2:log-wal");
    hasher.update(b"farm-lease-member-v1:dynamic-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-region-multihost-v1:keyed-failover");
    hasher.update(b"select-mixed-timeout-v1:cancel-or-deadline");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2900-2400");
    hasher.update(b"audit-exclude-real-v1:phase110");
    hasher.update(b"lsp-semantic-v12:phase110-24-plus-real");
    hasher.update(b"driver-gate-cr160-v1:phase110-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:88.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-glyph-v1:raster-mvp");
    hasher.update(b"gui-emoji-v3:colr-outline");
    hasher.update(b"gui-vertical-hb-v1:ink-on");
    hasher.update(b"gui-doctor-used-v1:phase111");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v4:thin-arith-minmax-f32");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-multihost-v1:config-members");
    hasher.update(b"farm-lease-tenant-v1:isolation-mvp");
    hasher.update(b"farm-lease-wal-crc-v1:reject-corrupt");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-region-nonloopback-v1:dual-bind-failover");
    hasher.update(b"select-timeout-syntax-v1:lang-arm");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3050-2450");
    hasher.update(b"audit-exclude-cat-v1:phase111");
    hasher.update(b"lsp-semantic-v13:phase111-24-plus-real");
    hasher.update(b"driver-gate-cr170-v1:phase111-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:89.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-layout-v1:paragraph-bidi-mvp");
    hasher.update(b"gui-emoji-v4:cbdt-or-colr");
    hasher.update(b"gui-vertical-unify-v1:hb-advance");
    hasher.update(b"gui-doctor-strict-v1:phase112");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v5:f32-ops-guest-ls");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-runtime-v1:config-members");
    hasher.update(b"farm-lease-tenant-v2:token-mvp");
    hasher.update(b"farm-lease-wal-snap-v1:checkpoint-replay");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-region-hostkey-v2:config-nonloopback");
    hasher.update(b"dist-cdn-tcp-v1:peer-mvp");
    hasher.update(b"select-deadline-unify-v1:lang-over-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3200-2500");
    hasher.update(b"audit-exclude-fill-v1:phase112");
    hasher.update(b"lsp-semantic-v14:phase112-24-plus-real");
    hasher.update(b"driver-gate-cr180-v1:phase112-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:90.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-paragraph-v1:textlayout");
    hasher.update(b"gui-emoji-v5:cbdt-ci-or-colr");
    hasher.update(b"gui-doctor-engine-v1:phase113");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v6:f32div-guest-ls-f64");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-tenant-v3:token-default");
    hasher.update(b"farm-lease-wal-compact-v1:snap-truncate-log");
    hasher.update(b"farm-lease-members-v2:dyn-list");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-tcp-v2:peer-delivery");
    hasher.update(b"dist-region-hostkey-v3:runtime-bind");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3350-2550");
    hasher.update(b"audit-exclude-cat-v1:phase113");
    hasher.update(b"lsp-semantic-v15:phase113-24-plus-real");
    hasher.update(b"driver-gate-cr190-v1:phase113-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:91.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-paragraph-v2:paint");
    hasher.update(b"gui-emoji-v6:cbdt-or-native-colr-grad");
    hasher.update(b"gui-font-system-v1:mgr-fallback");
    hasher.update(b"gui-doctor-engine-v2:phase114");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v7:f64reduce-guest-ls-f64sub");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-multidc-v1:discover-failover");
    hasher.update(b"farm-lease-tenant-v4:token-all-paths-multidc");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-pop-v1:health-anycast");
    hasher.update(b"dist-region-hostkey-v4:pop-bind-noloop");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3500-2600");
    hasher.update(b"audit-exclude-fill-v1:phase114");
    hasher.update(b"lsp-semantic-v16:phase114-24-plus-real");
    hasher.update(b"driver-gate-cr200-v1:phase114-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:92.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-paragraph-v3:single-path");
    hasher.update(b"gui-font-system-v2:proven-fallback");
    hasher.update(b"gui-emoji-v7:colr-radial-or-sweep");
    hasher.update(b"gui-doctor-engine-v3:phase115");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v8:nospill-binop-f64muldiv-reduce");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-multidc-v2:runtime-wired");
    hasher.update(b"farm-lease-registry-v1:reload-probe");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-pop-v2:noloop-e2e");
    hasher.update(b"dist-region-hostkey-v5:bind-eq-key");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3650-2650");
    hasher.update(b"audit-exclude-fill-v1:phase115");
    hasher.update(b"lsp-semantic-v17:phase115-24-plus-real");
    hasher.update(b"driver-gate-cr210-v1:phase115-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:93.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-paragraph-v4:frame-single-ink");
    hasher.update(b"gui-font-system-v3:os-proven");
    hasher.update(b"gui-emoji-v8:colr-radial-and-sweep");
    hasher.update(b"gui-doctor-engine-v4:phase116");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v9:nospill-unary-shift-shuffle-f32ls");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-multidc-v3:cross-host");
    hasher.update(b"farm-lease-registry-v2:health-sync");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-pop-v3:dual-process-host");
    hasher.update(b"dist-region-hostkey-v6:concrete-ip");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3800-2700");
    hasher.update(b"audit-exclude-fill-v1:phase116");
    hasher.update(b"lsp-semantic-v18:phase116-24-plus-real");
    hasher.update(b"driver-gate-cr220-v1:phase116-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:94.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-skia-paragraph-v5:all-widgets-ink");
    hasher.update(b"gui-font-system-v4:os-only-ga");
    hasher.update(b"gui-emoji-v9:font-radial-and-sweep");
    hasher.update(b"gui-ot-features-v1:kern-liga-calt");
    hasher.update(b"gui-doctor-engine-v5:phase117");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd128-guest-v10:stack-native-simd8-i64");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-multidc-v4:real-connect");
    hasher.update(b"farm-lease-registry-v3:consensus-sync");
    hasher.update(b"farm-lease-token-v1:rotate");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-bgp-v1:anycast-mvp");
    hasher.update(b"dist-cdn-pop-v4:remote-hostkey");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-3950-2750");
    hasher.update(b"audit-exclude-fill-v1:phase117");
    hasher.update(b"lsp-semantic-v19:phase117-24-plus-real");
    hasher.update(b"driver-gate-cr230-v1:phase117-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:95.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-freestanding-v1:role-enforce");
    hasher.update(b"macro-external-v2:plugin-prod");
    hasher.update(b"macro-codable-l2-v1:enum-generic");
    hasher.update(b"macro-attached-conf-v1:no-silent-drop");
    hasher.update(b"macro-pipeline-v2:resultbuilder-in-engine");
    hasher.update(b"macro-builtin-v2:stringify-compiletime");
    hasher.update(b"macro-package-v1:exports-e2e");
    hasher.update(b"gui-ot-features-v3:script-subset");
    hasher.update(b"gui-font-vf-v2:multi-axis-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-registry-v5:multi-follower-sync");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"dist-cdn-pop-v6:multi-hostkey-route");
    hasher.update(b"select-deadline-strict-v1:no-env");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-docs-v1:m9-honesty-pass");
    hasher.update(b"audit-align-v1:ci-script-adr-4250-2850");
    hasher.update(b"audit-exclude-fill-v1:phase119");
    hasher.update(b"lsp-semantic-v21:phase119-24-plus-real");
    hasher.update(b"driver-gate-cr250-v1:phase119-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:97.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-package-v2:exports-schema");
    hasher.update(b"macro-service-v1:attr-scan");
    hasher.update(b"macro-service-v1:registry-codegen");
    hasher.update(b"macro-service-v1:ctor-inject");
    hasher.update(b"runtime-service-v1:singleton-container");
    hasher.update(b"macro-docs-v2:m10-service-honesty");
    hasher.update(b"macro-pipeline-v3:rb-nested-hostcall");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ot-features-v5:paint-matrix-mvp");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v3:hsm-ref-mvp");
    hasher.update(b"dist-cdn-bgp-v3:ops-visibility");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-4400-2900");
    hasher.update(b"audit-exclude-fill-v1:phase120");
    hasher.update(b"lsp-semantic-v22:phase120-24-plus-real");
    hasher.update(b"driver-gate-cr260-v1:phase120-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:98.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v2:typed-resolve");
    hasher.update(b"macro-service-v2:field-autowired");
    hasher.update(b"macro-service-v2:import-visible-scan");
    hasher.update(b"runtime-service-v2:real-ctor-resolve");
    hasher.update(b"runtime-service-v2:postconstruct-mvp");
    hasher.update(b"macro-pipeline-v4:rb-nested-fixed-or-reject");
    hasher.update(b"macro-docs-v3:m11-service-v2-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ot-features-v5:paint-matrix-mvp");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v4:kms-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v4:route-convergence-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-4550-2950");
    hasher.update(b"audit-exclude-fill-v1:phase121");
    hasher.update(b"lsp-semantic-v23:phase121-24-plus-real");
    hasher.update(b"driver-gate-cr270-v1:phase121-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:99.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v3:multi-package-scan");
    hasher.update(b"macro-service-v3:typed-resolve-harden");
    hasher.update(b"runtime-service-v3:prototype-scope");
    hasher.update(b"runtime-service-v3:predestroy-mvp");
    hasher.update(b"macro-pipeline-v5:rb-nested-stack-fixed");
    hasher.update(b"macro-docs-v4:m12-service-v3-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ot-features-v6:default-paint-ot-mvp");
    hasher.update(b"gui-dylib-parity-v1:ot-vf-export");
    hasher.update(b"gui-font-vf-v4:paint-variations-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v5:pkcs11-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v5:multi-region-mesh-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-4700-3000");
    hasher.update(b"audit-exclude-fill-v1:phase122");
    hasher.update(b"lsp-semantic-v24:phase122-24-plus-real");
    hasher.update(b"driver-gate-cr280-v1:phase122-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:100.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"runtime-service-v4:request-scope");
    hasher.update(b"macro-service-v4:ceremony-sugar");
    hasher.update(b"macro-pipeline-v6:rb-hostcall-closure-fixed");
    hasher.update(b"macro-service-v4:typed-resolve-ident");
    hasher.update(b"macro-docs-v5:m13-service-v4-honesty");
    hasher.update(b"macro-service-v4:scan-resolved-only");
    hasher.update(b"macro-service-v3:multi-package-scan");
    hasher.update(b"runtime-service-v3:prototype-scope");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-ot-features-v7:os-fontmgr-default-paint");
    hasher.update(b"gui-ot-features-v6:default-paint-ot-mvp");
    hasher.update(b"gui-dylib-parity-v1:ot-vf-export");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    hasher.update(b"wasm-simd16-guest-v4:shift-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v3:logic-or-shift");
    hasher.update(b"wasm-simd16-guest-v2:matrix-expand");
    hasher.update(b"wasm-simd128-guest-v11:zero-scratch-simd16");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v6:pkcs11-so-adapter");
    hasher.update(b"dist-cdn-bgp-v6:multi-pop-mvp");
    hasher.update(b"farm-lease-token-v5:pkcs11-bridge-mvp");
    hasher.update(b"dist-cdn-bgp-v5:multi-region-mesh-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-4850-3050");
    hasher.update(b"audit-exclude-fill-v1:phase123");
    hasher.update(b"lsp-semantic-v25:phase123-26-plus-real");
    hasher.update(b"driver-gate-cr290-v1:phase123-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:101.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v5:di-package-ga");
    hasher.update(b"runtime-service-v5:with-request");
    hasher.update(b"macro-service-v5:conditional-qualifier");
    hasher.update(b"macro-service-v5:conditional-profile");
    hasher.update(b"macro-docs-v6:m14-service-v5-honesty");
    hasher.update(b"runtime-service-v4:request-scope");
    hasher.update(b"macro-service-v4:ceremony-sugar");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v1:path-dep-desktop-mvp");
    hasher.update(b"gui-synui-v1:ui-package-surface");
    hasher.update(b"gui-synui-v1:host-bridge");
    hasher.update(b"gui-synui-v2:desktop-run-mvp");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    hasher.update(b"wasm-simd16-guest-v5:logic-or-xor");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v7:pkcs11-session-mvp");
    hasher.update(b"dist-cdn-bgp-v7:weighted-anycast-mvp");
    hasher.update(b"farm-lease-token-v6:pkcs11-so-adapter");
    hasher.update(b"dist-cdn-bgp-v6:multi-pop-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5000-3100");
    hasher.update(b"audit-exclude-fill-v1:phase124");
    hasher.update(b"lsp-semantic-v26:phase124-28-plus-real");
    hasher.update(b"driver-gate-cr300-v1:phase124-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:102.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v6:polymorphic-base-inject");
    hasher.update(b"macro-service-v6:field-qualifier-metadata");
    hasher.update(b"runtime-service-v6:protocol-binding");
    hasher.update(b"macro-m15:service-method-interceptor-mvp");
    hasher.update(b"macro-docs-v7:m15-service-v6-honesty");
    hasher.update(b"macro-service-v5:di-package-ga");
    hasher.update(b"runtime-service-v5:with-request");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v2:desktop-run-mvp");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    hasher.update(b"gui-synui-v1:host-bridge");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    hasher.update(b"wasm-simd16-guest-v6:logic-not");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v8:session-pin-persist");
    hasher.update(b"dist-cdn-bgp-v8:sticky-pop-mvp");
    hasher.update(b"farm-lease-token-v7:pkcs11-session-mvp");
    hasher.update(b"dist-cdn-bgp-v7:weighted-anycast-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5150-3150");
    hasher.update(b"audit-exclude-fill-v1:phase125");
    hasher.update(b"lsp-semantic-v27:phase125-30-plus-real");
    hasher.update(b"driver-gate-cr310-v1:phase125-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:103.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"runtime-service-v7:protocol-witness");
    hasher.update(b"macro-service-v7:advice-chain");
    hasher.update(b"macro-docs-v8:m16-service-v7-honesty");
    hasher.update(b"macro-service-v6:polymorphic-base-inject");
    hasher.update(b"runtime-service-v6:protocol-binding");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v3:desktop-run-optin");
    hasher.update(b"gui-synui-v3:widget-checkbox-slider");
    hasher.update(b"gui-synui-v2:signal-callback");
    hasher.update(b"gui-synui-v2:widget-expand");
    hasher.update(b"gui-synui-v2:ci-sibling-checkout");
    hasher.update(b"gui-synui-v1:host-bridge");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    hasher.update(b"wasm-simd16-guest-v7:lane-min-max");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v9:pkcs11-sign-mvp");
    hasher.update(b"farm-lease-token-v8:session-pin-persist");
    hasher.update(b"dist-cdn-bgp-v9:sticky-ttl-mvp");
    hasher.update(b"dist-cdn-bgp-v8:sticky-pop-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5300-3200");
    hasher.update(b"audit-exclude-fill-v1:phase126");
    hasher.update(b"lsp-semantic-v28:phase126-32-plus-real");
    hasher.update(b"driver-gate-cr320-v1:phase126-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:104.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v8:advice-proceed");
    hasher.update(b"macro-service-v8:configuration-bean-mvp");
    hasher.update(b"runtime-service-v8:configuration-bean-mvp");
    hasher.update(b"macro-docs-v9:m17-service-v8-honesty");
    hasher.update(b"macro-service-v7:advice-chain");
    hasher.update(b"runtime-service-v7:protocol-witness");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v4:signal-widget-bind");
    hasher.update(b"gui-synui-v4:desktop-onclick-wire");
    hasher.update(b"gui-synui-v4:widget-switch-radio");
    hasher.update(b"gui-synui-v3:desktop-run-optin");
    hasher.update(b"gui-synui-v3:widget-checkbox-slider");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v10:session-close-mvp");
    hasher.update(b"dist-cdn-bgp-v10:sticky-persist-mvp");
    hasher.update(b"farm-lease-token-v9:pkcs11-sign-mvp");
    hasher.update(b"dist-cdn-bgp-v9:sticky-ttl-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5450-3250");
    hasher.update(b"audit-exclude-fill-v1:phase127");
    hasher.update(b"lsp-semantic-v29:phase127-34-plus-real");
    hasher.update(b"driver-gate-cr330-v1:phase127-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:105.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"runtime-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"macro-docs-v10:m18-service-v9-honesty");
    hasher.update(b"macro-service-v8:advice-proceed");
    hasher.update(b"macro-service-v8:configuration-bean-mvp");
    hasher.update(b"runtime-service-v8:configuration-bean-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v5:toggle-radio-bind");
    hasher.update(b"gui-synui-v5:widget-progress-picker");
    hasher.update(b"gui-synui-v5:mobile-surface-unretire");
    hasher.update(b"gui-synui-v4:signal-widget-bind");
    hasher.update(b"gui-synui-v4:desktop-onclick-wire");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v11:hsm-ref-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v11:sticky-multi-snapshot-mvp");
    hasher.update(b"farm-lease-token-v10:session-close-mvp");
    hasher.update(b"dist-cdn-bgp-v10:sticky-persist-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5600-3300");
    hasher.update(b"audit-exclude-fill-v1:phase128");
    hasher.update(b"lsp-semantic-v30:phase128-36-plus-real");
    hasher.update(b"driver-gate-cr340-v1:phase128-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:106.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-codable-l2:associated-enum-auto");
    hasher.update(b"macro-service-v10:bean-param-autowire-mvp");
    hasher.update(b"macro-docs-v11:m19-service-v10-honesty");
    hasher.update(b"macro-service-v9:lazy-cycle-proxy-mvp");
    hasher.update(b"runtime-service-v9:lazy-cycle-proxy-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v6:widget-textarea-list");
    hasher.update(b"gui-synui-v6:progress-number-bind");
    hasher.update(b"gui-synui-v6:desktop-run-layout-ga");
    hasher.update(b"gui-synui-v5:toggle-radio-bind");
    hasher.update(b"gui-synui-v5:widget-progress-picker");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    hasher.update(b"wasm-simd16-guest-v8:lane-extract-replace");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v12:hsm-sign-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v12:peer-announce-sticky-mvp");
    hasher.update(b"farm-lease-token-v11:hsm-ref-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v11:sticky-multi-snapshot-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5750-3350");
    hasher.update(b"audit-exclude-fill-v1:phase129");
    hasher.update(b"lsp-semantic-v31:phase129-38-plus-real");
    hasher.update(b"driver-gate-cr350-v1:phase129-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:107.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    hasher.update(b"macro-codable-l2:associated-enum-auto");
    hasher.update(b"macro-service-v10:bean-param-autowire-mvp");
    hasher.update(b"macro-docs-v11:m19-service-v10-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    hasher.update(b"gui-synui-v6:widget-textarea-list");
    hasher.update(b"gui-synui-v6:progress-number-bind");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    hasher.update(b"wasm-simd16-guest-v9:compare-lane-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    hasher.update(b"farm-lease-token-v12:hsm-sign-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v12:peer-announce-sticky-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-5900-3400");
    hasher.update(b"audit-exclude-fill-v1:phase130");
    hasher.update(b"lsp-semantic-v32:phase130-40-plus-real");
    hasher.update(b"driver-gate-cr360-v1:phase130-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:108.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v3:decl-hygiene-mvp");
    hasher.update(b"macro-service-v13:before-after-pointcut-bind-mvp");
    hasher.update(b"macro-docs-v14:m22-service-v13-honesty");
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v9:image-signal-bind");
    hasher.update(b"gui-synui-v9:widget-divider");
    hasher.update(b"gui-synui-v9:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v15:hsm-digest-streaming-mvp");
    hasher.update(b"dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp");
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-6200-3500");
    hasher.update(b"audit-exclude-fill-v1:phase132");
    hasher.update(b"lsp-semantic-v34:phase132-44-plus-real");
    hasher.update(b"driver-gate-cr380-v1:phase132-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:110.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-6050-3450");
    hasher.update(b"audit-exclude-fill-v1:phase131");
    hasher.update(b"lsp-semantic-v33:phase131-42-plus-real");
    hasher.update(b"driver-gate-cr370-v1:phase131-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:109.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v4:stmt-token-subst-mvp");
    hasher.update(b"macro-service-v14:pointcut-within-name-mvp");
    hasher.update(b"macro-token-v5:expr-token-subst-mvp");
    hasher.update(b"macro-service-v15:pointcut-args-name-mvp");
    hasher.update(b"macro-docs-v16:m24-service-v15-honesty");
    hasher.update(b"macro-docs-v15:m23-service-v14-honesty");
    hasher.update(b"macro-token-v3:decl-hygiene-mvp");
    hasher.update(b"macro-service-v13:before-after-pointcut-bind-mvp");
    hasher.update(b"macro-docs-v14:m22-service-v13-honesty");
    hasher.update(b"macro-token-v2:decl-param-subst-mvp");
    hasher.update(b"macro-service-v12:pointcut-execution-name-mvp");
    hasher.update(b"macro-docs-v13:m21-service-v12-honesty");
    hasher.update(b"macro-token-v1:expr-token-mvp");
    hasher.update(b"macro-service-v11:pointcut-name-mvp");
    hasher.update(b"macro-docs-v12:m20-service-v11-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v10:widget-stack");
    hasher.update(b"gui-synui-v10:scrollview-signal-bind");
    hasher.update(b"gui-synui-v10:web-mobile-layout-deepen");
    hasher.update(b"gui-synui-v9:image-signal-bind");
    hasher.update(b"gui-synui-v9:widget-divider");
    hasher.update(b"gui-synui-v9:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v8:textarea-list-bind");
    hasher.update(b"gui-synui-v8:widget-spacer");
    hasher.update(b"gui-synui-v8:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v7:widget-image-scroll");
    hasher.update(b"gui-synui-v7:picker-signal-bind");
    hasher.update(b"gui-synui-v7:web-language-surface-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v16:hsm-multi-slot-mvp");
    hasher.update(b"dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp");
    hasher.update(b"farm-lease-token-v15:hsm-digest-streaming-mvp");
    hasher.update(b"dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp");
    hasher.update(b"farm-lease-token-v14:hsm-digest-bound-mvp");
    hasher.update(b"dist-cdn-bgp-v14:peer-announce-fanout-mvp");
    hasher.update(b"farm-lease-token-v13:hsm-sign-verify-mvp");
    hasher.update(b"dist-cdn-bgp-v13:peer-announce-ack-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"stdlib-os-v1:cpu-mem-user-mvp");
    hasher.update(b"runtime-concurrency-v1:task-worker-config-mvp");
    hasher.update(b"runtime-concurrency-v1:actor-limit-mailbox-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-6350-3550");
    hasher.update(b"audit-exclude-fill-v1:phase133");
    hasher.update(b"lsp-semantic-v35:phase133-46-plus-real");
    hasher.update(b"driver-gate-cr390-v1:phase133-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:111.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v2:docs-honesty");
    hasher.update(b"lang-props-v2:inherited-observers");
    hasher.update(b"lang-props-v2:init-suppress-observers");
    hasher.update(b"lang-props-v2:computed-get-only-mvp");
    hasher.update(b"lang-props-v2:static-var-mvp");
    hasher.update(b"lang-props-v2:lazy-var-mvp");
    hasher.update(b"lang-props-v3:super-init-field-readback");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v11:widget-tabview");
    hasher.update(b"gui-synui-v11:tab-signal-bind");
    hasher.update(b"gui-synui-v11:web-mobile-controls-deepen");
    hasher.update(b"gui-synui-v10:widget-stack");
    hasher.update(b"gui-synui-v10:scrollview-signal-bind");
    hasher.update(b"gui-synui-v10:web-mobile-layout-deepen");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v17:le-mvp");
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v17:hsm-session-pin-mvp");
    hasher.update(b"dist-cdn-bgp-v17:peer-announce-ttl-mvp");
    hasher.update(b"farm-lease-token-v16:hsm-multi-slot-mvp");
    hasher.update(b"dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v5:expr-token-subst-mvp");
    hasher.update(b"macro-service-v15:pointcut-args-name-mvp");
    hasher.update(b"macro-docs-v16:m24-service-v15-honesty");
    hasher.update(b"runtime-concurrency-v2:bounded-queue-abort-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase135");
    hasher.update(b"lsp-semantic-v37:phase135-50-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase135-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:113.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v3:super-init-field-readback");
    hasher.update(b"lang-props-v3:computed-get-set-mvp");
    hasher.update(b"lang-props-v3:property-override-mvp");
    hasher.update(b"lang-props-v3:docs-honesty");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v12:widget-navigation");
    hasher.update(b"gui-synui-v12:nav-signal-bind");
    hasher.update(b"gui-synui-v12:web-mobile-surface-deepen");
    hasher.update(b"gui-synui-v11:widget-tabview");
    hasher.update(b"gui-synui-v11:tab-signal-bind");
    hasher.update(b"gui-synui-v11:web-mobile-controls-deepen");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v19:avgr-u-mvp");
    hasher.update(b"wasm-simd16-guest-v18:ge-mvp");
    hasher.update(b"wasm-simd16-guest-v17:le-mvp");
    hasher.update(b"wasm-simd16-guest-v16:ne-mvp");
    hasher.update(b"wasm-simd16-guest-v15:neg-mvp");
    hasher.update(b"wasm-simd16-guest-v14:abs-mvp");
    hasher.update(b"wasm-simd16-guest-v13:unsigned-min-max-mvp");
    hasher.update(b"wasm-simd16-guest-v12:load-store-mvp");
    hasher.update(b"wasm-simd16-guest-v11:shuffle-mvp");
    hasher.update(b"wasm-simd16-guest-v10:saturating-add-sub-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v18:hsm-key-label-mvp");
    hasher.update(b"dist-cdn-bgp-v18:peer-announce-priority-mvp");
    hasher.update(b"farm-lease-token-v17:hsm-session-pin-mvp");
    hasher.update(b"dist-cdn-bgp-v17:peer-announce-ttl-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v6:decl-token-subst-mvp");
    hasher.update(b"macro-service-v16:pointcut-and-compose-mvp");
    hasher.update(b"macro-docs-v17:m25-service-v16-honesty");
    hasher.update(b"runtime-concurrency-v3:caller-runs-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase135");
    hasher.update(b"lsp-semantic-v37:phase135-50-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase135-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:113.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v4:extension-computed-mvp");
    hasher.update(b"lang-props-v4:docs-honesty");
    // SynUI: inherit v12; gui-synui-v13 deferred → 137+
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v19:avgr-u-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    hasher.update(b"stdlib-os-v2:hostname-uptime-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v7:token-stream-mvp");
    hasher.update(b"macro-service-v17:pointcut-or-compose-mvp");
    hasher.update(b"macro-docs-v18:m26-service-v17-honesty");
    hasher.update(b"runtime-concurrency-v4:core-max-keepalive-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2500-2900");
    hasher.update(b"audit-exclude-fill-v1:phase136");
    hasher.update(b"lsp-semantic-v38:phase136-52-plus-real");
    hasher.update(b"driver-gate-cr400-v1:phase136-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:114.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v5:published-mvp");
    hasher.update(b"lang-props-v5:docs-honesty");
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v20:popcnt-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"stdlib-os-v3:disk-load-mvp");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v8:decl-expand-mvp");
    hasher.update(b"macro-service-v18:pointcut-not-within-mvp");
    hasher.update(b"macro-docs-v19:m27-service-v18-honesty");
    hasher.update(b"runtime-concurrency-v5:discard-oldest-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2600-2950");
    hasher.update(b"audit-exclude-fill-v1:phase137");
    hasher.update(b"lsp-semantic-v39:phase137-54-plus-real");
    hasher.update(b"driver-gate-cr410-v1:phase137-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:115.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v6:static-observers-mvp");
    hasher.update(b"lang-props-v6:docs-honesty");
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v21:bitmask-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"stdlib-os-v4:network-mvp");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    hasher.update(b"farm-lease-token-v19:hsm-key-id-mvp");
    hasher.update(b"dist-cdn-bgp-v19:peer-announce-hold-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"macro-token-v9:token-rewrite-mvp");
    hasher.update(b"macro-service-v19:pointcut-within-wildcard-mvp");
    hasher.update(b"macro-docs-v20:m28-service-v19-honesty");
    hasher.update(b"runtime-concurrency-v6:actor-shared-pool-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2600-3000");
    hasher.update(b"audit-exclude-fill-v1:phase138");
    hasher.update(b"lsp-semantic-v40:phase138-56-plus-real");
    hasher.update(b"driver-gate-cr420-v1:phase138-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:116.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(crate::lang_surface_v1::STRING_MULTILINE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::STRING_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_PRODUCT_TYPE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_NAMED_ELEMENTS_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::TUPLE_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::NESTED_QUALIFIED_PATH_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::NESTED_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::RANGE_CLOSED_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::RANGE_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::CHARACTER_DISTINCT_SCALAR_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CHARACTER_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_EXTERNAL_LABEL_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_OMIT_UNDERSCORE_MVP.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_LABELED_ARGS_REQUIRED.as_bytes());
    hasher.update(crate::lang_surface_v1::CALL_DOCS_HONESTY.as_bytes());
    hasher.update(crate::lang_surface_v1::FINAL_SEALED_REMOVED_DOCS.as_bytes());
    hasher.update(crate::lang_surface_v1::FINAL_ONLY_HONESTY.as_bytes());
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"wasm-simd16-guest-v22:all-any-true-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"gui-synui-v12:deferred-no-v13");
    hasher.update(b"farm-dist-v19:frozen-no-v20");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"lang-props-v6:static-observers-mvp");
    hasher.update(b"macro-token-v9:token-rewrite-mvp");
    hasher.update(b"macro-service-v19:pointcut-within-wildcard-mvp");
    hasher.update(b"runtime-concurrency-v6:actor-shared-pool-mvp");
    hasher.update(b"stdlib-os-v4:network-mvp");
    hasher.update(b"wasm-simd16-guest-v21:bitmask-mvp");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    hasher.update(b"audit-align-v1:ci-script-adr-2600-3050");
    hasher.update(b"audit-exclude-fill-v1:phase139");
    hasher.update(b"lsp-semantic-v41:phase139-58-plus-real");
    hasher.update(b"driver-gate-cr430-v1:phase139-real-behavioral");
    hasher.update(b"stdlib-parity-pin-v1:117.0");
    for id in syx_stdlib::HostFnId::all() {
        hasher.update(id.name().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

/// Phase 127 Track D: farm session-close + dist sticky-persist.
/// Phase 126 Track A: Macro M16 + Service DI v7.
/// Phase 126 Track B: SynUI v3 (opt-in run + checkbox/slider).
/// Phase 126 Track C: SIMD16 compare-lane MVP + inherited extract/replace.
/// Phase 126 Track D: farm C_Sign + dist sticky-TTL.
/// Phase 126 DX: audit 5300/3200, fill EXCLUDE, LSP semantic v28, driver CR320, stdlib 104.0 pin.
/// Phase 127 Track A: Macro M17 + Service DI v8 (advice proceed + `@Bean` MVP).
/// Phase 127 DX: audit 5450/3250, fill EXCLUDE, LSP semantic v29, driver CR330, stdlib 105.0 pin.
/// Phase 125 Track A: Macro M15 + Service DI v6.
/// Phase 125 Track A (+ folded v93_dx…v96_dx): invalidation bytes preserved in tip order.
/// Phase 125 Track B: SynUI deepen (run/signal/widgets/CI).
/// Phase 125 Track C: SIMD16 extract/replace + inherited Min/Max.
/// Phase 125 Track D: farm session-pin-persist + dist sticky-pop.
/// Phase 125 DX: audit 5150/3150, fill EXCLUDE, LSP semantic v27, driver CR310, stdlib 103.0 pin.
/// Phase 124 Track A: Macro M14 + Service DI v5.
/// Phase 124–125 Track B: SynUI path-dep GUI replacement.
/// Phase 124 Track C: SIMD16 Min/Max + inherited Not.
/// Phase 124 Track D: farm pkcs11-session + dist weighted-anycast.
/// Phase 124 DX: audit 5000/2900, fill EXCLUDE, LSP semantic v26, driver CR300, stdlib 102.0 pin.
/// Phase 122 Track A: Macro M12 + Service DI v3.
/// Phase 122 Track C: SIMD16 Or/Xor + inherited shift lane / guest-v11 / matrix-expand.
/// Phase 123 Track C: SIMD16 Not + inherited or/xor / shift lane / guest-v11 / matrix-expand.
/// Phase 123 DX: audit 4850/3050, fill EXCLUDE, LSP semantic v25, driver CR290, stdlib 101.0 pin.
/// Phase 123 Track A: Macro M13 + Service DI v4.
/// Phase 123 Track B: OS FontMgr default-paint OT v7.
/// Phase 123 Track D: farm pkcs11-so adapter + dist multi-PoP MVP.
/// Phase 122 DX: audit 4700/3000, fill EXCLUDE, LSP semantic v24, driver CR280, stdlib 100.0 pin.
/// Phase 121 Track A: Macro M11 Service DI v2 (real ctor, field Autowired, typed resolve, RB E009).
/// Phase 121 Track B: OT paint-matrix v5 + VF paint-variations v4.
/// Phase 122 Track B: default-paint OT v6 + dylib OT/VF export parity.
/// Phase 121 Track C: SIMD16 shift lane + inherited logic-or-shift / guest-v11.
/// Phase 121 Track D: farm KMS bridge MVP + BGP route-convergence.
/// Phase 121 Track D (+ folded v89_c…v93_c): invalidation bytes preserved in tip order.
/// Phase 122 Track D: farm PKCS#11 bridge MVP + dist multi-region mesh.
/// Phase 121 DX: audit 4550/2950, fill EXCLUDE, LSP semantic v23, driver CR270, stdlib 99.0 pin.
/// Phase 120 Track A: Macro M10 Service DI + exports schema + RB nested honesty.
/// Phase 120 Track B: OT feature-matrix + VF axes≥3.
/// Phase 120 Track C: SIMD16 i8x16 logic/shift (And) + inherited guest-v11 / matrix-expand.
/// Phase 121 Track C: adds `wasm-simd16-guest-v4:shift-lane-mvp` (Shl/Shr).
/// Phase 120 Track D: farm HSM-ref token MVP + BGP ops-visibility.
/// Phase 120 DX: audit 4400/2900, fill EXCLUDE, LSP semantic v22, driver CR260, stdlib 98.0 pin.
/// Phase 117 Track B (+ folded v86_a…v89_a): invalidation bytes preserved in tip order.
/// Phase 117 Track C: Farm real-connect + consensus-sync + token rotate.
/// Phase 117 Track D: BGP anycast MVP + remote-hostkey PoP + select no-env.
/// Phase 117 DX: audit 3950/2750, fill EXCLUDE, LSP semantic v19, driver CR230, stdlib 95.0 pin.
/// Phase 119 Track A: Macro M9 + OT script + VF multi-axis.
/// Phase 119 Track B: simd16 matrix expand + guest-v11 zero-scratch inherited.
/// Phase 119 Track C: Farm multi-follower-sync registry.
/// Phase 119 Track D: multi-hostkey-route PoP + select no-env.
/// Phase 119 DX: audit 4250/2850, fill EXCLUDE, LSP semantic v21, driver CR250, stdlib 97.0 pin.
/// Phase 113 DX (+ folded v82_d…v85_d): invalidation bytes preserved in tip order.
/// Phase 110 Track C (+ folded v79_b…v82_b): invalidation bytes preserved in tip order.
/// Phase 107 Track A (+ folded v70–v75 / v77–v78): invalidation bytes preserved in tip order.
/// Phase 107 Track B fingerprints (wired into `syx-cache-v79` later).
#[allow(dead_code)]
pub(crate) fn phase107_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "wasm-simd128-expand-v1:div-eq-lt-shuffle",
        "llvm-declare-call-v1:rt-hygiene",
    ]
}

/// Phase 108 Track B fingerprints.
#[allow(dead_code)]
pub(crate) fn phase108_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-bitwise-v1:gt-and-or-xor"]
}

/// Phase 109 Track A fingerprints.
#[allow(dead_code)]
pub(crate) fn phase109_track_a_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "gui-egui-wgpu-v1:winit-present",
        "gui-hb-paint-v1:string-shape",
        "gui-emoji-v1:color-mvp",
        "gui-vertical-rl-v1:label-mvp",
        "gui-doctor-wgpu-v1:phase109",
    ]
}

/// Phase 111 Track B fingerprints.
#[allow(dead_code)]
pub(crate) fn phase111_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-guest-v4:thin-arith-minmax-f32"]
}

/// Phase 110 Track B fingerprints.
#[allow(dead_code)]
pub(crate) fn phase110_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &[
        "wasm-simd128-guest-v3:spill-thin-shr",
        "wasm-simd128-div-assembled-v1:honest",
    ]
}

/// Phase 109 Track B fingerprints.
#[allow(dead_code)]
pub(crate) fn phase109_track_b_fingerprint_seeds() -> &'static [&'static str] {
    &["wasm-simd128-guest-v2:div-load-store-ext"]
}

/// Phase 97 (+ folded v53–v68): invalidation bytes preserved in tip order.
/// Phase 80 (+ folded v3/v4/v48–v51): generational-gc + inkwell-llvm-v6 + earlier invalidation bytes.
pub(crate) fn driver_options_with_deps(
    root: &Path,
    manifest: &SyxManifest,
    options: &DriverOptions,
) -> SyxResult<DriverOptions> {
    let mut opts = options.clone();
    if !manifest.dependencies.is_empty() {
        opts.resolved_packages = Some(load_resolved_packages(root)?);
    }
    Ok(opts)
}

/// Phase 71: protocol witness table v2 (MutRefCell wrap + post-sort renumber).
/// S3-I51: OnceLock — avoid re-hashing all embedded prelude on every `module_hash`.
pub(crate) fn witness_table_digest() -> String {
    static DIGEST: OnceLock<String> = OnceLock::new();
    DIGEST
        .get_or_init(|| {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(b"witness-v2:Iterator-next-renumber+MutRefCell-wrap:");
            for (_, src) in crate::embedded_std::embedded_protocols() {
                hasher.update(src.as_bytes());
            }
            for (_, src) in crate::embedded_std::core_prelude() {
                if src.contains("__witness_") || src.contains("func compare") {
                    hasher.update(src.as_bytes());
                }
            }
            format!("{:x}", hasher.finalize())
        })
        .clone()
}

/// Phase 66: class vtable layout hash for polymorphic dispatch invalidation.
pub(crate) fn vtable_layout_digest() -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"vtable-v1:runtime-read-indirect-dispatch");
    format!("{:x}", hasher.finalize())
}

/// Phase 68: WASM AOT tier-1/tier-2A/tier-2B fingerprint.
pub(crate) fn wasm_aot_tier_digest() -> String {
    format!(
        "wasm-aot-v3:tier1F-I+tier2A=ObjectGet,ArraySet,ValueEq,RefEq+tier2B=Await,Closure+iter-witness=1:sorted=ArrSorted:aot={}",
        if syx_config::wasm_aot_enabled() {
            "1"
        } else {
            "0"
        }
    )
}

pub(crate) fn build_syx_plan_hash(root: &Path) -> String {
    let path = root.join(build_syx::DEFAULT_BUILD_CONFIG);
    let text = std::fs::read_to_string(&path).unwrap_or_default();
    hash_source(&text)
}

pub(crate) fn plugin_manifest_hash(root: &Path) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    let lock = root.join("syx.lock");
    if lock.is_file() {
        if let Ok(text) = std::fs::read_to_string(&lock) {
            hasher.update(text.as_bytes());
        }
    }
    let plugins_dir = root.join("plugins");
    if plugins_dir.is_dir() {
        let mut paths: Vec<_> = std::fs::read_dir(&plugins_dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect();
        paths.sort();
        for p in paths {
            hasher.update(p.to_string_lossy().as_bytes());
            if let Ok(text) = std::fs::read_to_string(&p) {
                hasher.update(text.as_bytes());
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

/// S3-I51: OnceLock — avoid recounting embedded prelude `func` on every `module_hash`.
pub(crate) fn unified_mir_fn_count_digest() -> String {
    static DIGEST: OnceLock<String> = OnceLock::new();
    DIGEST
        .get_or_init(|| {
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(crate::embedded_std::internal_manifest_json().as_bytes());
            let mut fn_count = 0usize;
            for (_, src) in crate::embedded_std::core_prelude() {
                fn_count += src.matches("func ").count();
            }
            for (_, src) in crate::embedded_std::embedded_protocols() {
                fn_count += src.matches("func ").count();
            }
            hasher.update(fn_count.to_string().as_bytes());
            format!("{:x}", hasher.finalize())
        })
        .clone()
}

/// S3-I51: compile-time rustc identity (`SYX_RUSTC_VERSION` from build.rs) — no runtime spawn.
pub(crate) fn rustc_version_tag() -> String {
    option_env!("SYX_RUSTC_VERSION")
        .unwrap_or("unknown-rustc")
        .to_string()
}

pub(crate) fn host_feature_tag() -> String {
    format!(
        "native-rt-v1:mir-v25={}:mir-v24={}:mir-v23={}:mir-v22={}:mir-v21={}:mir-v20={}:mir-v19={}:mir-v18={}:mir-v17={}:mir-v16={}:mir-v15={}:mir-v14={}",
        if syx_mir::mir_opt_v25_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v24_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v23_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v22_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v21_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v20_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v19_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v18_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v17_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v16_aot_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v15_jit_barrier_enabled() {
            "1"
        } else {
            "0"
        },
        if syx_mir::mir_opt_v14_jit_barrier_enabled() {
            "1"
        } else {
            "0"
        }
    )
}

pub(crate) fn module_hash(path: &Path, root: &Path, deps: &[(String, String)]) -> String {
    use sha2::{Digest, Sha256};
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    hasher.update(cache_version_tag().as_bytes());
    hasher.update(build_syx_plan_hash(root).as_bytes());
    hasher.update(plugin_manifest_hash(root).as_bytes());
    hasher.update(unified_mir_fn_count_digest().as_bytes());
    hasher.update(witness_table_digest().as_bytes());
    hasher.update(vtable_layout_digest().as_bytes());
    hasher.update(wasm_aot_tier_digest().as_bytes());
    hasher.update(dependency_tree_digest(deps).as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(crate) fn hash_source(contents: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(contents.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub(crate) fn dependency_hashes_from_graph(
    graph: &ModuleGraph,
    self_key: &Path,
) -> Vec<(String, String)> {
    let mut deps: Vec<(String, String)> = graph
        .modules
        .iter()
        .filter(|(k, _)| *k != self_key)
        .map(|(k, u)| (k.display().to_string(), hash_source(&u.source.contents)))
        .collect();
    deps.sort_by(|a, b| a.0.cmp(&b.0));
    deps
}

pub(crate) fn read_module_source_for_cache(path: &str) -> Option<String> {
    if let Some(rest) = path.strip_prefix("<stdlib>/") {
        let name = rest.strip_suffix(".sy")?;
        let module = name.split('/').next()?;
        return embedded_std::lookup(module).map(|(_, src)| src.to_string());
    }
    std::fs::read_to_string(path).ok()
}

pub(crate) fn dependency_hashes_still_valid(deps: &[(String, String)]) -> bool {
    deps.iter().all(|(path, expected)| {
        read_module_source_for_cache(path).is_some_and(|c| hash_source(&c) == *expected)
    })
}

pub(crate) fn dependency_tree_digest(deps: &[(String, String)]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for (path, hash) in deps {
        hasher.update(path.as_bytes());
        hasher.update(hash.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
