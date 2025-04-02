#[macro_use] extern crate rocket;


#[get("/")]
fn index()-> &'static str {
    "Hello Rocket"
}

#[get("/custom")]
fn custom_index()-> &'static str {
    "Hello Server"
}

#[get("/name/<name>")]
fn print_name(name:&str)-> String{
    let mut hello = String::from("Hello ");
    hello.push_str(name);
    hello
}

#[get("/address/<address>")]
fn print_address(address:&str)-> String{
    let mut hello = String::from("Your address is :  ");
    hello.push_str(address);
    hello
}

#[rocket::main]
async  fn main() -> Result<(),rocket::Error>{
    let _rocket = rocket::build().mount("/", routes![index,custom_index,print_name,print_address])
    .launch()
    .await?;
    Ok(())
}