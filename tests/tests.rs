use google_speech::{Lang, Speech};
#[test]
fn test_hello_play() {
    let hello = Speech::new("Hello", Lang::en_us).unwrap();
    hello.play().unwrap();
}

#[test]
fn test_hello_save() {
    let hello = Speech::new("Hello", Lang::en_us).unwrap();
    unsafe { hello.save(concat!(env!("OUT_DIR"), "/hello.mp3")).unwrap() };
}

// #[test]
// fn test_hello_lang() {
//     let mut hello = Speech::new("Hello", Lang::af).unwrap();
//     hello.play().unwrap();
//     hello.lang(Lang::en_us).unwrap();
//     hello.play().unwrap();
// }
