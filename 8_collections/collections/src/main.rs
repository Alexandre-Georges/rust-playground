fn main() {
	let v: Vec<i32> = Vec::new();

	// By default numbers are i32
	let v = vec![1, 2, 3];

	// To append values
	let mut v = Vec::new();
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	{
		let v = vec![1, 2, 3, 4];
		// v is accessible
	} // v is freed when we leave the block

	// To access its elements, there are 2 ways
	let v = vec![1, 2, 3, 4, 5];

	// With an index, the program crashes if it is out of bounds
	let third: &i32 = &v[2];
	println!("The third element is {}", third);

	// With get where we can handle the 2 cases
	match v.get(2) {
		Some(third) => println!("The third element is {}", third),
		None => println!("There is no third element."),
	}

	// To iterate over a vector
	let v = vec![100, 32, 57];
	for i in &v {
		println!("{}", i);
	}

	// We can also change each element
	let mut v = vec![100, 32, 57];
	for i in &mut v {
		*i += 50;
	}

	// To have different types we can put them under an enum
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	// New empty string
	let mut s = String::new();
	// With an initial value
	let s = "initial contents".to_string();
	// This is equivalent
	let s = String::from("initial contents");

	// Update a string
	let mut s = String::from("foo");
	s.push_str("bar");
	println!("s is {}", s);

	// One character added
	s.push('l');

	let s1 = String::from("Hello, ");
	let s2 = String::from("world!");
	let s3 = s1 + &s2;
	println!("s3 is {}", s3);
	// s1 is inacessible as it has been modified s1.add(s2) and s2 is accessible

	// We can't get a specific character with [] because the sizes of characters can change
	// according to the alphabet
	let hello = "Здравствуйте";
	let s = &hello[0..4];
	// We actually get the first 4 bytes with corresponds to the 2 first chars
	println!("s is {}", s);

	// Converting the string in chars gives us each individual char, it knows where to split bytes
	for c in hello.chars() {
    println!("{}", c);
	}

	// Here we get bytes (2 bytes for 1 char for cyrillic)
	for b in hello.bytes() {
    println!("{}", b);
	}


	// Hashmaps
	use std::collections::HashMap;

	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	// To get an element
	let team_name = String::from("Blue");
	let score = scores.get(&team_name);
	println!("Score of the blue team {:?}", score);


	let teams = vec![String::from("Blue"), String::from("Yellow")];
	let initial_scores = vec![10, 50];

	// Types can also be specified if desired
	let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
	println!("{:?}", scores);

	// To iterate over each element
	for (key, value) in &scores {
		println!("{}: {}", key, value);
	}


	// After getting used in the insert, field_name and field_value are then unaccessible
	let field_name = String::from("Favorite color");
	let field_value = String::from("Blue");

	let mut map = HashMap::new();
	map.insert(field_name, field_value);
	println!("{:?}", map);


	// Overwriting the value
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Blue"), 25);
	println!("{:?}", scores);


	// Insert if the key does not exist
	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);
	println!("{:?}", scores);


	// Update based on the current entries
	let text = "hello world wonderful world";
	let mut map = HashMap::new();

	for word in text.split_whitespace() {
			let count = map.entry(word).or_insert(0);
			*count += 1;
	}
	println!("{:?}", map);
}
