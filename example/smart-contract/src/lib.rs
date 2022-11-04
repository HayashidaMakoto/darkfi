use darkfi_sdk::{
    db::{db_begin_tx, db_end_tx, db_get, db_init, db_lookup, db_set},
    entrypoint,
    error::ContractResult,
    initialize, msg,
    state::set_update,
    tx::Transaction,
    update_state,
};
use darkfi_serial::{deserialize, serialize, SerialDecodable, SerialEncodable};

/// Available functions for this contract.
/// We identify them with the first byte passed in through the payload.
#[repr(u8)]
pub enum Function {
    Foo = 0x00,
    Bar = 0x01,
}

impl From<u8> for Function {
    fn from(b: u8) -> Self {
        match b {
            0x00 => Self::Foo,
            0x01 => Self::Bar,
            _ => panic!("Invalid function ID: {:#04x?}", b),
        }
    }
}

// An example of deserializing the payload into a struct
#[derive(SerialEncodable, SerialDecodable)]
pub struct FooCallData {
    pub a: u64,
    pub b: u64,
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct BarArgs {
    pub x: u32,
}

#[derive(SerialEncodable, SerialDecodable)]
pub struct FooUpdate {
    pub name: String,
    pub age: u32,
}

initialize!(init_contract);
fn init_contract(_ix: &[u8]) -> ContractResult {
    msg!("wakeup wagies!");
    db_init("wagies")?;

    // Lets write a value in there
    let tx_handle = db_begin_tx()?;
    db_set(tx_handle, "jason_gulag".as_bytes(), serialize(&110))?;
    let db_handle = db_lookup("wagies")?;
    db_end_tx(db_handle, tx_handle)?;

    // Host will clear delete the batches array after calling this func.

    Ok(())
}

// This is the main entrypoint function where the payload is fed.
// Through here, you can branch out into different functions inside
// this library.
entrypoint!(process_instruction);
fn process_instruction(ix: &[u8]) -> ContractResult {
    match Function::from(ix[0]) {
        Function::Foo => {
            let tx_data = &ix[1..];
            // ...
            let (func_call_index, tx): (u32, Transaction) = deserialize(tx_data)?;
            let call_data: FooCallData =
                deserialize(&tx.func_calls[func_call_index as usize].call_data)?;
            msg!("call_data {{ a: {}, b: {} }}", call_data.a, call_data.b);
            // ...
            let update = FooUpdate { name: "john_doe".to_string(), age: 110 };

            let mut update_data = vec![Function::Foo as u8];
            update_data.extend_from_slice(&serialize(&update));
            set_update(&update_data)?;
            msg!("update is set!");

            // Example: try to get a value from the db
            let db_handle = db_lookup("wagies")?;
            // FIXME: this is just empty right now
            let age_data = db_get(db_handle, "jason_gulag".as_bytes())?;
            msg!("wagie age data: {:?}", age_data);
        }
        Function::Bar => {
            let tx_data = &ix[1..];
            // ...
            let args: BarArgs = deserialize(tx_data)?;
        }
    }
    /*
    msg!("Hello from the VM runtime!");
    // Deserialize the payload into `Args`.
    let args: Args = deserialize(ix)?;
    msg!("deserializing payload worked");

    if args.a < args.b {
        // Returning custom errors
        return Err(ContractError::Custom(69))
    }

    let sum = args.a + args.b;
    // Publicly logged messages
    msg!("Hello from the VM runtime!");
    msg!("Sum: {:?}", sum);

    // Querying of ledger state available from the VM host
    let nf = Nullifier::from(pallas::Base::from(0x10));
    msg!("Contract Nullifier: {:?}", nf);

    if nullifier_exists(&nf)? {
        msg!("Nullifier exists");
    } else {
        msg!("Nullifier doesn't exist");
    }
    */

    Ok(())
}

update_state!(process_update);
fn process_update(update_data: &[u8]) -> ContractResult {
    msg!("Make update!");

    match Function::from(update_data[0]) {
        Function::Foo => {
            let update: FooUpdate = deserialize(&update_data[1..])?;

            // Write the wagie to the db
            let tx_handle = db_begin_tx()?;
            db_set(tx_handle, update.name.as_bytes(), serialize(&update.age))?;
            let db_handle = db_lookup("wagies")?;
            db_end_tx(db_handle, tx_handle)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}

//fn state_transition() -> Result<StateUpdate> {
//    // read only
//}
//
//fn apply(update) {
//    // writes happen here
//}
