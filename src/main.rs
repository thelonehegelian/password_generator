use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    // creates an instance of of the type ThreadRng: pub struct ThreadRng { /* private fields */ }
    // the ThreadRng (or rng in our case) instance can be used to call methods like gen
    let rng = thread_rng();
    // create a random string
    let rand_string: String = rng
        .sample_iter(Alphanumeric)
        .map(char::from)
        // .take(30) will create a new iterator that will only yield the first 30 elements
        // of the original iterator, and .map(char::from) will create a new iterator that
        // will apply the char::from function to each element of the iterator,
        // resulting in a new iterator of characters.
        .take(30)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
}
