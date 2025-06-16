mod person;
mod products;
use person::person_enum::PersonType;
use person::person_struct::Person;
use person::product_person::ProductPersonImpl;
use products::product_enums::*;
use products::product_struct::Product;
use products::{product_getters::Getters, product_management::Manage};

fn main() {
    println!("Welcome to the Market Place! \n---We Starting--- ");
    // market place has been created
    // register new sellers
    let mut seller_1 = Person::new("kunle".to_owned(), PersonType::SELLER);
    let mut seller_2 = Person::new("gadus".to_owned(), PersonType::SELLER);
    let mut seller_3 = Person::new("john".to_owned(), PersonType::SELLER);

    // give summary for the three sellers
    println!("Seller 1 Summary: {} \n", seller_1.person_summary());
    println!("Seller 2 Summary: {} \n", seller_2.person_summary());
    println!("Seller 3 Summary: {} \n", seller_3.person_summary());

    // create new buyers
    let mut buyer_1 = Person::new("kolade".to_owned(), PersonType::BUYER);
    let mut buyer_2 = Person::new("blake".to_owned(), PersonType::BUYER);
    let mut buyer_3 = Person::new("kennedy".to_owned(), PersonType::BUYER);

    // give summary for the three buyers
    println!("Buyer 1 Summary: {} \n", buyer_1.person_summary());
    println!("Buyer 2 Summary: {} \n", buyer_2.person_summary());
    println!("Buyer 3 Summary: {} \n", buyer_3.person_summary());

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

    // create products for sellers
    seller_1.create_product(&seller_1_product_1).unwrap();
    seller_1.create_product(&seller_1_product_2).unwrap();
    seller_2.create_product(&seller_2_product_1).unwrap();
    seller_2.create_product(&seller_2_product_2).unwrap();
    seller_2.create_product(&seller_2_product_3).unwrap();
    seller_2.create_product(&seller_2_product_4).unwrap();
    seller_3.create_product(&seller_3_product_1).unwrap();
    seller_3.create_product(&seller_3_product_2).unwrap();
    seller_3.create_product(&seller_3_product_3).unwrap();

    // view products for sellers
    println!("Seller 1 Products: {:#?} \n", seller_1.view_products());
    println!("Seller 2 Products: {:#?} \n", seller_2.view_products());
    println!("Seller 3 Products: {:#?} \n", seller_3.view_products());

    // buyer 1 buys a product from seller 1
    buyer_1
        .buy_a_product(&mut seller_1, seller_1_product_1.clone())
        .unwrap();    

    // buyer 2 buys a product from seller 2
    buyer_2
        .buy_a_product(&mut seller_2, seller_2_product_1.clone())
        .unwrap();

    // buyer 3 buys a product from seller 3
    buyer_3
        .buy_a_product(&mut seller_3, seller_3_product_1.clone())
        .unwrap();

    // view products for sellers after purchase
    println!("Seller 1 Products After Purchase: {:#?} \n", seller_1.view_products());
    println!("Seller 2 Products After Purchase: {:#?} \n", seller_2.view_products());
    println!("Seller 3 Products After Purchase: {:#?} \n", seller_3.view_products());

    // view products for buyers after purchase
    println!("Buyer 1 Buys After Purchase: {:#?} \n", buyer_1.get_bought_products());
    println!("Buyer 2 Buys After Purchase: {:#?} \n", buyer_2.get_bought_products());
    println!("Buyer 3 Buys After Purchase: {:#?} \n", buyer_3.get_bought_products());

    // a seller can update a product 
    let updated_product = Product::new_product(
        "Nike Air Max".to_owned(),
        "Stylish Nike Air Max Shoes".to_owned(),
        500.0,
        ProductSize::LARGE,
        20,
        Category::SHOES,
        vec![
            "shoes".to_owned(),
            "fashion".to_owned(),
            "air max".to_owned(),
        ],
    );

    seller_1
        .update_product(seller_1_product_1.product_id, updated_product.clone())
        .unwrap();

    // view products by id after update

    let updated_product = seller_1
        .get_product_by_id(seller_1_product_1.product_id)
        .unwrap();

    println!("Updated Product: {:#?} \n", updated_product);


    // a seller can delete a product
    seller_1
        .delete_product(seller_1_product_2.product_id)
        .unwrap();

    // view products for sellers after delete
    println!("Seller 1 Products After Delete: {:#?} \n", seller_1.view_products());

    // product changes
    let mut product = Product::new_product(
        "Nike Air Max".to_owned(),
        "Stylish Nike Air Max Shoes".to_owned(),
        500.0,
        ProductSize::LARGE,
        20,
        Category::SHOES,
        vec![
            "shoes".to_owned(),
            "fashion".to_owned(),
            "air max".to_owned(),
        ],
    );

    //  increase quantity of that product
    product.increase_quantity(10).unwrap();

    // check it has increased
    println!("Product Quantity After Increase: {:#?} \n", product.get_amount());

    // check weather the product is in stock
    println!("Product Status Weather in struct {}", product.is_in_stock());

    // apply discount on the product
    product.apply_discount(50.0).unwrap();

    

}
