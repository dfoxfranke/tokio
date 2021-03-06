# 0.2.6 (December 19, 2019)

### Fixes
- `fs::File::seek` API regression (#1991).

# 0.2.5 (December 18, 2019)

### Added
- `io::AsyncSeek` trait (#1924).
- `Mutex::try_lock` (#1939)
- `mpsc::Receiver::try_recv` and `mpsc::UnboundedReceiver::try_recv` (#1939).
- `writev` support for `TcpStream` (#1956).
- `time::throttle` for throttling streams (#1949).
- implement `Stream` for `time::DelayQueue` (#1975).
- `sync::broadcast` provides a fan-out channel (#1943).
- `sync::Semaphore` provides an async semaphore (#1973).
- `stream::StreamExt` provides stream utilities (#1962).

### Fixes
- deadlock risk while shutting down the runtime (#1972).
- panic while shutting down the runtime (#1978).
- `sync::MutexGuard` debug output (#1961).
- misc doc improvements (#1933, #1934, #1940, #1942).

### Changes
- runtime threads are configured with `runtime::Builder::core_threads` and
  `runtime::Builder::max_threads`. `runtime::Builder::num_threads` is
  deprecated (#1977).

# 0.2.4 (December 6, 2019)

### Fixes
- `sync::Mutex` deadlock when `lock()` future is dropped early (#1898).

# 0.2.3 (December 6, 2019)

### Added
- read / write integers using `AsyncReadExt` and `AsyncWriteExt` (#1863).
- `read_buf` / `write_buf` for reading / writing `Buf` / `BufMut` (#1881).
- `TcpStream::poll_peek` - pollable API for performing TCP peek (#1864).
- `sync::oneshot::error::TryRecvError` provides variants to detect the error
  kind (#1874).
- `LocalSet::block_on` accepts `!'static` task (#1882).
- `task::JoinError` is now `Sync` (#1888).
- impl conversions between `tokio::time::Instant` and
  `std::time::Instant` (#1904).

### Fixes
- calling `spawn_blocking` after runtime shutdown (#1875).
- `LocalSet` drop inifinite loop (#1892).
- `LocalSet` hang under load (#1905).
- improved documentation (#1865, #1866, #1868, #1874, #1876, #1911).

# 0.2.2 (November 29, 2019)

### Fixes
- scheduling with `basic_scheduler` (#1861).
- update `spawn` panic message to specify that a task scheduler is required (#1839).
- API docs example for `runtime::Builder` to include a task scheduler (#1841).
- general documentation (#1834).
- building on illumos/solaris (#1772).
- panic when dropping `LocalSet` (#1843).
- API docs mention the required Cargo features for `Builder::{basic, threaded}_scheduler` (#1858).

### Added
- impl `Stream` for `signal::unix::Signal` (#1849).
- API docs for platform specific behavior of `signal::ctrl_c` and `signal::unix::Signal` (#1854).
- API docs for `signal::unix::Signal::{recv, poll_recv}` and `signal::windows::CtrlBreak::{recv, poll_recv}` (#1854).
- `File::into_std` and `File::try_into_std` methods (#1856).

# 0.2.1 (November 26, 2019)

### Fixes
- API docs for `TcpListener::incoming`, `UnixListener::incoming` (#1831).

### Added
- `tokio::task::LocalSet` provides a strategy for spawning `!Send` tasks (#1733).
- export `tokio::time::Elapsed` (#1826).
- impl `AsRawFd`, `AsRawHandle` for `tokio::fs::File` (#1827).

# 0.2.0 (November 26, 2019)

A major breaking change. Most implementation and APIs have changed one way or
another. This changelog entry contains a highlight

### Changed
- APIs are updated to use `async / await`.
- most `tokio-*` crates are collapsed into this crate.
- Scheduler is rewritten.
- `tokio::spawn` returns a `JoinHandle`.
- A single I/O / timer is used per runtime.
- I/O driver uses a concurrent slab for allocating state.
- components are made available via feature flag.
- Use `bytes` 0.5
- `tokio::codec` is moved to `tokio-util`.

### Removed
- Standalone `timer` and `net` drivers are removed, use `Runtime` instead
- `current_thread` runtime is removed, use `tokio::runtime::Runtime` with
  `basic_scheduler` instead.

# 0.1.21 (May 30, 2019)

### Changed
- Bump `tokio-trace-core` version to 0.2 (#1111).

# 0.1.20 (May 14, 2019)

### Added
- `tokio::runtime::Builder::panic_handler` allows configuring handling
  panics on the runtime (#1055).

# 0.1.19 (April 22, 2019)

### Added
- Re-export `tokio::sync::Mutex` primitive (#964).

# 0.1.18 (March 22, 2019)

### Added
- `TypedExecutor` re-export and implementations (#993).

# 0.1.17 (March 13, 2019)

### Added
- Propagate trace subscriber in the runtime (#966).

# 0.1.16 (March 1, 2019)

### Fixed
- async-await: track latest nightly changes (#940).

### Added
- `sync::Watch`, a single value broadcast channel (#922).
- Async equivalent of read / write file helpers being added to `std` (#896).

# 0.1.15 (January 24, 2019)

### Added
- Re-export tokio-sync APIs (#839).
- Stream enumerate combinator (#832).

# 0.1.14 (January 6, 2019)

* Use feature flags to break up the crate, allowing users to pick & choose
  components (#808).
* Export `UnixDatagram` and `UnixDatagramFramed` (#772).

# 0.1.13 (November 21, 2018)

* Fix `Runtime::reactor()` when no tasks are spawned (#721).
* `runtime::Builder` no longer uses deprecated methods (#749).
* Provide `after_start` and `before_stop` configuration settings for
  `Runtime` (#756).
* Implement throttle stream combinator (#736).

# 0.1.12 (October 23, 2018)

* runtime: expose `keep_alive` on runtime builder (#676).
* runtime: create a reactor per worker thread (#660).
* codec: fix panic in `LengthDelimitedCodec` (#682).
* io: re-export `tokio_io::io::read` function (#689).
* runtime: check for executor re-entry in more places (#708).

# 0.1.11 (September 28, 2018)

* Fix `tokio-async-await` dependency (#675).

# 0.1.10 (September 27, 2018)

* Fix minimal versions

# 0.1.9 (September 27, 2018)

* Experimental async/await improvements (#661).
* Re-export `TaskExecutor` from `tokio-current-thread` (#652).
* Improve `Runtime` builder API (#645).
* `tokio::run` panics when called from the context of an executor
  (#646).
* Introduce `StreamExt` with a `timeout` helper (#573).
* Move `length_delimited` into `tokio` (#575).
* Re-organize `tokio::net` module (#548).
* Re-export `tokio-current-thread::spawn` in current_thread runtime
  (#579).

# 0.1.8 (August 23, 2018)

* Extract tokio::executor::current_thread to a sub crate (#370)
* Add `Runtime::block_on` (#398)
* Add `runtime::current_thread::block_on_all` (#477)
* Misc documentation improvements (#450)
* Implement `std::error::Error` for error types (#501)

# 0.1.7 (June 6, 2018)

* Add `Runtime::block_on` for concurrent runtime (#391).
* Provide handle to `current_thread::Runtime` that allows spawning tasks from
  other threads (#340).
* Provide `clock::now()`, a configurable source of time (#381).

# 0.1.6 (May 2, 2018)

* Add asynchronous filesystem APIs (#323).
* Add "current thread" runtime variant (#308).
* `CurrentThread`: Expose inner `Park` instance.
* Improve fairness of `CurrentThread` executor (#313).

# 0.1.5 (March 30, 2018)

* Provide timer API (#266)

# 0.1.4 (March 22, 2018)

* Fix build on FreeBSD (#218)
* Shutdown the Runtime when the handle is dropped (#214)
* Set Runtime thread name prefix for worker threads (#232)
* Add builder for Runtime (#234)
* Extract TCP and UDP types into separate crates (#224)
* Optionally support futures 0.2.

# 0.1.3 (March 09, 2018)

* Fix `CurrentThread::turn` to block on idle (#212).

# 0.1.2 (March 09, 2018)

* Introduce Tokio Runtime (#141)
* Provide `CurrentThread` for more flexible usage of current thread executor (#141).
* Add Lio for platforms that support it (#142).
* I/O resources now lazily bind to the reactor (#160).
* Extract Reactor to dedicated crate (#169)
* Add facade to sub crates and add prelude (#166).
* Switch TCP/UDP fns to poll_ -> Poll<...> style (#175)

# 0.1.1 (February 09, 2018)

* Doc fixes

# 0.1.0 (February 07, 2018)

* Initial crate released based on [RFC](https://github.com/tokio-rs/tokio-rfcs/pull/3).
