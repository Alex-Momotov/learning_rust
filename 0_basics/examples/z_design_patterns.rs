/* 
TODO - make better notes of the below

⏺ The video's question is a good one. In Rust, every value has exactly one owner, and the owner is whoever decides when the
  value dies. Once you've decided that for each piece of data, everything else in the system either borrows from an owner or
  gets its own copy. Lifetimes are what the compiler uses to check that the borrows are sound.

  Questions to ask for each piece of data

  Ownership and lifetime
  1. Who decides when this dies? That component is the owner. Everyone else borrows, or makes a copy.
  2. Does it outlive the thing it came from? If data only lives while one message is being processed, borrow it. If it
     survives across messages (baselines, alerts, config), it must be owned, because the input buffer it would borrow from is
     gone after that message.
  3. Does it cross a boundary? Threads, tokio::spawn (which requires 'static), channels, and persistence all need owned data,
     or Arc for shared data. Borrowed views can't cross these boundaries.

  Access pattern

  4. Is it read, mutated, or both? Many readers: &T. One writer: &mut T, or the writer owns it. Many readers that are
     long-lived and may be on different threads: Arc<T> (for strings, Arc<str>).
  5. Is it on the hot path or the cold path? Allocating once per topic or per config load is free in practice. Allocating once
     per message is what hurts. Spend your design effort on the per-message path only.

  Reducing allocations

  6. Is the set of values bounded? Topic names, field names and schema keys repeat. Store each one once and refer to it by an
     ID (TopicId(u32)) or an Arc<str>. This is called interning.
  7. Can it point into the input buffer? Zero-copy parsing means &'a str slices of the raw message. If you sometimes need to
     change the text (e.g. unescaping JSON), use Cow<'a, str>: it borrows by default and only allocates when it has to.
  8. Can I reuse an allocation instead of making a new one? Keep a scratch Vec/String and call clear() on it each message, or
     use a per-batch arena (bumpalo).

  Design cost

  9. Will this lifetime spread? Lifetimes are contagious. Anything that holds an Event<'a> also needs 'a. Keep borrowed types
     short-lived and local, and keep long-lived types free of lifetimes.
  10. Have I measured? One to_owned() on a 20-byte string costs nanoseconds. Remove allocations where a profiler shows they
      matter, not everywhere.

  The pattern this leads to: owned state, borrowed views

  Your instinct is right for the per-message path. The refinement is that state which persists between messages has to be
  owned:

  // long-lived: owns its data, no lifetime parameter
  struct State { topics: HashMap<String, TopicStats> }

  // per-message: a view into the input buffer, never outlives it
  struct Event<'a> { topic: &'a str, payload: &'a str }

  fn ingest(state: &mut State, raw: &str) {
      let ev = parse(raw);                              // Event<'_>: zero allocation
      match state.topics.get_mut(ev.topic) {            // &str lookup into HashMap<String, _>: zero allocation
          Some(stats) => stats.observe(&ev),
          None => { state.topics.insert(ev.topic.to_owned(), TopicStats::new(&ev)); }  // allocates once per NEW topic
      }
  }

  get_mut works with a &str even though the map's keys are Strings, so the common case doesn't allocate. A common trap is
  state.topics.entry(ev.topic.to_string()), which allocates on every message, including ones for topics you've already seen.

  This means the only allocations happen when something becomes persistent: a new topic, an alert that outlives the message, a
  baseline snapshot. That's rare by nature, so the hot path stays allocation-free.

  This fits your sync-core/async-shell split well. The async shell owns the network buffer, and the sync core receives
  &str/&[u8] and works on borrowed views of it. The core's State owns everything that survives. Borrowed data never needs to
  cross a tokio::spawn, so the 'static requirement never comes up in the hot path.

✻ Worked for 32s · done 5:36 AM

❯ in addition to all this it can be the case that one subsystem is done with processing a message and can pass it to a
  different subsystem (e.g. the one that stores select examples of error messages) and this is when MOVING the String would
  be most appropriate instead of copying and dropping the original?

⏺ Yes. Moving is the natural way to hand something over when the first subsystem is done with it. A move transfers ownership
  without touching the heap. Only the String's 24-byte header (pointer, length, capacity) gets copied, and the compiler
  statically forbids any further use of the old variable. Nothing is allocated, nothing is freed, and there's no extra drop.

  fn process(msg: String, samples: &mut ErrorSamples) {
      if is_violation(&msg) {          // borrow while inspecting
          samples.keep(msg);           // move: ownership passes to the sample store
          return;                      // msg no longer usable here, and that's enforced at compile time
      }
      // ...msg dropped at end of scope otherwise
  }

  How useful this is depends on who owns the bytes in the first place:

  - One String per message: moving is ideal, as above. The message changes owner, so there's no copy and no drop.
  - One shared buffer per batch (the zero-copy setup from before): each message is only an &'a str view of that buffer. You
    can't move a slice out of a buffer you don't own, so to keep a sample you need to_owned(). That's fine here, because error
    samples are on the cold path (question 5).
  - A reused buffer (question 8): moving it away would cost you the reuse, and the next read would have to allocate a new
    buffer anyway. So it's better to copy the rare sample and keep the buffer.

  There's also a middle option that's common in streaming: bytes::Bytes, a reference-counted buffer that you can slice
  cheaply. buf.slice(start..end) gives you a separately owned handle to one message, without copying, while the rest of the
  buffer stays shared. The error store can hold that slice for as long as it wants, and the memory is freed when the last
  handle is dropped. It's essentially a move for a piece of a shared buffer. Kafka clients and tokio/hyper use it for exactly
  this.

  So the rule of thumb:
  - Move when you own the whole thing and are done with it.
  - Copy when you're keeping a small piece of something you borrowed, and it's off the hot path.
  - Use Bytes when you keep pieces of a shared buffer often enough that copying shows up in a profile.

✻ Baked for 15s · done 5:56 AM

❯ what about a situation like this:
  thread A - reads input messages and compares to statistical baseline. If violation is found it is passing it to thread B
  which is responsible for processing the violation, and holding onto the entire String message for a while as an example.

  Can a move of a String occur from one thread to another, such that thread B will own it, and no new allocations happen?

⏺ Yes. A String can move from thread A to thread B with no new allocation. You send it through a channel, and ownership goes
  with it:

  use std::sync::mpsc;
  use std::thread;

  let (tx, rx) = mpsc::sync_channel::<String>(1024);   // bounded queue of violations

  // thread B: owns every violation it receives
  let b = thread::spawn(move || {
      for msg in rx {                  // msg: String, owned by B now
          keep_as_example(msg);        // B can hold on to it as long as it likes
      }
  });

  // thread A
  for msg in input {
      if violates_baseline(&msg) {     // borrow to inspect
          tx.send(msg).unwrap();       // move: A gives up ownership, B receives it
      }                                // otherwise msg is dropped here
  }

  What actually crosses the channel is the 24-byte header (pointer, length, capacity). The heap bytes stay where they are, and
  B now owns that same memory. The compiler checks two things for you:
  - A can't touch msg after send. It's moved, so a later use is a compile error. That rules out a data race.
  - String is Send, meaning it's safe to transfer between threads. Channels only accept Send types.

  Three details:

  1. The channel itself may allocate. Unbounded channels (mpsc::channel) allocate internal storage as they grow. A bounded
     channel (sync_channel(n), or crossbeam::channel::bounded(n)) sets up its slots once at creation, so sending doesn't
     allocate. Bounded is also what you want in a streaming system anyway: if B falls behind, A blocks instead of using up
     unbounded memory.
  2. Freeing on another thread is fine. When B eventually drops the String, it frees memory that A's allocation created. The
     global allocator handles this. Some allocators make a cross-thread free slightly more expensive, but at violation
     frequency it doesn't matter.
  3. This only works for owned data. With the zero-copy batch buffer from before, A only has &'a str views, and those can't go
     to a long-lived thread B, because B could outlive the buffer. You'd then pick one of the options from the last answer:
     to_owned() the violating message (cheap, since violations are rare), or send a Bytes slice, which is also Send.


*/

fn main() {
    
}


