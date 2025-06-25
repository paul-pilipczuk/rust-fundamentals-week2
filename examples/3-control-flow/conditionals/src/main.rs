fn main() {
    let proceed = false;
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let age = 22;
    if age > 55 {
        println!("OLD!");
    } else if age > 20 { 
        println!("Not old");
    } else {
        println!("Child")
    }


    let height = 173;
    if height > 180 {
        println!("Tall");
    } else if height > 170{
        // this a check using nested if-else statements
        if age > 20{
            println!("double average");
        }else{
            println!("Average");
        }
        
    } else {
        println!("Short");
    }

}
