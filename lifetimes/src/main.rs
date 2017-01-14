fn main() {
    println!("Hello, world!");
}

// implicit, 묵시적 수명 표시
fn foo(x: &i32) {
}

// explicit, 명시적 수명 표시
fn bar<'a>(x: &'a i32) { // `a는 수명 a라고 읽는다.
}
