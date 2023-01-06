use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    // create password from provided byte string of chars
    let CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";

    let pass = create_password(20);
    println!("{}", pass);
    let char_password = create_password_from_charset(15, CHARSET);
    println!("{}", char_password);
}
// create a new password using the lenght and the given charset
fn create_password_from_charset(password_len: u32, charset: &[u8]) -> String {
    let mut rng = thread_rng();

    let password: String = (0..password_len)
        .map(|_| {
            // generate a random int between 0 and Charset length
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

fn create_password(password_len: u32) -> String {
    // creates an instance of of the type ThreadRng: pub struct ThreadRng { /* private fields */ }
    // the ThreadRng (or rng in our case) instance can be used to call methods like gen
    let mut rng = thread_rng();
    // create a random string
    let password: String = rng
        .sample_iter(Alphanumeric)
        .map(char::from)
        // .take(30) will create a new iterator that will only yield the first 30 elements
        // of the original iterator, and .map(char::from) will create a new iterator that
        // will apply the char::from function to each element of the iterator,
        // resulting in a new iterator of characters.
        .take(password_len as usize)
        .map(char::from)
        .collect();
    password
}
