use std::fmt::Debug;
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

// 기본형 타입에 trait를 이용해 메소를 추가했다.
// 좋은 방법은 아니다.
impl HasArea for i32 {
    fn area(&self) -> f64 {
        println!("this is silly");

        *self as f64
    }
}


//K는 clone과  debug trait를 구현한 type이여야 한다.
fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}



//타입에 트레이트가 늘어날 수록 함수읽기가 불편하므로 where절을 이용한다.
fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

trait Foo {//trait는 java의 interface와 같다.
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { !self.is_valid() } // JAVA8의 디폴트메스드랑 같은 개념이다.
}

trait Foo {
    fn foo(&self);
}

trait FooBar : Foo { // 트레이트  FooBar는 Foo를 상속한다. 따라서 FooBar를 구현하려면 Foo또한 구현해야 한다.
    fn foobar(&self);
}




fn main() {
    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);

	5.area();
}
