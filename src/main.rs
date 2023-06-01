#[macro_use] extern crate rocket;

use bdk::Balance;
use bdk::wallet::{AddressIndex, AddressInfo};
use bdk::{bitcoin::Network, Wallet, database::MemoryDatabase};

#[get("/")]
fn index() -> String {
    //Wallet
    ////la variable "descriptor" es la misma que hay en el ejemplo en https://github.com/bitcoindevkit/bdk/tree/master/crates/bdk
    let descriptor: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    println!("descriptor: {:?}", descriptor);

    let wallet: Wallet<MemoryDatabase> = Wallet::new(
        descriptor, 
         None,
        Network::Testnet,
        MemoryDatabase::default(),
    ).unwrap();

    let balance: Balance = wallet.get_balance().unwrap();
    let address: AddressInfo = wallet.get_address(AddressIndex::New).unwrap();

    format!("Balance: {:?}\nAddress: {:?}", balance, address)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}