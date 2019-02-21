#![feature(try_from)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address, entry::Entry, dna::entry_types::Sharing, error::HolochainError, json::JsonString,hash::HashString,
};

// see https://developer.holochain.org/api/0.0.4/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Ingredient {
    ingredients: String
}

#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct Dish {
    dish: String,
    completed: bool

}

pub fn handle_add_dish(dish: Dish) -> ZomeApiResult<Address> {
   let list_entry = Entry::App(
        "dish".into(),
        dish.into()
    );

    // commit the entry and return the address
	hdk::commit_entry(&list_entry)
}

pub fn handle_add_ingredients(ingredients: Ingredient, ingredient_addr: HashString) -> ZomeApiResult<Address> {
    let ingredient_item_entry = Entry::App(
        "Ingredient".into(),
        ingredients.into()
    );

	let item_addr = hdk::commit_entry(&ingredient_item_entry)?; // commit the list item
	hdk::link_entries(&ingredient_addr, &item_addr, "dishes")?; // if successful, link to list address
	Ok(item_addr)
}



define_zome! {
    entries: [
       entry!(
            name: "Dish",
            description: "Food recipi app",
            sharing: Sharing::Public,
            native_type: Dish,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |dish: Dish, _ctx: hdk::ValidationData| {
                Ok(())
            },
            links: [
                to!(
                    "Ingredient",
                    tag: "dishes",
                    validation_package: || hdk::ValidationPackageDefinition::Entry,
                    validation: |base: Address, target: Address, _ctx: hdk::ValidationData| {
                        Ok(())
                    }
                )
            ]
        ),
        entry!(
            name: "Ingredient",
            description: "Name of the Dish",
            sharing: Sharing::Public,
            native_type: Ingredient,
            validation_package: || hdk::ValidationPackageDefinition::Entry,
            validation: |ingredient: Ingredient, _ctx: hdk::ValidationData| {
                Ok(())
            }
        )
    ]

    genesis: || { Ok(()) }

    functions: [
        add_dish: {
            inputs: |dish: Dish|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_dish
        }
        add_ingredients: {
            inputs: |ingredient: Ingredient, ingredient_addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_add_ingredients
        }
    ]

    traits: {
        hc_public [add_dish,add_ingredients]
    }
}
