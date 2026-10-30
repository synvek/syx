//! async adapter deepen (TcpReadAsync on blocking_pool).
// was: m3_b02_tcp_async

use std::io::Write;
use std::net::TcpListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .expect("run")
        .printed
        .join("\n")
}

/// Accept one connection, write payload, then idle until stop.
fn start_write_loopback(payload: &'static [u8]) -> (u16, Arc<AtomicBool>, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("addr").port();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_flag = Arc::clone(&stop);
    listener
        .set_nonblocking(true)
        .expect("nonblocking listener");
    let handle = thread::spawn(move || {
        while !stop_flag.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let _ = stream.write_all(payload);
                    let _ = stream.flush();
                    while !stop_flag.load(Ordering::Relaxed) {
                        thread::sleep(Duration::from_millis(10));
                    }
                    break;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });
    (port, stop, handle)
}

#[test]
fn m3_b02_tcp_async_policy_fingerprint() {
    let src = r#"
func main() {
  print(__syx_io_adapter_policy())
}
"#;
    let out = run(src);
    assert!(
        out.contains("blocking-pool-async-adapter-v1-tcp"),
        "ioAdapterPolicy fingerprint after deepen: {out}"
    );
}

#[test]
fn m3_b02_tcp_async_read_async_await() {
    for _ in 0..8 {
        let (port, stop, join) = start_write_loopback(b"B02_TCP_OK");
        thread::sleep(Duration::from_millis(20));
        // Embed path may ObjectGet instance methods; nail HostFn directly (facade still gated in meta).
        let out = run(&format!(
            r#"
async func main() {{
  let h = __syx_tcp_connect("127.0.0.1", {port})
  let text = await __syx_tcp_read_async(h, 64)
  print(text)
  __syx_tcp_handle_release(h)
}}
"#
        ));
        stop.store(true, Ordering::Relaxed);
        let _ = join.join();
        assert!(
            out.contains("B02_TCP_OK"),
            "await TcpReadAsync must return payload: {out}"
        );
    }
}

#[test]
fn m3_b02_tcp_async_does_not_starve_short_task() {
    // Task pool size=1: long work on blocking_pool must not starve a short Task.
    let src = r#"
func main() {
  __syx_task_set_worker_count(1)
  let longT = Task.detached {
    __syx_blocking_sleep(30000)
    print("LONG_DONE")
  }
  let shortT = Task.detached {
    print("SHORT_DONE")
  }
  __syx_task_join(shortT)
  print("SHORT_JOINED")
  __syx_task_cancel(longT)
  try {
    __syx_task_join(longT)
  } catch (err) {
    print("LONG_CANCELLED")
  }
}
"#;
    let started = Instant::now();
    let out = run(src);
    let elapsed = started.elapsed();
    assert!(
        out.contains("SHORT_DONE") && out.contains("SHORT_JOINED"),
        "short Task must complete: {out}"
    );
    assert!(
        elapsed.as_millis() < 1500,
        "blocking_pool path must not starve Task pool: elapsed={elapsed:?} out={out}"
    );
}

