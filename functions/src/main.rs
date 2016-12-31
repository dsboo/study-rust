fn main() {
	print_number(5);
	print_sum(1, 2);
	println!("add one {}", add_one(1));
	let x: i32 = diverges();
	let x: String = diverges();
}

fn print_number(x:i32) {
	println!("x is: {}", x);
}

fn print_sum(x:i32, y:i32) {
//fn print_number(x, y) { //error type, 함수는 인자의 타입을 꼭 선언해야함
	println!("sum is: {}", x + y);
}

fn add_one(x:i32) -> i32 { //러스트는 리턴타입을 -> type 으로 명시한다.
	x + 1
	//x + 1; // 러스트는 표현식이므로 ;는 컴파일에러가 발생,  return x + 1;은 동작
             // 표현식이라는 것은 무조건 값을 반환한다. x+1 은 return x+1을 뜻한다.
}

// !는 diverges라고 읽는다.
# fn diverges() -> ! {
#    panic!("This function never returns!");
# }
