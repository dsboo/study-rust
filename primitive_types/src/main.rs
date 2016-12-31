fn main() {

	//1. boolean 1byte
	let a = true;
	let b: bool = false;

	//2. char 4byte
	let c = 'x';

	//3. numeric - fixed size signed - i8, i16, i32, i64
	//4. numeric - fixed size unsigned - u8, u16, u32, u64
	//5. numeric - variable size(32bit cpu 32bit, 64bit cpu 64bit) signed - isize
	//6. numeric - variable size(32bit cpu 32bit, 64bit cpu 64bit) unsigned - usize
	//7. numeric - fixed size float - f32, f64

	//8. array
	let d = [1,2,3];
	let e = [0; 20]; // a:[i32; 20]
	e.len(); // size of array

	let names = ["aaa", "bbb", "ccc"]; //names: [&str; 3]
	println!("The second names {}", names[1]); // The second names bbb

	//9. slice
	let f = [0, 1, 2, 3, 4];
	let middle:&[i32] = &f[1..4]; // let middle:&i32, A slice of a: just the elements 1, 2, 3
	let complete = &f[..]; // A slice containing all of the elements in f

	//10. str

	//11. tuples
	let g = (1, "hello"); // let g:(i32, &str), fixed size and ordered list
	let mut h = (1, 2); // x: (i32, i32)
	let y = (2, 3); // y: (i32, i32)
	x = y; // 튜플이 동일한 길이일때 대입 가능
	
	let (x, y, z) = (1, 2, 3); //여러개의 바인딩을 한번에 대입할 수 있음
	println!("x is {}", x); 

	//(0,) // single-element tuple
	//(0) // zero in parentheses, 괄호안의 제로

	let tuple = (1, 2, 3);

	let x = tuple.0; // index로 접근 가능
	let y = tuple.1;
	let z = tuple.2;

	//12. 함수
	fn foo(x: i32) -> i32 {x}
	let x:fn(i32) -> i32 = foo; // x is function pointer


    println!("Hello, world!");
}
