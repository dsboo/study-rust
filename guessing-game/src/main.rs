extern crate rand; //디펜던시에 명시된 것을 사용하겠다고 알림, use rand와 같은 역할도 수행한다.

//러스트는 기본적으로 prelude라는 라이브러리만 자동으로 import한다
use std::io; //따라서 다른 라이브러리를 이용하려면 use 키워드로 import
use rand::Rng;
use std::cmp::Ordering;


fn main() { // 프로그램 진입점
    println!("Guess the number!"); //화면에 문자열을 출력하는 macro 매크로 사용시에는 매크로명 뒤에 !가 붙는다.
	let secret_number = rand::thread_rng().gen_range(1, 101);
	println!("The secret number is: {}", secret_number);

	loop {
			println!("Please input your guess.");
			let mut guess = String::new(); // let 변수결합 키워드(현재는 변수선언이라고 생각하면 됨)
			// mut 해당변수가 변경가능하다는 의미, rust는 기본적으로 변수의 변경이 불가능하다.
			// rust는 기본적으로 UTF-8을 사용한다.
			// new()는 String의 함수이다, 자바의 static method와 유사
			io::stdin() //use를 사용하지 않으면 std::io::stdin() 와 같이 쓸수있다, 자바와 동일, 표준입력의 fd(handle)를 반환
					.read_line(&mut guess) // &는 참조(주소)를 가르키며 mut이 붙어 해당 주소의 메모리가 변경가능하다는 것을 얘기한다.
					.ok() // 오류가 발생해도 성공한 것으로 처리, 자바에서 try catch로 오류를 먹어버리는 것과 같음
					.expect("Failed to read line");//예외 처리용, read_line 함수가 실패시 ok함수가 expect함수를 실행(read_line에서 성공시에는 실행되지 않음)
			//let guess: u32 = guess.trim().parse() //앞에서 guess를 사용했지만, 러스트는 재정의가 가능하다.
			//		.ok()
			//		.expect("Please type a number!"); //parse는 문자열 to 숫자인데, guess가 문자만 있다면 ok가 expect를 호출할수있는 값으로 반환하게 된다.

			let guess: u32 = match guess.trim().parse() {
					Ok(num) => num,
					Err(_) => continue
			};

			println!("You guessed: {}", guess);

			match guess.cmp(&secret_number) {
					Ordering::Less    => println!("Too small!"),
							Ordering::Greater => println!("Too big!"),
							//Ordering::Equal   => println!("You win!")
							Ordering::Equal   => {println!("You win!"); break;}
			}
	}
}
