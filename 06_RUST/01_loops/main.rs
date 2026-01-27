
fn main() {
	
	let x:i8 = 10;

	if x > 0 {
		println!("x is positive");
	} else if x < 0 {
		println!("x is negative");
	} else {
		println!("x is zero");
	}

	let mut i = 0;

	loop {
		println!("i: {}", i);

		if i == 20 {
			break;
		}

		i += 1;
	}

	

	i = 1;

	'outer: loop {
		println!("i: {}", i);

		if i == 10 {
			break;
		}

		i += 1;

		'inner: loop {
			println!("i: {}", i);

			if i == 10 {
				break 'outer;
			}

			i += 1;
			continue 'inner;
		}
	}

	match x {
		0 => println!("x is zero"),
		1..5=>println!("x is between 1 to 5(both inclusive)"),
		_ => println!("x is some other value"),
	}

	
	for i in 1..=500 {
		println!("i: {}", i);
	}

	
	let mut i = 0;

	while i <= 10 {
		println!("i: {}", i);
		i += 1;
	}
}


