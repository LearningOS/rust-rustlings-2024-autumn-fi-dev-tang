mod macro3_1 {
    #[macro_export]
    macro_rules! my_macro_3_1 {
        () => {
            println!("Checkout my_macro_3_1!");
        }
    }
}

fn main(){
    my_macro_3_1!();
}