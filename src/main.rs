use std::env;
use bdk::Balance;
use bdk::wallet::{AddressIndex, AddressInfo};
use bdk::{bitcoin::Network, Wallet, database::MemoryDatabase};

fn main() -> anyhow::Result<()> {
    println!("Hello, wallet!");
    dotenv::from_filename(".env").unwrap();
    dotenv::dotenv().ok();

    let descriptor: String = env::var("WALLET_DESCRIPTOR")?;

    println!("descriptor: {:?}", descriptor);

    let wallet: Wallet<MemoryDatabase> = Wallet::new(
        &descriptor, 
        None,
        Network::Testnet,
        MemoryDatabase::default(),
    )?;

    let balance: Balance = wallet.get_balance()?;

    let adress: AddressInfo = wallet.get_address(AddressIndex::New)?;

    dbg!(balance);
    dbg!(adress);

    //print!("balance: {}", balance);
    //print!("Adres info: {}", adress);

    Ok(())
}

