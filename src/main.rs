mod products;
use products::product_enums::*;
use products::{product_getters::Getters, product_management::Manage};
use products::product_struct::Product;
fn main() {
    let mut new_shoe = Product::new_product(
        "Shoes".to_owned(),
        "comfortable jogger shoes".to_owned(),
        230.0,
        ProductSize::LARGE,
        20,
        Category::SHOES,
        vec!["1".to_owned()],
    );

    let mut product_details = new_shoe.get_product_details();
    println!("{product_details}");
    let app_disc = new_shoe.apply_discount(50.0).unwrap();
    product_details = new_shoe.get_product_details();
    println!(" aleter pd {product_details}");
    
}
