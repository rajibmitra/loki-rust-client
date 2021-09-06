
// rust cient for grafana loki 
//
// This client is a wrapper around the loki API.

use serde_json::{Result, Value};

fn untyped_stream() -> Result<()> {
    // Some JSON input data as a &str.
    let data = r#"
    {
        'streams': [
            {
                'labels': '{source=\"Name-of-your-source\",job=\"name-of-your-job\", host=\"' + host + '\"}',
                'entries': [
                    {
                        'ts': curr_datetime,
                        'line': '[WARN] ' + msg
                    }
                ]
            }
        ]
    }"#;

    // Parse the string of data into a Value.
    let j: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("{}", j["streams"][0]["labels"]);

    Ok(())
}

let client = reqwest::Client::new();

let payload = r#"{"streams": [{"labels": "source=\"Name-of-your-source\",job=\"name-of-your-job\", host=\"' + host + '\"", "entries": [{"ts": curr_datetime, "line": "[WARN] " + msg}]}]}"#;
let url = "http://localhost:3100/loki/api/v1/push";
let res = client.post(url).body(payload).send();


// let payload = serde_json::to_string(&map).unwrap();
