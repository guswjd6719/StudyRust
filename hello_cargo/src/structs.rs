struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username,
        // email: email,        필드를 반복할 필요 없이 간단하게 init 
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("test123"),
        email: String::from("test123@example.com"),
        sign_in_count: 1,
    };

    //mutable 로 만들면 수정 가능
    user1.email = String::from("anotheremail@example.com");
    println!("user1 email : {}", user1.email);

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("user2 email : {}", user2.email);
    println!("user2 sign_in_count : {}", user2.sign_in_count);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user3 email : {}", user3.email);
    println!("user3 username : {}", user3.username);

    // println!("user1 username : {}", user1.username); user3으로 옮겨갔기 때문에 사용할 수 없음
    println!("user1 email : {}", user1.email);  // email은 옮겨가지 않아서 사용할 수 있음

}   
