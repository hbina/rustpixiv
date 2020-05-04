use pixiv::pixiv::client::Pixiv;
use pixiv::pixiv::illustration::illustration::IllustrationProxy;
use pixiv::pixiv::request_builder::PixivRequestBuilder;

use serde_json::Value;

#[test]
fn test_login() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");
}

#[test]
fn test_refresh_auth() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    pixiv
        .refresh_auth()
        .expect("Failed to refresh access token");
}

#[test]
fn test_bad_words() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::bad_words().build();
    let bad_words: Value = pixiv
        .execute(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", bad_words);
}

#[test]
fn test_work() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::work(66024340).build();
    let work: Value = pixiv
        .execute(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", work);
}

#[test]
fn test_user() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::user(6996493).build();
    let following_works: Value = pixiv
        .execute(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[test]
fn test_following_works() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::following_works()
        .image_sizes(&["large"])
        .include_sanity_level(false)
        .build();
    let following_works: Value = pixiv
        .execute(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[test]
fn test_fetching_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::illustration(75523989).build();

    let illustration = pixiv
        .execute(request)
        .expect("Request failed.")
        .json::<IllustrationProxy>()
        .expect("Failed to parse as json.")
        .into_inner();

    illustration.download(&pixiv.client, &std::env::current_dir().unwrap());
    println!("{:#?}", illustration);
}

#[test]
#[should_panic]
fn test_login_fail() {
    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    pixiv.login("", "").expect("Failed to log in.");
}
