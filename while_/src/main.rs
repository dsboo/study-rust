fn main() {

	let mut x = 5; // mut x: i32
	let mut done = false; // mut done: bool

	while !done {
		x += x - 3;

		println!("{}", x);

		if x % 5 == 0 {
			done = true;
		}
	}

	loop { //while true

        x += x - 3;

        println!("{}", x); 

        if x % 5 == 0 { 
            break;
        }
	}

	for x in 0..10 {
	    if x % 2 == 0 { continue; }
	    println!("{}", x);
	}

	println!("Hello, world!");
}
