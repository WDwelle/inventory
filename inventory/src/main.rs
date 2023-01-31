//goals:
//have both backpack and hot-bar system
//be able to swap items in and out of slots
//possibly add different kind of items that correspond to certain slots ie:
//armor, weapon, throwable

fn main() {

    //item with stats(placeholder)
    struct Item {
        name:String,
        damage:i32,
        range:i32
    }


    let sword = Item{
        name:"sword".to_string(),
        damage:10,
        range:1
    };

    let bow = Item{
        name:"bow".to_string(),
        damage:7,
        range:5
    };

    let spear = Item{
        name:"spear".to_string(),
        damage:8,
        range:2
    };

    //hot-bar array that holds 6 items
    let hot_bar =  [
        sword, bow, spear
    ];


    for item in hot_bar {
        println!("the {} does {} damage ", item.name, item.damage);
    }



    struct Armor{
        name:String,
        health:i32
    }

    let helmet = Armor{
        name:"helmet".to_string(),
        health:25
    };

    let chest_piece = Armor{
        name:"chest piece".to_string(),
        health:50
    };

    let pants = Armor{
        name:"pants".to_string(),
        health:15
    };

    let armor_slots = [
        helmet, chest_piece, pants
    ];

    println!("your are wearing {}, {} and {}", helmet.name, chest_piece.name, pants.name);

    //backpack array that holds items not in the hot-bar
    let backpack: [i32; 20] = [0; 20];
}
