struct Firework {
    strength: i32,
}

impl Drop for Firework {
    fn drop(&mut self) {
        println!("BOOM times {}!!!", self.strength);
    }
}

fn main() {
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}//스크포가 끝나는 시점에 drop이 실행된다, 따라서 struct와 관련된 자원을 청소하는 역할로 많이 사용, like java's finalize

