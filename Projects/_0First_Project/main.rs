fn main(){
    println!("Hellow World, I am using Rust");
    println!("I am Niman");

    let  world ="World is yours";
    println!("{}", world);

    let sumamry = "\nSummary of this project \n
    ===Create a new project without using cargo===\n
    1. nano <main_filenam>.rs
        Example : nano main.rs

    2. Compile the code
        rustc main.rs

    3. Run the project
        .\\main.exe\n";
    println!("{}", sumamry);
}