mod person;
mod products;
use person::person_enum::PersonType;
use person::person_struct::Person;
use person::product_person::ProductPersonImpl;
use products::product_enums::*;
use products::product_struct::Product;
use products::{product_getters::Getters, product_management::Manage};

fn main() {
    // market place has been created
    // register new sellers
    let mut seller_1 = Person::new("kunle".to_owned(), PersonType::SELLER);
    let mut seller_2 = Person::new("gadus".to_owned(), PersonType::SELLER);
    let mut seller_3 = Person::new("john".to_owned(), PersonType::SELLER);

    // give summary for the three sellers
    println!("Seller 1 Summary: {}", seller_1.person_summary());
    println!("Seller 2 Summary: {}", seller_2.person_summary());
    println!("Seller 3 Summary: {}", seller_3.person_summary());

    // create new buyers
    let mut buyer_1 = Person::new("kolade".to_owned(), PersonType::BUYER);
    let mut buyer_2 = Person::new("blake".to_owned(), PersonType::BUYER);
    let mut buyer_3 = Person::new("kennedy".to_owned(), PersonType::BUYER);

    // give summary for the three buyers
    println!("Buyer 1 Summary: {}", buyer_1.person_summary());
    println!("Buyer 2 Summary: {}", buyer_2.person_summary());
    println!("Buyer 3 Summary: {}", buyer_3.person_summary());

    let seller_1_product_1 = Product::new_product(
        "Nike".to_owned(),
        "Beautiful Nike Shoes For Fahion And Jogging".to_owned(),
        400.0,
        ProductSize::LARGE,
        40,
        Category::SHOES,
        vec![
            "shoes".to_owned(),
            "loafers".to_owned(),
            "fahsion".to_owned(),
        ],
    );

    let seller_1_product_2 = Product::new_product(
        "Adidas".to_owned(),
        "Sports Running Shoes".to_owned(),
        350.0,
        ProductSize::MEDIUM,
        30,
        Category::SHOES,
        vec![
            "sports".to_owned(),
            "running".to_owned(),
            "shoes".to_owned(),
        ],
    );

    let seller_2_product_1 = Product::new_product(
        "Cotton T-Shirt".to_owned(),
        "Comfortable Cotton T-Shirt".to_owned(),
        25.0,
        ProductSize::MEDIUM,
        100,
        Category::CLOTHING,
        vec!["shirt".to_owned(), "cotton".to_owned(), "casual".to_owned()],
    );

    let seller_2_product_2 = Product::new_product(
        "Denim Jeans".to_owned(),
        "Classic Blue Denim Jeans".to_owned(),
        75.0,
        ProductSize::LARGE,
        50,
        Category::CLOTHING,
        vec!["jeans".to_owned(), "denim".to_owned(), "pants".to_owned()],
    );

    let seller_2_product_3 = Product::new_product(
        "Backpack".to_owned(),
        "Waterproof Hiking Backpack".to_owned(),
        120.0,
        ProductSize::LARGE,
        25,
        Category::ACCESSORIES,
        vec!["bag".to_owned(), "hiking".to_owned(), "outdoor".to_owned()],
    );

    let seller_2_product_4 = Product::new_product(
        "Watch".to_owned(),
        "Digital Sports Watch".to_owned(),
        200.0,
        ProductSize::SMALL,
        15,
        Category::ACCESSORIES,
        vec![
            "watch".to_owned(),
            "digital".to_owned(),
            "sports".to_owned(),
        ],
    );

    let seller_3_product_1 = Product::new_product(
        "Laptop Bag".to_owned(),
        "Professional Laptop Carrying Case".to_owned(),
        80.0,
        ProductSize::MEDIUM,
        45,
        Category::ACCESSORIES,
        vec![
            "laptop".to_owned(),
            "bag".to_owned(),
            "professional".to_owned(),
        ],
    );

    let seller_3_product_2 = Product::new_product(
        "Sunglasses".to_owned(),
        "UV Protection Sunglasses".to_owned(),
        150.0,
        ProductSize::SMALL,
        60,
        Category::ACCESSORIES,
        vec![
            "sunglasses".to_owned(),
            "fashion".to_owned(),
            "UV".to_owned(),
        ],
    );

    let seller_3_product_3 = Product::new_product(
        "Winter Jacket".to_owned(),
        "Warm Winter Coat with Fur Hood".to_owned(),
        250.0,
        ProductSize::LARGE,
        35,
        Category::CLOTHING,
        vec!["jacket".to_owned(), "winter".to_owned(), "warm".to_owned()],
    );
}
