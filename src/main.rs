use serde::Deserialize;
use serde_json::{self, Value};
use std::io::stdin;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    cols: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Data {
    List(Vec<Value>),
    Other(Value),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let data: Data = serde_json::from_reader(stdin().lock())?;

    match data {
        Data::List(d) => {
            for units in d {
                for col in &opt.cols {
                    print_col(col, &units);
                }
                println!();
            }
        }
        Data::Other(units) => {
            for col in &opt.cols {
                print_col(col, &units);
            }
            println!();
        }
    }

    Ok(())
}

// column is the key of the object in data that you want to print.
// However, column may actually be a composite of keys separated by periods.
// Keys can be either usizes or strings. To account for this, we try to parse the key as a usize.
// If successful, it still could be either a usize or a string. So we treat it as a usize. If that
// fails, we then try as a string. And, naturally, if we could not parse it as a usize, we treat it
// as a string.
//
// NOTE: there's no escaping of periods, so if you're trying to print a column
// that involves a string key with a period in it, it won't work.
fn print_col(column: &str, mut data: &Value) {
    for key in column.split('.') {
        if let Ok(key_num) = key.parse::<usize>() {
            if let Some(d) = data.get(key_num) {
                data = d;
            } else {
                data = &data[key];
            }
        } else {
            data = &data[key];
        }
    }
    print!("{}\t", data.to_string());
}
