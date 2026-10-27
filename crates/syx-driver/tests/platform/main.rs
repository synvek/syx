//! SYX platform integration gates (feature-preserving suite).
//!
//! Layout:
//! - `cache_vNNN_invalidation` — MIR cache fingerprint contracts
//! - `dist_*` / `token_*` / `props_*` / `macro_di_*` / `os_*` — product-semantic platform nails
//! - `driver_gate` / `compile_run_*` — CR behavioral gates
//! - plugins / borrow / resultbuilder — evergreen platform surfaces

// --- cache invalidation contracts ---
mod cache_v108_invalidation;
mod cache_v109_invalidation;
mod cache_v110_invalidation;
mod cache_v111_invalidation;

// --- dist / CDN ---
mod dist_mailbox_multiprocess;
mod dist_mailbox_tcp_ga;
mod dist_transport_e2e;
mod dist_anycast;
mod dist_multihost;
mod dist_nonloopback;
mod dist_hostkey_configurable;
mod dist_hostkey_loopback_reject;
mod dist_bgp_anycast_pop;
mod dist_bgp_peer_exchange;
mod dist_multi_hostkey_route;
mod dist_bgp_ops;
mod dist_bgp_convergence;
mod dist_multi_region_mesh;
mod dist_multi_pop;
mod dist_weighted_anycast;
mod dist_sticky_pop;
mod dist_sticky_ttl;
mod dist_sticky_persist;
mod dist_sticky_multi_snapshot;
mod dist_peer_announce_sticky;
mod dist_peer_announce_ack;
mod dist_peer_announce_fanout;
mod dist_peer_announce_conflict_epoch;
mod dist_peer_announce_epoch_persist;
mod dist_peer_announce_ttl;
mod dist_peer_announce_priority;
mod dist_peer_announce_hold;
mod farm_hsm_key_id;
mod os_hostname_uptime;
mod os_disk_load;
mod os_network;
mod service_di_mvp;
mod codable_associated_enum_e2e;
mod token_expr_mvp;
mod token_decl_mvp;
mod token_decl_hygiene;
mod token_stmt;
mod token_expr_deepen;
mod bounded_task_queue;
mod props_computed_lazy_static;
mod props_computed_override;
mod builtin_static_let;
mod props_extension_computed;
mod props_published;
mod props_static_observers;
mod service_di_and_pointcut;
mod token_decl_deepen;
mod task_queue_caller_runs;
mod macro_di_or_pointcut;
mod macro_di_not_within;
mod macro_di_within_wildcard;

// --- Language surface: multiline string, closed range, call labels, nested types, tuple, Character ---
mod string_multiline;
mod closed_range;
mod call_labels;
mod nested_types;
mod tuple;
mod character;

// --- Stdlib / fmt / datetime / http behavioral nails ---
mod dict_remove;
mod crypto_pbkdf2;
mod tcp_stream_handle;
mod fmt_fail_closed;
mod fmt_check;
mod datetime_gregorian;
mod http_transport_fail;

// --- driver compile-run / gate ---
mod driver_gate;
mod compile_run_smoke;
mod compile_run_batch;
mod build_optional_weak_deinit;

// --- current audit fill (prior p105–p117 pad files deleted) ---
mod os_concurrency;

// --- evergreen platform ---
mod borrow_prover_mandatory;
mod file_watch_os;
mod plugin_cdn_prod;
mod plugin_dlclose;
mod plugin_marketplace;
mod resultbuilder_ga;
