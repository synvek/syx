//! SYX platform integration gates (feature-preserving suite).
//!
//! Layout:
//! - `cache_vNNN_invalidation` — MIR cache fingerprint contracts
//! - `dist_*` / `token_*` / `props_*` / `macro_di_*` / `os_*` — product-semantic platform nails
//! - `driver_gate` / `compile_run_*` — CR behavioral gates
//! - plugins / borrow / resultbuilder — evergreen platform surfaces

// --- cache invalidation contracts ---
mod cache_v111_invalidation;

// --- dist / CDN ---
mod bounded_task_queue;
mod builtin_static_let;
mod codable_associated_enum_e2e;
mod dist_anycast;
mod dist_bgp_anycast_pop;
mod dist_bgp_convergence;
mod dist_bgp_ops;
mod dist_bgp_peer_exchange;
mod dist_hostkey_configurable;
mod dist_hostkey_loopback_reject;
mod dist_mailbox_multiprocess;
mod dist_mailbox_tcp_ga;
mod dist_multi_hostkey_route;
mod dist_multi_pop;
mod dist_multi_region_mesh;
mod dist_multihost;
mod dist_nonloopback;
mod dist_peer_announce_ack;
mod dist_peer_announce_conflict_epoch;
mod dist_peer_announce_epoch_persist;
mod dist_peer_announce_fanout;
mod dist_peer_announce_hold;
mod dist_peer_announce_priority;
mod dist_peer_announce_sticky;
mod dist_peer_announce_ttl;
mod dist_sticky_multi_snapshot;
mod dist_sticky_persist;
mod dist_sticky_pop;
mod dist_sticky_ttl;
mod dist_transport_e2e;
mod dist_weighted_anycast;
mod farm_hsm_key_id;
mod macro_di_not_within;
mod macro_di_or_pointcut;
mod macro_di_within_wildcard;
mod os_disk_load;
mod os_hostname_uptime;
mod os_network;
mod props_computed_lazy_static;
mod props_computed_override;
mod props_extension_computed;
mod props_published;
mod props_static_observers;
mod service_di_and_pointcut;
mod service_di_mvp;
mod task_queue_caller_runs;
mod token_decl_deepen;
mod token_decl_hygiene;
mod token_decl_mvp;
mod token_expr_deepen;
mod token_expr_mvp;
mod token_stmt;

// --- Language surface: multiline string, closed range, call labels, nested types, tuple, Character ---
mod call_labels;
mod character;
mod closed_range;
mod nested_types;
mod string_multiline;
mod tuple;

// --- Stdlib / fmt / datetime / http behavioral nails ---
mod crypto_pbkdf2;
mod datetime_gregorian;
mod dict_remove;
mod fmt_check;
mod fmt_fail_closed;
mod fmt_write_align;
mod http_transport_fail;
mod tcp_stream_handle;

// --- driver compile-run / gate ---
mod build_optional_weak_deinit;
mod compile_run_batch;
mod compile_run_smoke;
mod driver_gate;

// --- current audit fill (prior p105–p117 pad files deleted) ---
mod os_concurrency;

// --- evergreen platform ---
mod borrow_prover_mandatory;
mod file_watch_os;
mod plugin_cdn_prod;
mod plugin_dlclose;
mod plugin_marketplace;
mod resultbuilder_ga;
