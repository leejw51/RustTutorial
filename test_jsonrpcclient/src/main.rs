use jsonrpc::client::Client ;
use jsonrpc::Request;
use serde_json::{json, Value};

fn main() {
    let client = Client::new("http://localhost:26657".to_owned(), None, None);
    let name="status";
    let params: Vec<Value>= vec![];
    let request = client.build_request(name, &params);
    println!("{:?}", request);
    let response = client.send_request(&request).unwrap();    
    println!("{:?}", response.result);

}