use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
	left: usize, //벡터 인덱스(e.g. Vec[index]) 타입
	right: usize
}

impl Philosopher {
    fn new(name: &str, left:usize, right:usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
			left: left,
			right: right
        }
    }

	fn eat(&self, table: &Table) {

        let _left = table.forks[self.left].lock().unwrap();//러스트에게 이값들은 사용되지 않을 것을 알려주어 warning 메시지를 없앰
        let _right = table.forks[self.right].lock().unwrap();

		println!("{} is eating.", self.name);

		thread::sleep_ms(1000);

		println!("{} is done eating.", self.name);
	}
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

	let philosophers = vec![
    	Philosopher::new("Judith Butler", 0, 1),
    	Philosopher::new("Gilles Deleuze", 1, 2),
    	Philosopher::new("Karl Marx", 2, 3),
    	Philosopher::new("Emma Goldman", 3, 4),
    	Philosopher::new("Michel Foucault", 0, 4)
	];

	let handles: Vec<_> = // _는 RUST에게 어떤 형인지 추론하라고 알려주는 것이다.
	 philosophers.into_iter() // into_iter()는 java iterrator + foreach와 같은 역할이다.
		.map(|p| {
			let table = table.clone(); // Arc<T>의 메서드로써 참조카운트를 올림
			thread::spawn( //spawn또한 인자로 클로저를 받는다.
				move || {p.eat(&table);} //move 키워드는 map이 수행하는 클로저가 갔고있는 값 p의 소유권을 가져온다.
			) // map 함수는 인자로 클로저를 받는다.
		}).collect();

	for h in handles {
		h.join().unwrap();
	}
}
