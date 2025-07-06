use proc_macro::TokenStream;

#[proc_macro]
pub fn my_func_macro(args: TokenStream) -> TokenStream {
    println!("Input TokenStream is:");
    for tt in args {
        println!(" {tt:?}");
    }
    TokenStream::new()
}

fn main() {
    println!("Hello, world!");
}
