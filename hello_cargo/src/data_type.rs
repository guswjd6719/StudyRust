fn another_function(){
    // 연산

    // addition
    let sum = 5 + 10;
	println!("{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
	println!("{difference}");

    // multiplication
    let product = 4 * 30;
	println!("{product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
	println!("{quotient}");
	println!("{truncated}");

    // remainder
    let remainder = 43 % 5;
	println!("{remainder}");

    // Boolean
    let _f: bool = false; // with explicit type annotation

    //Unicode 사용
    let heart_eyed_cat = '😻';
	println!("{heart_eyed_cat}");

    //tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of 0 is: {five_hundred}");
    println!("The value of 1 is: {six_point_four}");
    println!("The value of 2 is: {one}");

    //Array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("Array a[0]: {first}");

    let a = [3; 5]; // let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    let third = a[2];
    println!("Array a[0]: {first}");
    println!("Array a[1]: {second}");
    println!("Array a[2]: {third}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

}