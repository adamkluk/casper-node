use casper_engine_test_support::internal::LmdbWasmTestBuilder;
use casper_execution_engine::shared::stored_value::StoredValue;
use casper_types::{bytesrepr::ToBytes, Key};

use clap::{App, Arg};

mod auction_utils;
mod utils;

use crate::{
    auction_utils::{
        gen_snapshot, generate_entries_removing_bids, generate_entries_removing_withdraws,
    },
    utils::{hash_from_str, validators_diff},
};

/// Prints a global state update entry in a format ready for inclusion in a TOML file.
fn print_entry(key: &Key, value: &StoredValue) {
    println!("[[entries]]");
    println!("key = \"{}\"", key.to_formatted_string());
    println!("value = \"{}\"", base64::encode(value.to_bytes().unwrap()));
    println!();
}

fn main() {
    let matches = App::new("Global State Update Generator")
        .version("0.1")
        .about("Generates a global state update file based on the supplied parameters")
        .arg(
            Arg::with_name("data_dir")
                .short("d")
                .long("data-dir")
                .value_name("PATH")
                .help("Data storage directory containing the global state database file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hash")
                .short("h")
                .long("hash")
                .value_name("HASH")
                .help("The global state hash to be used as the base")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("validator")
                .short("v")
                .long("validator")
                .value_name("KEY,STAKE")
                .help("A new validator with their stake")
                .takes_value(true)
                .required(true)
                .multiple(true)
                .number_of_values(1),
        )
        .get_matches();

    let data_dir = matches.value_of("data_dir").unwrap_or(".");
    let state_hash = matches.value_of("hash").unwrap();
    let validators = match matches.values_of("validator") {
        None => vec![],
        Some(values) => values
            .map(|validator_def| {
                let mut fields = validator_def.split(',').map(str::to_owned);
                let field1 = fields.next().unwrap();
                let field2 = fields.next().unwrap();
                (field1, field2)
            })
            .collect(),
    };

    // Open the global state that should be in the supplied directory.
    let mut test_builder =
        LmdbWasmTestBuilder::open_raw(data_dir, Default::default(), hash_from_str(state_hash));

    // Read the old SeigniorageRecipientsSnapshot
    let old_snapshot = test_builder.get_seigniorage_recipients_snapshot();

    // Create a new snapshot based on the old one and the supplied validators.
    let new_snapshot = gen_snapshot(
        validators,
        *old_snapshot.keys().next().unwrap(),
        old_snapshot.len() as u64,
    );

    // Print the write to the snapshot key.
    for (era_id, validators) in &new_snapshot {
        print_entry(
            &Key::EraValidators(*era_id),
            &StoredValue::EraValidators(validators.clone()),
        );
    }

    let validators_diff = validators_diff(&old_snapshot, &new_snapshot);

    // Print the writes fixing the bids.
    for (key, value) in
        generate_entries_removing_bids(&mut test_builder, &validators_diff, &new_snapshot)
    {
        print_entry(&key, &value);
    }

    // Print the writes removing the no longer valid withdraws.
    for (key, value) in generate_entries_removing_withdraws(&mut test_builder, &validators_diff) {
        print_entry(&key, &value);
    }
}
