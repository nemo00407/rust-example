use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    
    // using insert to add elements
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");

    /*
    get method return an Option<&v>, if there's no value, get method return None.
    copied method get the value of Option<i32> from Option<&i32>.
    unwrap_or method returns the contained "Some" value or the provided default.
    */
    let blue_scores = scores.get(&team_name).copied().unwrap_or(0);
    println!("{blue_scores}");

    // iterate a hash map is same to a vector.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /* 
    For type that implement Copy trait, like i32, will be copied into the hash map. For owned value, like
    String, the value will be moved and the hash map wiil own the value.
    */

    /*
    In the hash map, each unique key will only have one value. Therefore, if calling "insert" twice, and
    give same key but different value, the hash map will only have one pair still, and the value will be
    overwrite.
    */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    /*
    Using "entry" and "or_insert" to check if the key already in the hash map. "entry" will return an enum called
    "Entry".
    "Entry" has two variant, "Occupied(OccupiedEntry)" and "Vacant(VacantEntry)". This is a core api to control and
    check whether the pair is in the hash map or not.
    */
    // "Yellow":10 will be inserted to the hash map.
    scores.entry(String::from("Yellow")).or_insert(10);
    // "Blue":25 will still the same
    scores.entry(String::from("Blue")).or_insert(10);
    /*
    if using the variant "Occupied", it need to use "if let" or "match" because compilier cannot know whether the key is 
    exist or not in the compile time.
    insert and get method is implement on Occupied.
    */
    let key = String::from("Blue");
    if let std::collections::hash_map::Entry::Occupied(mut blue) = scores.entry(key) {
        let value = blue.get();
        println!("blue: {value}");
        blue.insert(100);
    }
    println!("{:?}", scores);    


    /*
    application: Count each words appear how many times in a string
    */
    let text = String::from("Hello world hahaha world");
    let mut count = HashMap::new();
    for word in text.chars() {
        let count = count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{count:?}");
    /* 
    Sort hash map according the key or the value.
    It is unable to sort hash map directly, so usually we transfer it into a vector, then use sort_by_key to sort it.
    Actually, the hash map itself will not be sorted, if want to use data after sorted, using the sorted vec.
    */
    let mut map_to_vec: Vec<_> = count.iter().collect();
    map_to_vec.sort_by_key(|&(_key, value)| value);
    for (key, value) in map_to_vec {
        println!("'{}' {}", key, value);
    }
}