///////////////////////////////////
//Game Player Example
///////////////////////////////////
#[derive(Debug)]
struct Item {
    item_id: i32,
    item_name: String,
    is_consumable: bool,
}

#[derive(Debug)]
struct Player {
    unique_id: i32,
    health: i32,
    armor: i32,
    inventory: Vec<Item>
}

fn show_player () {
    let item1 = Item {
        item_id: 1,
        item_name: String::from("Medkit"),
        is_consumable: true,
    };

    let item2 = Item {
        item_id: 1,
        item_name: String::from("Armor Pack"),
        is_consumable: true,
    };

    let player = Player {
        unique_id: 99,
        health: 100,
        armor: 50,
        inventory: vec![item1, item2]
    };

    println!("Player: {:?}", player);
}

///////////////////////////////////
///////////////////////////////////
///////////////////////////////////

///////////////////////////////////
//User Struct Example
///////////////////////////////////
#[derive(Debug)]
struct User {
    id: u128,
    name: String,
    meta: Option<String>,
    email: String,
    password: String,
}

fn build_user(id: u128, name: String, meta: Option<String>, email: String, password: String) -> User {
    User {
        id,
        name,
        meta,
        email,
        password
    }
}

fn show_user() {
    let user = User {
        id: 5,
        name: String::from("Batuhan"),
        meta: None,
        email: String::from("batuhan@test.com"),
        password: String::from("5BQobofyOBfsskw")
    };

    let user2 = User {
        id: 6,
        name: String::from("Batuhan2"),
        meta: Some(String::from("Exists")),
        email: String::from("batuhan2@test.com"),
        password: String::from("5BQoxoayOsBfsgw")
    };

    let user3 = build_user(7, String::from("Test"), Some(String::from("Meta")), String::from("testemail@gmail.com"), String::from("Password"));
    println!("User: {:?}", user);
    println!("User-2: {:?}", user2);
    println!("User-3: {:?}", user3);

    // Value Borrowed Here
    let users = vec![user, user2, user3];
    println!("users: {:?}", users);
    println!("first user test: {:?}", users[0]);

}
///////////////////////////////////
///////////////////////////////////
///////////////////////////////////

///////////////////////////////////
// Area Math Calculation Example
///////////////////////////////////
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn show_area() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
///////////////////////////////////
///////////////////////////////////
///////////////////////////////////
fn main() {
    show_area();
    show_user();
    show_player();
}

