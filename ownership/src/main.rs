fn main() {

	let v = vec![1,2,3];
	let v2 = v;                    // 소유권이 이동됨
	//println!("v[0] is :{}", v[0]); // its occure error
	take(v); // same error

	let w = 1; // let w:i32 = 1;
	let w = v;
	println!("v is {}", v); // its not error, because i32 implements Copy
                            // every primitive type has Copy 특성


    println!("Hello, world!");
}

fn take(v: Vec<i32>) {
    // what happens here isn’t important.
}
