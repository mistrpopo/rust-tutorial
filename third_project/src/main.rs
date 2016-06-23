fn  foo(x: u32) -> u32 {
	let y: u32 = 3;
	let b: bool = x > 5;
	let z = if b { y } else { y - 1 };
	x*z
}

fn _5timesloop() { //can't start with a number :)
    let mut i = 1;
    let mut j = 0;
    loop {
    	println!("loop: {}", i);
    	i+=1;
    	j+=1;
    	if j >= 5 { break; }
    }	
    for i in 0..5 {
    	println!("for loop: {}", i+1)
    }
    for (i,j) in (1..6).enumerate() {
    	println!("enumerate: {} {}", i, j)
    }
    let one2five = "1 2 3 4 5".split_whitespace();
    for x in one2five {
    	println!("split a string: {}", x);
    }
}

fn ownership_tests() {
	/*
	//Vec<T>: moved by default type
	let v = vec![1, 2, 3]; 						//create v
	let u = v; 									//u takes ownership of v
    println!("test single move: {}", v[0]);		//main.rs:34:26: 34:27 error: use of moved value: `v` [E0382]
    */

    //i32 has copy traits and this works :
	let i = 120;
	let j = i; 									//full copy
    println!("test copy integer: {}", i);		//OK

	let v = vec![1, 2, 3]; 						//create v
	let u = v; 									//u takes ownership of v
	let v = u; 									//v takes ownership of u
    println!("test double move: {}", v[0]);		//OK
}


fn main() {
    println!("Hello, world! { }", foo(6));
    _5timesloop();
    ownership_tests();
}
