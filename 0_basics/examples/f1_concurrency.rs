/* 

❯ can you explain to me simply what are the main mechanisms that rust uses to ensure thread safety - that no two
  threads can act at the same data concurrently? I haven't studied the topic of concurrent programming in Rust yet,
  but from what I learned so far I have a feeling it's enforced using the same principles the borrow checker relies
  on - as in, no two threads can hold a handle on the same variable? They have to either clone or the variable has to
  be read reference and then the other thread has to join before the main thread can mutata it again - something
  like this?

⏺ Your intuition is right in spirit, but there's a specific mechanism doing the work that's worth naming: Send and
  Sync, two marker traits.

  The core idea: ownership rules don't stop at thread boundaries — they extend into them. But "extend" needs a new
  question answered: is it safe for this type to cross a thread boundary at all? That's not something the borrow
  checker's aliasing rules alone can answer (a Rc<T>, for example, obeys all the normal borrow rules but is not safe
  to share across threads, because its refcount increments aren't atomic — two threads could race on it and corrupt
  the count). So Rust adds two traits:

  - Send: a type is Send if it's safe to move ownership of it to another thread. Almost everything is Send
  automatically (it's an auto-trait, compiler-derived). Rc<T> is the famous exception — that's why threads use Arc<T>
  (atomic Rc) instead.
  - Sync: a type is Sync if it's safe to share a reference &T to it across threads. Formally: T is Sync if &T is Send.

  thread::spawn's closure is required to be Send, and (for non-'static data) the compiler also enforces the
  borrow-checker rule you already know: no reference in the closure may outlive what it points to. Since a spawned
  thread's lifetime isn't tied to the parent's stack frame, plain spawn requires owned (move) or 'static data — which
  is exactly the wall you'd hit if you tried to pass a plain &data without move, as in the last snippet.

  So: ownership/borrowing rules are checked at compile time by lifetimes, and thread-safety of the type itself is
  checked at compile time by Send/Sync. Neither is a runtime check — if it compiles, a whole class of data races is
  already ruled out. This is the famous "fearless concurrency" pitch.

  Now, for the part you described — "clone it, or read-only until the other joins" — that's roughly right for two
  patterns, but there's a third that's more common for real shared mutable state:

  1. Move ownership in (what you saw with move ||) — one thread owns it, no sharing.
  2. Scoped threads / join then mutate — borrow it immutably, and the borrow checker (via thread::scope, stable since
  1.63) guarantees all threads finish before the borrow ends, so you can mutate afterward. This is close to what you
  described.
  3. Arc<Mutex<T>> — for actually sharing mutable state while threads are alive concurrently. Arc gives multiple
  threads shared ownership (like Rc, but thread-safe refcounting). Mutex<T> gives interior mutability with a runtime
  lock — .lock() blocks until exclusive access is available, and hands back a guard that derefs to &mut T. This is
  effectively "the borrow checker's exclusivity rule (one mutable access at a time), but enforced at runtime instead
  of compile time," because the compiler can't know at compile time which thread will get the lock when.

  There's also a fourth idiom worth knowing exists (I'll leave the details for when you get there): channels
  (std::sync::mpsc), which sidestep shared state entirely — "don't communicate by sharing memory; share memory by
  communicating." Data is moved (ownership transferred, Send-checked) through a queue instead of both threads touching
  the same memory.

  Notes:
  - Send — safe to move to another thread (auto-trait; Rc<T> opts out)
  - Sync — safe to share &T across threads ⟺ &T: Send (auto-trait; Cell/RefCell opt out — their interior mutability
  isn't atomic)
  - Lifetimes/borrow rules → checked at compile time, same mechanism as single-threaded code, extended by
  'static/scoped-thread requirements
  - Send/Sync → checked at compile time, but about the type, not a specific borrow
  - Mutex<T> → the runtime analogue of "one mutable borrow at a time"; Arc<T> → thread-safe Rc<T> for shared
  ownership; combo Arc<Mutex<T>> is the idiomatic "shared mutable state across threads"
  - Channels (mpsc) → ownership-transfer model, avoids shared mutable state altogether



-------------------------------------------------------------------------------------------------------------------------

❯ 1. this all sounds nice and simply-ish. I vaguly remember though that async rust is more of a default compared to
  traditional threads, but at the same time a million times more complicated (from what I heard?). If working with
  traditional threads is nice and simple (using the techniques you just told me about - Send, Sync, Mutex, channels,
  etc) then why is the async paradigm tends to be more common and is everywhere (from my understanding)?
  2. I also vaguely rememver hearing somewhere that Arc<Mutex<T>> style is what you end up writing when there is no
  disciplined technical leader overlooking your project and presumably this is not ideal? I think because it defeats
  the point of rust which actively avoids refcounting (gc) for performance purposes etc?

⏺ Good instinct to reach for, but the analogy points the wrong direction. Python's threading.Thread is still using
  real OS threads — the GIL doesn't change that. What the GIL does is serialize execution of Python bytecode: only one
  thread runs Python code at any instant, but it's still the OS doing real preemptive scheduling and
  context-switching between real OS-level threads, each with its own real stack. The GIL gets released during I/O
  calls (and periodically during CPU-bound loops) so other threads get a turn — but the underlying mechanism is
  exactly the heavyweight OS-thread machinery, just handicapped by a global lock on top of it.

  So threading.Thread isn't Rust's async task — it's closer to Rust's thread::spawn, except Python nerfs it so you
  never get real CPU parallelism from it (that's why CPU-bound Python work reaches for multiprocessing instead —
  separate processes, separate GILs, actual parallelism, at the cost of IPC instead of shared memory).

  The thing that's actually structurally identical to Rust's async tasks is asyncio — and this isn't a loose analogy,
  it's the same design: an event loop (≈ Tokio's runtime) runs lightweight coroutines (≈ Rust's tasks) on a small
  number of OS threads (often just one), and each coroutine cooperatively yields control at await points instead of
  being preemptively switched. Python even uses the identical async/await keywords for the identical reason. If you
  squint, asyncio.create_task(...) and tokio::spawn(...) are doing conceptually the same job.

  So the mapping is:

  This is why async dominates in networking/web-server/service code — that world is almost entirely "juggle thousands
  of mostly-idle I/O-bound things" — but is often overkill or even the wrong tool for CPU-bound parallel work (that's
  what rayon or plain thread::spawn + a thread pool is for).

  The "million times more complicated" reputation is real and comes from: async fn isn't a normal function, it
  desugars into a state machine and needs a runtime (tokio::main etc. — std doesn't ship one); Future/pinning show up
  when you go off the beaten path; lifetimes get gnarlier inside async blocks; and the ecosystem is split (Tokio vs
  async-std vs smol) in a way threads never are. It's not that async is conceptually harder to reason about
  safety-wise — it's that you're debugging a compiler-generated state machine instead of straightforward sequential
  code.

  2. Is Arc<Mutex<T>> a smell?

  There's real truth in what you heard, but it's a heuristic, not a law. Arc<Mutex<T>> is the "I give up on designing
  ownership, just let everyone share a handle and lock when they touch it" escape hatch. It compiles, it's safe (no
  data races — the type system still guarantees that), but it quietly re-introduces the exact thing Rust's ownership
  model was built to avoid: many places holding a handle to the same mutable thing, with correctness now depending on
  runtime discipline (lock ordering, not holding two locks in an order that can deadlock, not holding a lock across an
  .await and stalling the whole thing) rather than something the compiler checks for you at compile time.

  Your GC intuition is close but slightly off target — Arc's atomic refcounting isn't nearly as costly as a GC (it's a
  couple of atomic increments/decrements, not stop-the-world tracing), and it's opt-in only where you actually need
  shared ownership, not pervasive. The real cost isn't CPU cycles, it's design cost: reaching for Arc<Mutex<T>> by
  default often means you never worked out who actually owns the data, so you papered over that question by making it
  "everyone, sort of." A more deliberate design usually has one clear owner and uses message passing (channels) to let
  other threads/tasks request changes, rather than everyone reaching directly into shared state. That's the "with a
  disciplined lead" version — someone insisted on drawing ownership boundaries instead of taking the shortcut.

  That said — Arc<Mutex<T>> is a perfectly legitimate tool for genuinely shared, low-contention state (a shared cache,
  shared config, a connection pool). The smell is specifically reaching for it reflexively everywhere instead of
  asking "does this actually need to be shared, or did I just not think about who owns it."

  Notes:
  - Threads = safe by the same mechanisms already covered, but costly per-unit (OS-scheduled, MB-scale stacks) → great
  for CPU-bound parallelism, bad for massive I/O-bound concurrency
  - Async = lightweight tasks multiplexed by a runtime (Tokio) onto few OS threads → great for I/O-bound concurrency
  at scale, adds real complexity (state machines, Future, pinning, runtime dependency)
  - Arc<Mutex<T>> = compiler-safe but design-lazy; a runtime-enforced version of exclusivity, not a compile-time one —
  legitimate for genuinely shared state, a smell when used as a default instead of thinking about ownership
  - Alternative to reaching for shared-mutable-state: message passing (channels) — keep one owner, let others send
  requests to it

-------------------------------------------------------------------------------------------------------------------------

❯ are channels the only real alternative to Arc Mutex? Can you list and rank (best ones first) the most common
  alternatives? Or in other words can you give me a list of best practices for when working with threads (non async)
  e.g. reach for approaches x, y, z, first and then if you genuinely need shared mutable state do a, b, c (something
  like that). Also, I'm not looking for a very detailed description in each best practice point - I'm just exploring
  the topic that I'm planning to learn later

⏺ Good question — channels aren't the only alternative, they're just the most idiomatic one. Here's the rough pecking
  order Rust folks tend to reach for, best/simplest first, down to the "you actually need this" tier:

  1. Don't share at all — partition the data. Split ownership up front (e.g. slice::chunks_mut, or rayon's parallel
  iterators) so each thread just owns its piece outright. No synchronization needed because there's no overlap.
  2. Channels (mpsc, or crossbeam-channel for multi-consumer). Ownership moves through a queue instead of being shared
  — "share by communicating."
  3. Arc<T> with no lock at all, when the data is read-only after it's built. No mutation → no race → no Mutex needed,
  just shared read access.
  4. Scoped threads (thread::scope) when you need threads to borrow the parent's data temporarily. Compiler guarantees
  they all finish before the borrow ends, so no 'static/Arc required.
  5. Atomics (AtomicUsize, AtomicBool, etc.) for simple shared counters/flags — lock-free, cheaper than a Mutex for
  these narrow cases.
  6. RwLock<T> when reads vastly outnumber writes — many readers concurrently, writers get exclusive access.
  7. Arc<Mutex<T>> — the general-purpose fallback once you genuinely need shared mutable state and none of the above
  fit.
  8. Specialized concurrent crates (dashmap for a sharded concurrent hashmap, crossbeam for more exotic structures) —
  reach for these only once a plain Mutex is a measured bottleneck.

  Rule of thumb going down the list: each step trades away a compile-time guarantee for a runtime one, so the "best
  practice" is to stop as early on the list as your problem allows, and only drop to Arc<Mutex<T>> when you've
  actually confirmed the data must be both shared and mutated from multiple threads at once.
*/


