use std::ops::Range;

fn condition(){
    let number = 3;

    if number < 5{
        println!("condition was true");
    }
    else{
        println!("condition was false");
    }
}

/*
fn is_there(){
    let number = 3;
    if number{
        println!("number is {number}"); //not works
    }
}
*/

fn if_let_statement(){
    let condition = true;
    let number = if condition {5} else {-5};
    println!("condition: {condition}, number: {number}");

    /*
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
    // this genereates error

            error[E0308]: `if` and `else` have incompatible types
        --> src/main.rs:4:44
        |
        4 |     let number = if condition { 5 } else { "six" };
        |                                 -          ^^^^^ expected integer, found `&str`
        |                                 |
        |                                 expected because of this
    */
}

fn looper(){
    let mut counter = 0;
    let result = loop{
        counter += 1;
        
        if counter == 10{
            break counter * 2;      //loop can have return value
        }
    };

    println!("result: {result}");
}

fn loop_and_label(){
    let mut count = 0;          //외부 루프를 추적

    'counting_up: loop{                 //'counting_up: 레이블이 지정된 외부 루프
        println!("count = {count}");
        let mut remaining = 10;

        loop{                                       //내부 루프
            println!("remaining = {remaining}");
            if remaining == 9{
                break;                              //내부 루프만 종료
            }
            if count == 2{
                break 'counting_up;                 //외부 루프 'counting_up 종료
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}

fn condition_loop_with_while(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop(){
    let li: Range<i32> = 1..4;
    println!("li: {:?}", li);
    let reversed = li.rev();
    println!("reversed: {:?}", reversed);


    for number in (1..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFF!!");
}

fn main(){
    condition();
    //isthere();
    if_let_statement();
    looper();
    loop_and_label();
    condition_loop_with_while();
    for_loop();
}