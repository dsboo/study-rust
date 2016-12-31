fn main() {
	let x:i32 = 5; //변수 x는 i32 타입-타입은 생략가능 -이고 5가 바인딩되어 있다, 기본적으로 rust 변수는 immutable 속성을 갖는다.

	let (y, z) = (1,2); //패턴이 변수자리에 올수 있다.

	let mut a = 5;// mut is mean, a is re assigned value.
	a = 10; //its possible
	//x = 10; //mut 키워드가 없는 변수는 재할당이 불가능하다. 컴파일단계에서 에러

	let b:i32;

    //println!("Hello, world! {}", b); // rust is cannot use uninitially variable, compile error
    println!("Hello, world!");
}
