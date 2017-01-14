fn main() {
	let v1 = vec![1, 2, 3];
	let v2 = vec![1, 2, 3];

	let answer = foo(&v1, &v2); // & 주소(참조) 형식을 사용함으로써 빌림을 구현한다.

	// we can use v1 and v2 here!

	let v3 = vec![];
	foo2(&v3); // foo2 함수가 빌린 변수를 변화시키기 때문에 error가 발생한다.


	let mut x = 5;
	{
		let y = &mut x;
		*y += 1; // its not error, y is mutable 빌림, 가변빌림이라고 표현, 원본 역시 가변변수여야한다.
	}
	println!("{}", x);

	let mut z = 5;

	let a = &mut z;     // -+ &mut borrow of x starts here
						//  |
	*a += 1;            //  |
						//  |
	println!("{}", z);  // -+ - try to borrow x here, its error
						// -+ &mut borrow of x ends here


	let b: &i32;
	{ 
		let c = 5;
		b = &c;
	}

	println!("{}", b); // its error y에 빌려주는 x의 스코프가 이미 끝나있기 때문이다.

	let d: &i32;
	let e = 5;
	d = &e; // its error, d는 e보다 먼저 선언되었기 때문에 d의 수명이 e보다 길어 자신보다 수명이 짧은 변수를 참조할 수 없다.

	println!("{}", d);

}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
	// do stuff with v1 and v2
	// return the answer
	42
}

fn foo2(v: &Vec<i32>) {
     v.push(5);
}

