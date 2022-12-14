use std::collections::hash_map::RandomState;
use std::collections::HashMap;
fn main() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Check for a specific one.
    // When collections store owned values (String), they can still be
    // queried using references (&str).
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed."),
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    // Iterate over everything.
    for (book, review) in &book_reviews {
        println!("{book}: \"{review}\"");
    }

    // HashMap::from
    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    for (planet, distance) in &solar_distance {
        println!("{planet}: \"{distance}\"");
    }

    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats
        .entry("defence")
        .or_insert_with(random_stat_buff);

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    // modify an entry before an insert with in-place mutation
    player_stats
        .entry("mana")
        .and_modify(|mana| *mana += 200)
        .or_insert(100);

    for (k, v) in &player_stats {
        println!("{k}: \"{v}\"");
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        /// Creates a new Viking.
        fn new(name: &str, country: &str) -> Viking {
            Viking {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }

    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }

    let s = RandomState::new();
    let mut map = HashMap::with_hasher(s);
    map.insert(1, 2);

    for (k, v) in &map {
        println!("{k}: \"{v}\"");
    }

    let s = RandomState::new();
    let mut map = HashMap::with_capacity_and_hasher(10, s);
    map.insert(1, 2);

    for (k, v) in &map {
        println!("{k}: \"{v}\"");
    }
    assert!(map.capacity() >= 10);

    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    println!("keys-------------");
    for key in map.keys() {
        println!("{key}");
    }

    println!("values-------------");
    for val in map.values() {
        println!("{val}");
    }

    let mut vec: Vec<&str> = map.into_keys().collect();
    vec.sort_unstable();
    assert_eq!(vec, ["a", "b", "c"]);

    let mut map_mut = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("values_mut-------------");
    for val in map_mut.values_mut() {
        *val = *val + 10;
    }

    for val in map_mut.values() {
        println!("{val}");
    }

    let map_into_value = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let mut vec_into_value: Vec<i32> = map_into_value.into_values().collect();
    // The `IntoValues` iterator produces values in arbitrary order, so
    // the values must be sorted to test them against a sorted array.
    vec_into_value.sort_unstable();
    assert_eq!(vec_into_value, [1, 2, 3]);
}
