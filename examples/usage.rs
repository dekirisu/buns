// Same as:
// const TEST: u32 = 10;
// const OMEGA: u32 = 59;
buns::sandwich!{ 
    const ^0: u32 = ^1; // Buns
    #TEST^10 #OMEGA^59  // Toppings
}

buns::prepare!{testy
    let a = ^0 + ^0; 
    println!("{a}");
}

fn main(){
    testy!{#1 #TEST #4+4 #OMEGA #2*2}
}

