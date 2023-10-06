// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use postgres::{Client, NoTls, Error};
use glob::glob;


fn upload_data() {
    struct GroupStruct {
        name: String,
        hidden: bool
    }
    
    struct SimulationStruct {
        title: String,
        orbitPeriod: f64
    }
    
    struct SummaryGroupStruct {
        description: String,
        groupId: String,
        maxLimit: f64,
        minLimit: f64
    }
    
    struct TestGroupStruct {
        name: String,
        maximums: Vec<f64>,
        minimums: Vec<f64>,
        timestamps: Vec<f64>,
        maximum: f64,
        minimum: f64
    }
    
    struct ThermostatStruct {
        time: String,
        id: String,
        onOff: String,
        tempSensor: String,
        power: String,
        numbers: String,
        totalTimeOn: String,
        totalEnergyUsed: String
    }
    
    struct TestCasesStruct {
        ownerId: String,
        identifier: String,
        groups: Vec<TestGroupStruct>,
        thermostats: Vec<ThermostatStruct>
    }
    
    const MAJOR_VERSION: i32 = 1;
    const MINOR_VERSION: i32 = 4;
    const BUILD_VERSION: i32 = 1;

    println!("helios v{0}.{1}.{2}", MAJOR_VERSION, MINOR_VERSION, BUILD_VERSION);

    // Stores the name and hidden boolean for every group in the REPF files.
    let mut groups: Vec<(String,u32)> = vec![];
    // Chnage this to the location of the 'TTR files v3' folder.
    let mut path= String::from("C:\\Users\\Dugua\\TRR files v3");

    // Glob recursively finds each file that maches the given pattern.
    for entry in glob(&(path + "\\**\\REPF")).expect("Failed to read glob pattern.") {
        match entry {
            Ok(PathBuf) => println!("Success!"),
            Err(e) =>  println!("{:?}", e),
        }

    }

    


}



fn create_table() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:ilovespace@localhost/heliosDB", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
        )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
        )
    ")?;

    Ok(())

}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}