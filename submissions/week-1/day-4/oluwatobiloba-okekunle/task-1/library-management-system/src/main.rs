enum ItemType {
    Book,
    Magazine,
    Fiction,
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}


fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    // Create a new library item
    let book = LibraryItem {
        quantity: 5,
        id: 1001,
        item_type: ItemType::Book,
    };

    // Display information about the book
    println!("Library Item Information:");
    println!("------------------------");
    display_quantity(&book);
    display_id(&book);
    display_type(&book);
}