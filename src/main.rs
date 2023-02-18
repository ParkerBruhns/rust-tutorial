#[allow(unused)]
// use rusqlite::{Connection, Result};

static mut ID_NUM: i64 = 1;

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    password: String,
    id: i64,
}

impl User {

    unsafe fn new_user(first_name: String, last_name: String, password: String) -> User {
        let id = ID_NUM;
        let person = User {
            first_name: first_name, last_name: last_name, password: password, id: id,
        };
        ID_NUM = ID_NUM + 1;
        person
    }
}


fn main() {
    // let conn = Connection::open_in_memory()?;

    // conn.execute(
    // 	"CREATE TABLE User {
    // first_name TEXT NOT NULL,
    // last_name TEXT NOT NULL,
    // password TEXT NUT NULL,
    // 	id INTEGER PRIMARY KEY
    // )",
    // 	(),
    // )?;

    // let mut stmt = conn.prepare("SELECT first_name, last_name, password, id")?;
    // let User_iter = stmt.query_map([], |row| {
    // 	Ok( User {
    // 		first_name: row.get(0)?,
    // 		last_name: row.get(1)?,
    // 		password: row.get(2)?,
    // 		id: row.get(3)?,
    // 	})
    // })?;

    // for User in User_iter {
    // 	println!("Found person {:?}", User.unwrap());
    // }
    // Ok(())

    create_user();
    create_user();
}

fn find_password() -> String {
    loop {
        let mut password = String::new();
        let mut temp_password = String::new();

        println!("Enter your password.");
        std::io::stdin().read_line(&mut password).unwrap();
        password = password.trim().to_string();

        println!("Please confirm your password");
        std::io::stdin().read_line(&mut temp_password).unwrap();
        temp_password = temp_password.trim().to_string();

        if password.eq(&temp_password) {
            return password;
        } else {
            println!("Passwords do not match.");
        }
    }
}

fn create_user() {
    std::process::Command::new("cls"). status(). unwrap();

    let mut first_name = String::new();
    let mut last_name = String::new();

    println!("Enter your first name.");
    std::io::stdin().read_line(&mut first_name).unwrap();
    first_name = first_name.trim().to_string();

    println!("Enter your last name.");
    std::io::stdin().read_line(&mut last_name).unwrap();
    last_name = last_name.trim().to_string();

    let password = find_password();

    unsafe {
        let user_1 = User::new_user(first_name, last_name, password);

        println!("Hello {} {}! Your password is {}. Your User ID is {}",user_1.first_name, user_1.last_name, user_1.password, user_1.id);
    }
}