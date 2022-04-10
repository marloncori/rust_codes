use into_iter::online_shop;

fn main() {
    let shoes = vec![
        online_shop::Shoe {
            size: 42,
            style: String::from("sneakers"),
        },
        online_shop::Shoe {
            size: 35,
            style: String::from("sandals"),
        },
        online_shop::Shoe {
            size: 39,
            style: String::from("slippers"),
        },
        online_shop::Shoe {
            size: 39,
            style: String::from("trackers"),
        },
        online_shop::Shoe {
            size: 42,
            style: String::from("boots"),
        },
        online_shop::Shoe {
            size: 39,
            style: String::from("high-heels"),
        },
        online_shop::Shoe {
            size: 39,
            style: String::from("elegant shoes"),
        },
    ];

    let shoes2 = vec![
        online_shop::Shoe {
            size: 41,
            style: String::from("sneakers"),
        },
        online_shop::Shoe {
            size: 40,
            style: String::from("sandals"),
        },
        online_shop::Shoe {
            size: 40,
            style: String::from("slippers"),
        },
        online_shop::Shoe {
            size: 37,
            style: String::from("trackers"),
        },
        online_shop::Shoe {
            size: 41,
            style: String::from("boots"),
        },
        online_shop::Shoe {
            size: 40,
            style: String::from("high-heels"),
        },
        online_shop::Shoe {
            size: 39,
            style: String::from("elegant shoes"),
        },
    ];
    let age1 = 36; let age2 = 55;
    let male_size = 42; let female_size = 40;

    let male_client = online_shop::Customer::new(
        age1, "Marlon".to_string(), male_size);
    let female_client = online_shop::Customer::new(
        age2, "Laura".to_string(), female_size);

    let marlon_shoes = male_client.buy_shoes(shoes)
                            .unwrap_or(Vec::new());
    let laura_shoes = female_client.buy_shoes(shoes2)
                            .unwrap_or_default();

    println!("\n First customer information: 
        \n\t Name: {} \n\t Age: {} \n\t Shoe size: {}\n",
        &male_client.name, male_client.age, male_client.shoe_size);

    println!(" ==> {} has bought the following shoes today: \n\t{:?}.",
        male_client.name, marlon_shoes);

    println!("\n Second customer information: 
        \n\t Name: {} \n\t Age: {} \n\t Shoe size: {}\n",
        &female_client.name, female_client.age, female_client.shoe_size);

    println!(" ==> And {} has bought the following shoes today: \n\t{:?}.",
        female_client.name, laura_shoes);

}
