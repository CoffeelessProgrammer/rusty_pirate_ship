# Crates for Date & Time

- [std::time](https://doc.rust-lang.org/std/time/index.html)
- [chrono](https://crates.io/crates/chrono) ∙∙∙∙∙∙∙∙∙∙∙ [chrono::format](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)

## Snippets
```rs
// IMPORTS
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use chrono::{Utc, Local};
use std::thread::sleep;
```

```rs
    let start = Instant::now();

    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("Seconds since epoch: {}", duration.as_secs());

    let utc_now = Utc::now();
    let local_now = Local::now();

    println!("UTC: {}", utc_now);
    println!("Local: {}", local_now);
    println!("Formatted: {}", local_now.format("%b %d, %Y %H:%M"));

    
    let elapsed = start.elapsed();
    println!("Elapsed: {:?}", elapsed);
```