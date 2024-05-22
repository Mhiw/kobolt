use kobolt::application::{
    Application,
    RunOption,
};

fn main() {
    Application::new()
        .execute(start, RunOption::Init)
    ;
}

fn start() {
    println!("Hello World!");
}
