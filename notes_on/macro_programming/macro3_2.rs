#[macro_use]
mod macro3_2 {
    macro_rules! my_macro_3_2 {
        () => {
            println!("Checkout my_macro_3_2!");
        }
    }
}

fn main(){
    my_macro_3_2!();
}