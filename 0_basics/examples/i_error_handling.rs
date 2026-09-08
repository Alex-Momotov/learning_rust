#![allow(unused)]

/* 
Rust has no exceptions. No throw, no try/catch. Instead there are two separate mechanisms, chosen by whether the failure is EXPECTED or a BUG.
    EXPECTED    Result<V, E>    "This can legitimately fail". The caller handles it explicitly.     
                                The error is a value being returned. Error handling is just normal data flow.
                                The happy path and the error path are the same machinery.
    BUG         panic!          "The program is wrong". You handle it by fixing the code.
                                A panic in the main thread kills the program with exit code 101.
                                A panic in a spawned thread kills that thread only.
    
Where the line actually falls. The question to ask: could a correct program hit this at runtime through no fault of its own?
    Yes -> Result   File not found, network refused, malformed input, parse failure, permission denied. The world is allowed to be like this.
    No -> panic!    Index out of bounds, unwrap() on None, a broken invariant, arithmetic overflow.
                    Reaching one means your code has a bug, and there's no sensible recovery — the state you assumed is already false.

panic! vs exit(1)
Panic - kills the thread only, does all the resource cleanups, prints a meaningful message. Preferred over exit().
exit() - kills the entire process, doesn't do any resource cleanup, doesn't print a message.


When to use what
- match — when you want to handle both cases explicitly.
- ? — when you want to pass the error up to your caller (the default in real code).
- unwrap/expect — when crashing is acceptable (tests, prototypes).
- unwrap_or/if let Ok — when you have a sensible fallback.
                    
*/

use std::{fs::File, io::Read, num::ParseIntError, path::Path};

fn main() -> Result<(), std::io::Error> {
    // _____________________________________________________________________________________________
    // Three good ways to use RESULT or OPTION

    let res = Ok(1);

    // 1. Match — the explicit, no-magic way. This is the foundation — everything else is shorthand for this.
    match res {
        Ok(n)  => {},   // Happy path
        Err(e)      => {},   // Error path
    }

    // 2. Let else - predict that Result is Ok and extract T into the main scope of the function. Diverge if the prediction is wrong.
    let Ok(i) = res else {
        return;
    };

    // 3. If let - quick side effect if the Result is Ok
    if let Ok(i) = res {
        println!("{:?}", i);
    }
    
    // _____________________________________________________________________________________________
    // OPTION   (NULL STORY)
    // Rust has no null and it has no uninitialised variables.
    // Absence is modelled EXPLICITLY with the Option<T> enum.
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    
    let present: Option<i32> = Some(5);     // Initialise option with a value
    let absent:  Option<i32> = None;        // Initialise an empty option
    let string_opt = Some(String::from("hi"));
    let str_opt = Some(&1);

    // inspect
    present.is_some();   // True if present
    present.is_none();   // True if absent

    // Unwrap with panic
    present.unwrap();                    // Get value or panic
    present.expect("whatevs");           // Get value or panic with a message

    // Unwrap with fallback
    absent.unwrap_or(0);                 // Get value or default
    present.unwrap_or_default();         // Get value or type's default  (0 for i32)
    absent.unwrap_or_else(|| my_func()); // Get value or closure
    
    // transform what's inside, stay in Option
    present.map(|x| x * 2);         // Option<T> -> Option<U>
    present.and_then(|x| Some(x));  // Closure returns option - flat-maps
    present.zip(absent);                 // Option<(A, B)> if both are Some
    present.filter(|x| *x > 0);    // Some -> None if predicate fails

    present.or(absent);                  // another Option
    present.or_else(|| my_func_opt());   // another Option from closure

    // borrowing conversions (the ones that fix borrow errors)
    present.as_ref();       // &Option<T>     -> Option<&T>     The answer to "cannot move out of borrowed content."
    present.as_mut();       // &mut Option<T> -> Option<&mut T>
    string_opt.as_deref();  // Option<String> -> Option<&str>
    str_opt.cloned();       // Option<&T>     -> Option<T>   (also .copied())
    present.take();         // move the value out, leave None behind

    // Option is also an iterator of zero or one items, so iterator methods word on it
    Some(1).into_iter().map(|x| x + 1).collect();

    // ? works on Option inside a function, which itself returns an Option - early-return None
    fn get_uppercase(s: &str) -> Option<String> {
        s.get(0..1)?;
        Some(s.to_uppercase())
    }

    // _____________________________________________________________________________________________
    // RESULT
    // Result is just an enum. There's no machinery behind it. Result is Option where the "nothing" case can tell you what went wrong.
    // A function that can fail returns a Result, and the compiler forces you to deal with both cases — you can't accidentally ignore the error.

    // enum Result<T, E> {
    //     Ok(T),      <- success, holds a value of type T
    //     Err(E),     <- failure, holds an error of type E
    // }

    let res = Ok(1);

    // inspect
    res.is_ok();
    res.is_err();

    // convert to option (discart one side)
    res.ok();       // Option<T>
    res.err();      // Option<E>

    // Unwrap with panic
    // Fine for quick scripts, examples, and prototyping. Avoid in real code — a bad input crashes the whole program.
    res.unwrap();                    // Get value or panic
    res.expect("whatevs");           // Get value or panic with a message

    // Unwrap with fallback
    absent.unwrap_or(0);                 // Get value or default
    res.unwrap_or_default();             // Get value or type's default  (0 for i32)
    absent.unwrap_or_else(|| my_func()); // Get value or closure
    
    // Transform what's inside, stay in Result
    res.map(|x| x * 2);         // Result<T> -> Result<U>
    res.and_then(|x| Ok(x));    // Closure returns Result - flat-maps

    res.or(Ok(1));                  // another Result
    res.or_else(|x| my_func_res());   // another Result from closure

    // borrowing conversions (the ones that fix borrow errors)
    res.as_ref();       // &Result<T>     -> Result<&T>     The answer to "cannot move out of borrowed content."
    res.as_mut();       // &mut Result<T> -> Result<&mut T>

    
    // ----------------
    // Result is #[must_use] -> meaning itgoring the Result type produces a warning.
    let _ = my_func_res(); // is the explicit opt-out

    // A Result with only one type parameter is an alias - a Result with one generic param type pre-filled.
    pub type Resultt<T> = std::result::Result<T, std::io::Error>;
    let r = Resultt::<i32>::Ok(1);
    


    // _____________________________________________________________________________________________
    // ? - THE QUESTION MARK OPERATOR
    // Used on Result or Option to early return from a function that itself returns Result or Option of the same type.
    // It does three things:
    //  1. Unwrap on success -> the variable becomes available for the rest of the function. You continue writing the happy path.
    //  2. Return early on a failure
    //  3. Convert the error type via From on the way out

    fn some_func() -> Result<String, String> {
        let config = read_config()?;
        Ok(config)
    }

    // Desugars to:
    fn some_func2() -> Result<String, String> {
        let config = match read_config() {
            Ok(v)   => v,
            Err(e)  => return Err(From::from(e))
        };
        Ok(config)
    }

    // A function can call five things that fail five different ways, and as long as your error type implements
    // From for each, every call site is just ?
    //      fn load(path: &Path) -> Result<Config, MyError> {
    //          let text = fs::read_to_string(path)?;       // io::Error   -> MyError
    //          let raw: Raw = toml::from_str(&text)?;      // toml::Error -> MyError
    //          let port = raw.port.parse::<u16>()?;        // ParseIntError -> MyError
    //          Ok(Config { port })
    //      }
    // When one of the From impl blocks is missing you get "the trait bound MyError: From<io::Error> is not satisfied".
    // The inline fix for it is .map_err() :
    //      let text = fs::read_to_string(path).map_err(MyError::Io)?;

    
    // ? on a Result → function must return Result
    fn get_contents(path: &str) -> Result<String, std::io::Error> {
        let mut f = File::open(path)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        Ok(content)
    }

    // ? on an Option → function must return Option
    fn first_char(s: &str) -> Option<char> {
        let c = s.chars().next()?;   // unwraps Some, or returns None early
        Some(c.to_ascii_uppercase())
    }

    // _____________________________________________________________________________________________
    // Custom errors
    // A custom error is just a type you own — usually an enum with one variant per failure mode. The variants carry payload
    // (the path, the bad string, the underlying io::Error, etc), and callers respond by matching on variants instead of catching subclasses.

    // What makes it a real error type rather than just an enum are three trait impls:
    // 1. Debug - required because unwrap/expect and mian() -> Result<...> print the error via Debug, and because the Error trait demands it
    // 2. Display - you write it by hand; std has no derive. It's the one line human message. Convention:
    //    lowercase, no trailing period, and don't repeat the wrapped error's message.
    // 3. std::error::Error - usually an empty impl. It's what lets your error be converted into Box<dyn Error> / anyhow::Error
    //    and where you optionally implement source() to expose the underlying cause.

    // And also the From. ? calls From::from on the way out, so one impl From<io::Error> for ConfigError is what
    // turns every falliable call in that function into a bare ?.

    // Practical split: 
    // Libraries - define their own enum errors, because the error type is part of the public API and callers need to match on it.
    // Applications - mostly use Box<dyn Error> (std-only) or anyhow::Error since they mostly don't care about matching. 
    // In the real code nobody write the boilderplace below - instead thiserror derives Display, From, and source from attributes.

    // _____________________________________________________________________________________________
    // CUSTOM ERROR TYPES
    // A custom error is just a type you own — usually an enum with one variant per failure mode.
    // No exception hierarchy, no inheritance: you don't subclass an error, you enumerate the ways an op fails.
    // The error type is part of your public signature — changing it is a breaking change for your callers.
    // Three impls make a type a "proper" error:  Debug (derive) + Display (by hand) + Error (usually empty).

    // The shape ------
    #[derive(Debug)]                        // required — Error: Debug + Display. unwrap/expect/main print vi Debug.
    enum ConfigError {
        Missing(PathBuf),                   // variants carry whatever the message needs to be useful
        Io(std::io::Error),                 // wrapping a lower-level error = recording the cause
        BadPort { raw: String, source: ParseIntError },
    }

    // Display — the human message ------
    // Rules: lowercase, no trailing period, and DON'T restate the wrapped error (the chain printer adds it).
    impl std::fmt::Display for ConfigError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ConfigError::Missing(p)          => write!(f, "config file not found: {}", p.display()),
                ConfigError::Io(_)               => write!(f, "could not read config"),   // cause supplies the detail
                ConfigError::BadPort { raw, .. } => write!(f, "invalid port: {raw}"),
            }
        }
    }

    // The Error trait — the interop marker ------
    // Empty impl is enough. source() is optional but is what builds the "caused by" chain.
    impl std::error::Error for ConfigError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                ConfigError::Io(e)                  => Some(e),
                ConfigError::BadPort { source, .. } => Some(source),
                ConfigError::Missing(_)             => None,   // the buck stops here
            }
        }
    }

    // From — this is what makes `?` work ------
    // `?` calls From::from on the error on its way out. One impl per foreign error you want to absorb.
    impl From<std::io::Error> for ConfigError {
        fn from(e: std::io::Error) -> Self { ConfigError::Io(e) }
    }

    // Now every fallible call is a bare `?`, whatever it failed with:
    fn load(path: &Path) -> Result<u16, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::Missing(path.to_path_buf()));   // your own failure: construct it directly
        }
        let text = std::fs::read_to_string(path)?;                  // io::Error   -> ConfigError   (via From)
        let raw  = text.trim().to_string();
        let port = raw.parse::<u16>()
            .map_err(|source| ConfigError::BadPort { raw, source })?;   // no From impl -> map_err inline
        Ok(port)
    }

    // The payoff for the caller: failure modes are data, so they can be matched and handled differently.
    match load(Path::new("app.toml")) {
        Ok(port)                          => println!("port {port}"),
        Err(ConfigError::Missing(_))      => println!("using defaults"),   // recover from this one
        Err(e)                            => println!("fatal: {e}"),       // Display
    }

    // Single failure mode? A struct (often a unit struct) is enough — same three impls.
    #[derive(Debug)]
    struct EmptyInput;

    // Boxing — the app-level escape hatch ------
    // Box<dyn Error> is a trait object: "some error, I only promise it can Display".
    // A blanket impl means ANY error converts into it, so `?` absorbs everything with zero From impls.
    // Cost: the variants are gone — you can't match, only print (or .downcast_ref::<ConfigError>()).
    fn quick(path: &Path) -> Result<u16, Box<dyn std::error::Error>> {
        let port = std::fs::read_to_string(path)?.trim().parse::<u16>()?;   // two unrelated error types, both fine
        Ok(port)
    }

    // Ergonomics ------
    pub type Result2<T> = std::result::Result<T, ConfigError>;   // library convention: crate-local Result alias

    // #[non_exhaustive] on a public error enum = "I may add variants later".
    // Forces downstream matches to include a `_ =>` arm, so adding a variant isn't a breaking change.
    #[non_exhaustive]
    #[derive(Debug)]
    enum PublicError { Io(std::io::Error) }

    // Keep error types SMALL. Every Result<T, E> is sized max(T, E) — a fat error taxes the happy path.
    // Clippy's result_large_err fires past ~128 bytes; the fix is to Box the fat variant: Parse(Box<BigThing>).

    // Real-world: nobody hand-writes the above ------
    // thiserror = derive the boilerplate (libraries). Display, From and source all come from attributes:
    //     #[derive(Debug, thiserror::Error)]
    //     enum ConfigError {
    //         #[error("config file not found: {0}")]
    //         Missing(PathBuf),
    //         #[error("could not read config")]
    //         Io(#[from] std::io::Error),          // #[from] generates the From impl AND source()
    //         #[error("invalid port: {raw}")]
    //         BadPort { raw: String, source: ParseIntError },   // a field named `source` is picked up automatically
    //     }
    //
    // anyhow = one dyn error for applications. Box<dyn Error> with a backtrace and .context():
    //     fn load(path: &Path) -> anyhow::Result<u16> {
    //         let text = fs::read_to_string(path).context("reading config")?;   // adds a layer to the chain
    //         let port = text.trim().parse().context("parsing port")?;
    //         ensure!(port > 1024, "port {port} is privileged");                // bail!/ensure! = early Err
    //         Ok(port)
    //     }
    //
    // Rule of thumb:  library -> thiserror enum (callers must be able to match).
    //                 binary  -> anyhow (you log and exit; variants buy you nothing).
    //                 Mixing both in one workspace is the normal setup, not a smell.

    // vs Python ------
    // - No `raise` up an implicit channel: an error is a return value, so it's visible in every signature.
    // - No `except SomeError` on a class hierarchy: you match enum variants (a closed set the compiler checks).
    // - `except Exception` ≈ Box<dyn Error>: catches everything, tells you nothing specific.
    // - __cause__/`raise from` ≈ source(); Python chains it for you, in Rust you wire it yourself (or #[from]).

    
    // _____________________________________________________________________________________________
    // main can return a Result

    //   1. main can return Result, which is what lets you use ? at the top level:
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let n: i32 = "42".parse()?;
        Ok(())
    }
    // main returning Err Result is for when you want to do graceful shutdown on an error


    //   One rule that trips people up: ? and returning Ok(...) only work in a function whose return type is itself a
    //   Result (or Option). main can be made to return one:

    //   fn main() -> Result<(), Box<dyn std::error::Error>> {
    //       let n: i32 = "42".parse()?;
    //       println!("{n}");
    //       Ok(())
    //   }
    
    // _____________________________________________________________________________________________
    // Divergence macros
    panic!("config file corrupt: {path}");   // unrecoverable, with a message for a human
    unreachable!();                          // logically impossible — a bug if reached
    todo!();                                 // not written yet; I intend to write it
    unimplemented!();                        // deliberately unsupported, permanently
    assert!(n > 0, "n must be positive");    // check an invariant (stays in release)
    debug_assert!(inv_holds());              // check an invariant (debug builds only)
    std::process::exit(1);                   // stop the process now — no unwinding, no Drop
    
}

/* 
7. Dynamic errors + the ecosystem

  Box<dyn Error> — the std answer

  A trait object that erases the concrete type. It works with ? across mixed error types because of a blanket impl:
  any E: Error + 'static converts into it automatically.

  fn run() -> Result<(), Box<dyn std::error::Error>> {
      let text = fs::read_to_string("app.toml")?;   // io::Error
      let cfg: Raw = toml::from_str(&text)?;        // toml::de::Error
      let port: u16 = cfg.port.parse()?;            // ParseIntError
      Ok(())
  }

  Zero dependencies, and fine for main and small programs. What you give up: the caller can't match on the failure
  (only downcast_ref::<io::Error>(), which is clunky and requires guessing the type), there's no way to attach
  context, and Debug prints only the outermost message — so main shows Error: No such file or directory (os error 2)
  with no path and no cause chain.

  Use Box<dyn Error + Send + Sync + 'static> if the error ever crosses a thread or enters async code. The bare form
  isn't Send, and you'll discover that at the worst moment.

  thiserror — derive the boilerplate from item 6

  thiserror = "2"

  #[derive(Debug, thiserror::Error)]
  pub enum ConfigError {
      #[error("cannot read config at {path}")]
      Io { path: PathBuf, #[source] source: io::Error },

      #[error("config is not valid TOML")]
      Parse(#[from] toml::de::Error),

      #[error("missing required field `{0}`")]
      MissingField(&'static str),

      #[error("port {0} is out of range 1-65535")]
      PortOutOfRange(u32),
  }

  That generates the exact Display, Error::source, and From impls you hand-wrote — 40 lines down to attributes. The
  attributes to know:

  - #[error("...")] → the Display body; {0} / {path} interpolate fields directly.
  - #[from] → generates From and marks the field as source (so ? works on it).
  - #[source] → marks the cause without generating From — for variants like Io that need extra context, so ? can't
  build them blindly.
  - #[error(transparent)] → delegate Display and source straight through, for a pure wrapper variant.

  Critically, thiserror produces a plain enum with plain impls. There's no runtime cost and no trace of the crate in
  your public API — it's a proc macro that disappears after compilation.

  anyhow — one opaque error type for applications

  anyhow = "1"

  use anyhow::{Context, Result, bail, ensure};

  fn load(path: &Path) -> Result<Config> {       // Result<Config, anyhow::Error>
      let text = fs::read_to_string(path)
          .with_context(|| format!("reading config from {}", path.display()))?;
      let raw: Raw = toml::from_str(&text).context("parsing config")?;
      ensure!(raw.port > 0, "port must be positive");
      if raw.name.is_empty() { bail!("name is empty"); }
      Ok(raw.into())
  }

  anyhow::Error is Box<dyn Error + Send + Sync> with the rough edges fixed: it's one word wide, it captures a
  backtrace, .context() attaches a message at each layer, and — the big one — its Debug impl prints the whole chain,
  so main gives you:

  Error: reading config from /etc/app.toml

  Caused by:
      0: No such file or directory (os error 2)

  Anything implementing Error + Send + Sync + 'static converts in via ?, including your thiserror enums. And you can
  get the type back when you need to: err.downcast_ref::<ConfigError>().

  Which to use

  Library → thiserror. Binary → anyhow.

  The reason is who's on the other end. A library's caller is code, which needs to match on the failure mode to decide
  what to do — so give them a typed enum. An application's caller is a human reading stderr, who needs a legible
  chain of what was being attempted — so opaque-plus-context is exactly right, and enumerating every variant of every
  failure your app can have is pointless work.

  Two corollaries: never put anyhow::Error in a public library signature (you erase the caller's ability to react and
  add a public dependency), and it's completely normal for one workspace to use both — thiserror enums in the core
  crates, anyhow in the binary, glue code, and main, with the enums converting into anyhow::Error for free at the
  boundary.

                      library / core crate       binary / app / glue
  error type          thiserror enum             anyhow::Error
  caller does         match on variants          print and exit
  signature           Result<T, ConfigError>     anyhow::Result<T>

  Others you'll see: eyre / color-eyre (anyhow forks with customisable, prettier reports — popular in CLIs), and snafu
  (thiserror-alike with built-in context selectors). failure and error-chain are dead; ignore them in old blog posts.

*/


// _____________________________________________________________________________________________

/* 

8. Context & chaining

 The problem ? leaves behind. Bare propagation gives you the innermost fact and nothing else:

 Error: No such file or directory (os error 2)

 Which file? During what? io::Error doesn't carry a path — it can't, it's a thin wrapper over an OS errno. Every
 layer above knew something the layer below didn't, and ? threw all of it away.

 The fix is a chain: each layer attaches what it knows, and the printer renders the layers top-down as a narrative.

 let text = fs::read_to_string(path)
     .with_context(|| format!("reading config from {}", path.display()))?;

 Error: starting the ingest worker

 Caused by:
     0: reading config from /etc/app.toml
     1: No such file or directory (os error 2)

 That reads as: what we were doing → what it needed → why it failed. Three layers, three facts, no repetition.

 .context() vs .with_context(): the first takes a value and evaluates it always; the second takes a closure and only
 runs on the error path. Use .context("static message") for literals and .with_context(|| format!(..)) whenever
 you're formatting — otherwise you pay for a format! on every successful call.

 .context() also works on Option, which is often the neatest way to turn absence into a reportable error:

 let port = raw.port.context("config is missing `port`")?;

 What a good context message says

 Name the operation being attempted, not the failure. The printer already frames it as an error, and "Caused by:
 failed to read config: failed to open file: No such file" is three redundant words in a row.

 .context("reading config from /etc/app.toml")     // good — the operation
 .context("connecting to broker at kafka:9092")    // good — includes the identity
 .context("failed to read config")                 // weak — "failed" is implied
 .context("error")                                 // useless

 Each layer adds exactly what the layer below couldn't know: the path, the URL, the record offset, the tenant id, the
 retry attempt. If you have nothing to add, add nothing — a bare ? is the correct call, and context on every single
 line is noise that buries the two lines that mattered.

 For a streaming pipeline that means the identifying data is the point: .with_context(|| format!("validating record
 {offset} on topic {topic}")) is what turns a 3am page into a two-minute fix.

 Printing chains

 With anyhow, the format specifier picks the view:

 println!("{e}");     // top message only:  reading config from /etc/app.toml
 println!("{e:#}");   // one-line chain:    reading config from /etc/app.toml: No such file...
 println!("{e:?}");   // multi-line with "Caused by:" — and what `main` uses

 Use {e:#} in structured logs (one line per event), {e:?} for terminal output.

 With plain std types there's no built-in chain printer — you walk source() yourself:

 let mut src: Option<&dyn Error> = err.source();
 while let Some(e) = src {
     eprintln!("  caused by: {e}");
     src = e.source();
 }

 which is precisely the boilerplate anyhow exists to delete.

 With thiserror, context is fields, not calls. Same principle, typed:

 #[error("reading config at {path}")]
 Io { path: PathBuf, #[source] source: io::Error },

 The variant carries the path so callers can use it, not just read it.

 Wrap, convert, propagate, or swallow

 ┌───────────────────────────────────────────────────────────────────────┬────────────────────────────────────────┐
 │                               situation                               │                   do                   │
 ├───────────────────────────────────────────────────────────────────────┼────────────────────────────────────────┤
 │ you have identifying data the source lacks                            │ .context(..)                           │
 ├───────────────────────────────────────────────────────────────────────┼────────────────────────────────────────┤
 │ the source already fully identifies itself, and you'd only restate it │ bare ?                                 │
 ├───────────────────────────────────────────────────────────────────────┼────────────────────────────────────────┤
 │ callers need to branch on this failure                                │ convert to your own variant            │
 ├───────────────────────────────────────────────────────────────────────┼────────────────────────────────────────┤
 │ you're at a boundary you own and continuing is genuinely correct      │ log it, use a default                  │
 ├───────────────────────────────────────────────────────────────────────┼────────────────────────────────────────┤
 │ you feel like it                                                      │ nothing — resist context on every line │
 └───────────────────────────────────────────────────────────────────────┴────────────────────────────────────────┘

 Anti-patterns worth naming. Don't collapse an error to a String early (.map_err(|e| e.to_string())?) — you lose the
 chain, the type, and any chance of matching on it downstream. Don't restate the source's message in your own. And
 don't log::error! and propagate the same error, or it gets reported twice at different layers; log where you handle
 it, propagate where you don't.

 Backtraces: anyhow captures one automatically when RUST_BACKTRACE=1 is set and prints it after the cause chain. For
 custom types, a std::backtrace::Backtrace field with #[backtrace] does the same. That's the bridge to the next item
 — reading what a panic or an error actually dumps.

 Progress: 8/10.
*/

// _____________________________________________________________________________________________

/*
⏺ 9. Panics in depth

  Anatomy of the message

  thread 'main' panicked at src/ingest.rs:42:31:
  called `Option::unwrap()` on a `None` value
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  Read src/ingest.rs:42:31 first — always. File, line, column of the call in your code, and 90% of the time you're
  done before reading anything else. Then read line 2 for what kind of failure it was. The note: is boilerplate.

  Line 1 is trustworthy because of #[track_caller]: unwrap, expect, indexing and friends are annotated so the location
  reported is the caller's, not somewhere inside core/src/option.rs. Without it every unwrap panic would point at the
  same std file.

  thread 'main' matters too — see below.

  Backtraces

  RUST_BACKTRACE=1 cargo run        # your frames
  RUST_BACKTRACE=full cargo run     # plus all std/runtime frames

  How to read one: frames are innermost-first, and the top ~5 are always panic machinery. Skip anything named
  rust_begin_unwind, core::panicking::*, core::option::unwrap_failed, std::panicking::*. The first frame in your own
  crate is the culprit; everything below it is the call path that got you there, which is what tells you why the value
  was None.

     0: rust_begin_unwind              ← skip
     1: core::panicking::panic_fmt     ← skip
     2: core::panicking::panic         ← skip
     3: myapp::ingest::parse_record    ← START HERE  (this is line 42)
     4: myapp::ingest::run_batch       ← who called it
     5: myapp::main

  In release builds you get addresses instead of line numbers unless you keep symbols:

  [profile.release]
  debug = 1          # line tables, no size cost at runtime

  In tests, output is captured unless the test fails — cargo test -- --nocapture if you need it anyway.

  Where the panic lands

  - Main thread → process dies, exit code 101.
  - Spawned thread → only that thread dies. thread '<unnamed>' panicked appears on stderr and the process keeps
  running, usually with a queue that no longer drains. handle.join() returns Err. This is the nastiest failure mode in
  a worker-pool design: the symptom is a hang, not a crash.
  - Async task (tokio) → the task dies, the runtime survives, JoinHandle yields a JoinError. Same silent-stall risk
  unless you check join results.

  So in any long-running service, install a panic hook so panics reach your logging instead of just stderr:

  std::panic::set_hook(Box::new(|info| {
      log::error!("panic: {info}");      // info has the location and the message
  }));

  unwind vs abort

  Default is unwind: the stack unwinds running every Drop, so buffers flush and locks release.

  [profile.release]
  panic = "abort"

  abort skips all of that — SIGABRT, process gone immediately. Smaller binaries, no landing-pad code, faster compiles.
  Choose it for embedded, or for containers where the recovery strategy is "restart the pod anyway" and you'd rather
  not run destructors over corrupted state. Keep unwind when you isolate failures per request/task, when you cross an
  FFI boundary, or when you rely on #[should_panic] tests — those need unwinding to work.

  (Panicking while unwinding — a panic inside a Drop — always aborts. That's why Drop impls must not panic.)

  catch_unwind is not catch

  let result = std::panic::catch_unwind(|| risky());
  if let Err(payload) = result {
      let msg = payload.downcast_ref::<&str>()
          .copied()
          .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
          .unwrap_or("unknown panic");
  }

  The payload is Box<dyn Any>, so you downcast to get the message. Legitimate uses: never let a panic cross into C
  (it's UB), isolating one request in a server, test harnesses, plugin hosts. It does nothing under panic = "abort",
  and using it for ordinary error handling is a misuse — the state you were operating on is already presumed broken.

  The usual suspects

  When you see a panic, it's almost always one of these:

  v[10]                   // index out of bounds: the len is 3 but the index is 10
  &s[0..5]                // byte index 5 is not a char boundary  ← UTF-8, not length
  opt.unwrap()            // called `Option::unwrap()` on a `None` value
  res.unwrap()            // called `Result::unwrap()` on an `Err` value: ...
  a / b                   // attempt to divide by zero
  x + y                   // attempt to add with overflow   ← debug only!
  cell.borrow_mut()       // already borrowed: BorrowMutError  ← RefCell, runtime borrow check

  Integer overflow is the one that bites in production: it panics in debug builds and silently wraps in release,
  because overflow-checks defaults to off there. If wrapping would be a correctness bug in your data path, turn it on
  explicitly ([profile.release] overflow-checks = true) or use checked_add / saturating_add and handle it as a value.

  The workflow

  1. Read file:line:col. Open it.
  2. Read the message — which of the suspects above is it?
  3. If the line isn't obviously wrong, RUST_BACKTRACE=1 and find the first frame in your crate, then read downwards
  for how you got there.
  4. Fix by making the assumption explicit: replace the unwrap with expect("…") stating the invariant, or — usually
  better — with the Result/Option handling that the panic proved you needed.

*/

// _____________________________________________________________________________________________

/* 

⏺ 10. Testing & taste

  Tests are panic-driven

  A #[test] fails by panicking, which is why every assert! macro panics. Inside tests, unwrap is not just acceptable —
  it's the idiomatic failure report, since it carries the line number.

  assert!(cfg.port > 0);
  assert_eq!(cfg.port, 8080, "default port changed");   // prints left/right on failure
  assert!(matches!(err, ConfigError::MissingField("port")));

  ? in tests

  Return a Result from the test and ? works throughout — much cleaner than a chain of unwraps:

  #[test]
  fn loads_valid_config() -> Result<(), Box<dyn std::error::Error>> {
      let cfg = load(Path::new("tests/fixtures/valid.toml"))?;
      assert_eq!(cfg.port, 8080);
      Ok(())
  }

  An Err return fails the test and prints the error with Debug. Note you can't combine this with #[should_panic].

  Testing the error paths

  This is where most real bugs live, and it's the half people skip.

  #[test]
  fn rejects_missing_port() {
      let err = load(Path::new("tests/fixtures/no_port.toml")).unwrap_err();
      assert!(matches!(err, ConfigError::MissingField("port")));
  }

  #[test]
  fn error_message_names_the_file() {
      let err = load(Path::new("/nope.toml")).unwrap_err();
      assert!(err.to_string().contains("/nope.toml"));
  }

  matches! is the tool of choice — it pattern-matches without requiring PartialEq on the error type. For anyhow
  errors, recover the type with err.downcast_ref::<ConfigError>(), or assert on format!("{err:#}") when what you care
  about is the rendered chain. Asserting on exact message strings is brittle for internal errors and entirely
  reasonable for user-facing CLI output — that message is the contract there.

  #[should_panic]

  #[test]
  #[should_panic(expected = "port must be positive")]
  fn rejects_zero_port() {
      Config::new(0);
  }

  Always include expected — it substring-matches the panic message. Without it the test passes when the code panics
  for a completely unrelated reason, which is exactly the bug you were trying to catch. (Requires unwinding, so it
  doesn't work under panic = "abort".)

  Library vs binary policy

  ┌───────────────┬───────────────────────────────────────────────┬─────────────────────────────┐
  │               │             library / core crate              │    binary / application     │
  ├───────────────┼───────────────────────────────────────────────┼─────────────────────────────┤
  │ error type    │ thiserror enum, #[non_exhaustive]             │ anyhow::Error               │
  ├───────────────┼───────────────────────────────────────────────┼─────────────────────────────┤
  │ unwrap/expect │ denied outside tests via clippy               │ fine at startup, sparingly  │
  ├───────────────┼───────────────────────────────────────────────┼─────────────────────────────┤
  │ panics        │ only on caller contract violation, documented │ fine; install a panic hook  │
  ├───────────────┼───────────────────────────────────────────────┼─────────────────────────────┤
  │ context       │ fields on the variant                         │ .context() at each boundary │
  ├───────────────┼───────────────────────────────────────────────┼─────────────────────────────┤
  │ main          │ n/a                                           │ -> Result, exit code 1      │
  └───────────────┴───────────────────────────────────────────────┴─────────────────────────────┘

  Document both in rustdoc — clippy::missing_errors_doc and missing_panics_doc will nag you into it:

  /// Loads and validates the config.
  ///
  /// # Errors
  /// Returns [`ConfigError::Io`] if the file can't be read, or
  /// [`ConfigError::MissingField`] if a required key is absent.
  ///
  /// # Panics
  /// Panics if `capacity` is zero.

  The taste rules

  - Bad input is a Result; a broken invariant is a panic. A library that panics on malformed user data is a broken
  library.
  - Errors are public API. Adding a variant is a breaking change without #[non_exhaustive]; changing a Display string
  breaks anyone who parsed it.
  - Don't over-granularise. One variant per thing a caller would do differently, not one per call site.
  - Convert at boundaries, not at every layer — From impls at the crate edge, bare ? inside.
  - Never stringify early. .map_err(|e| e.to_string())? destroys the chain and the type.
  - Log where you handle, propagate where you don't. Doing both double-reports.
  - unwrap is an assertion. If you can't say why it holds, you wanted ?.
*/

/* 
Clippy config that makes panics impossible.

[lints.clippy]
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
arithmetic_side_effects = "deny"
unreachable = "deny"
unimplemented = "deny"
unchecked_time_subtraction = "deny"
todo = "deny"
string_slice = "deny"
panic_in_result_fn = "deny"
panic = "deny"
exit = "deny"
as_conversions = "deny"

*/

/* 
----------------------------------------------------------
The result/panic split
- Expected failure (bad input, missing file) → the library gives you a Result. The compiler makes you handle it. You're not left to fate.
- Panic → almost always means your code has a bug (indexed past the end, unwrapped a None you shouldn't have). The fix is to fix the bug, not to "catch" it.

  Every language has the category "programmer made a logic error." The question is what happens then:
  ┌───────────┬───────────────────────────────────────────────────────────────────────────────────────────┐
  │           │                             On a logic bug (bad index, etc.)                              │
  ├───────────┼───────────────────────────────────────────────────────────────────────────────────────────┤
  │ C/C++     │ Undefined behavior — silent corruption, security holes, "works on my machine," heisenbugs │
  ├───────────┼───────────────────────────────────────────────────────────────────────────────────────────┤
  │ Python/JS │ Exception thrown — but can be silently swallowed anywhere up the stack; you may ship it   │
  ├───────────┼───────────────────────────────────────────────────────────────────────────────────────────┤
  │ Rust      │ Panic — immediate, loud, deterministic, memory-safe stop at the exact bug site            │
  └───────────┴───────────────────────────────────────────────────────────────────────────────────────────┘
  Expected failures are Result (compiler-enforced), bugs are panics (fail loud).

----------------------------------------------------------
There are two kinds of errors - recoverable and unrecoverable:
 
- Recoverable - Result<T, E>
	The caller can't ignore it — the error is baked into the return type, and the compiler nudges you to deal with it.

- Unrecoverable errors → panic! (aborts)
  	A panic is not meant to be caught and recovered from.

	The good news is that panics are limited to a small set of known operations such as:
		- Indexing out of bounds — v[i], slice[i]. (Avoid with v.get(i) → returns Option.)
		- .unwrap() / .expect() — you explicitly wrote "panic if this is None/Err." These are opt-in; they don't sneak up.
		- Integer divide by zero — a / 0.
		- Arithmetic overflow in debug builds — a + b past the max (wraps in release; use checked_add etc. when it matters).
		- .unwrap()-flavored assertions — assert!, todo!, unimplemented!, explicit panic!.

	Rust's documentation convention is that any function which can panic has a # Panics section in its docs saying exactly when.	
	For example, Vec's indexing docs literally say "Panics if index is out of bounds."
	And there's a strong community norm: a well-behaved library does not panic on data you give it — it returns Result/Option instead.

Practical ways to avoid panics:
1. Prefer non-panicking APIs when the failure is expected: v.get(i) over v[i], checked_add over +, match/? over .unwrap().
2. Treat .unwrap() as a signal you're writing. It's fine in prototypes/tests. In production, each one is a 
  claim "this genuinely can't fail" — and if you're unsure, use ? or handle it.
3. Read the # Panics section when a function has one. It's there.
4. Panics stay contained: by default a panic unwinds one thread. A server can isolate a panicking request
   handler so one bad request doesn't kill the process.
*/

fn my_func() -> i32 {42}
fn my_func_opt() -> Option<i32> {Some(42)}
fn my_func_res() -> Result<i32, &'static str> {Ok(42)}
fn read_config() -> Result<String, String> {Ok("config".to_string())}


struct MyError;