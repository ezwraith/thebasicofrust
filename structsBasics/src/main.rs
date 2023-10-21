struct User {
    username:String,
    // email:String,
    // sign_in_count:u64,
    // active:bool,
}

fn main() {
    let mut user1 = User{
        username: String::from("Evan Ralph"),
        // email: String::from("evanvale.seo@gmail.com"),
        // sign_in_count:1,
        // active:true,
    };
   print(&mut user1);
   println!("{}",user1.username );
}

fn print( name: &mut User){
    println!("{}",&name.username);
    name.username = String::from("changed");
    println!("{}",&name.username);
}