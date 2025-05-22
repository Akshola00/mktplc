#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ProductSize {
    SMALL,
    MEDIUM,
    LARGE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    CLOTHING,
    EDIBLE,
    SHOES,
    AUTOMOBILE,
    OTHERS,
    ACCESSORIES,
}
