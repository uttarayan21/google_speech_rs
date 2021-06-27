# [Google Speech][google_speech] Rust Bindings

rust bindings for the python module [google_speech][google_speech].

Check [desbma/google_speech][google_speech] for installation instructions (You need the python module to make this crate work).

TL;DR
```bash
pip install google_speech
```

Then install sox and mp3 dependencies for sox


For archlinux
```bash
sudo pacman -S sox libmad libid3tag twolame
```


Only has `Speech`.  
`SpeechSegment` not implemented.


## Example:

```rust
use google_speech::{Speech,Lang};
let hello = Speech::new("Hello, World", Lang::en_us).unwrap();
hello.play().unwrap();
```

[google_speech]: https://github.com/desbma/GoogleSpeech
