// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to crate different flavors of drinks
// * User a struce to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounce
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Fruity => println!("fruity"),
        Flavor::Sweet => println!("sweet"),
        Flavor::Sparkling => println!("sparkling"),
    }

    println!("oz: {:?}", drink.fluid_oz);
}



// structs demo
struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let cereal = GroceryItem{
        stock: 10,
        price: 4.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);

    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 5.5
    };

    print_drink(sweet);

    let sparkling = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 5.5
    };

    print_drink(sparkling);

    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 5.5
    };

    print_drink(fruity);
    
}
