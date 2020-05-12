fn main() {
    const _PI:f32 = 3.1416;

    let mut x = 5;
    let y = 10;

    //calling a function with 1 parameter
    call(x);
    
    //devil in the details... mutable state... 
    x = 6;
    
    //if is an expression, however, ; defines a statement, no value returned a block
    if  x >= 6 {  //doesnt requires () conditional
        call(x);
    } else {
        call(y);
    }
    
    let m = sum(x,y);
    
    call(m);

    //if as an expression like in Scala (with love)
    let z = if x == y { x + y } else { m };

    call(z);

    loop_example();

    iterate_example();
}

fn call(x: i32) {
    println!("The value of x is: {}", x);
}

//function as an expression it doesnt require return and notice the missing ; statement call
fn sum(x: i32, y:i32) -> i32 {
    let z = {  //block expression
        let m = 2*x;
        m + 5 
    }; 

    x + y + z
}

fn loop_example() {
    let mut counter = 0; //here we go again with mutable variable...

    //interesting, loop as an expression, break can return a value XD
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

//Rust includes a foreach, however, it requires to call .iter()
//Scala for comprehension rules again
fn iterate_example() {
    let a = [10, 20, 30, 40, 50]; //remember array does not change size, for that use vector

    for element in a.iter() {
        call(*element);  //element must be explicit called with *, element is &i32 (a reference)
    }

    for i in (1..4).rev() { //it also has a Range from std lib
        call(i);
    }
}