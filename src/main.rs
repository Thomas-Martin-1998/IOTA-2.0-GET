
use iota_sdk::{client::{Client, Result}, types::block::payload::{Payload},};
use serde_json::{json};

#[tokio::main]
async fn main() -> Result<()> {
 
    // Take the node URL from command line argument.
    let node_url = std::env::args()
        .nth(1).unwrap();

    let node_auth = iota_sdk::client::node_manager::node::NodeAuth {
        jwt: Some("jwt".to_string()),
        basic_auth_name_pwd: None,
    };

    // Create a node client.
    let client = Client::builder().with_permanode(&node_url, Some(node_auth))?.finish().await?;

    // Take the block ID from command line argument or...
    let block_id = if let Some(Ok(block_id)) = std::env::args().nth(2).map(|s| s.parse()) {
        block_id
    } else {
        // ... fetch one from the node.
        client.get_tips().await?[0]
    };

    // Get the block.
    let block = client.get_block(&block_id).await?;

    let payload = block.payload().unwrap();

    if let Payload::TaggedData(tagged_data_payload) = payload {

        let data_bytes = tagged_data_payload.data();

        let data = String::from_utf8(data_bytes.to_vec()).expect("Found invalid UTF-8");

        let tag_bytes = tagged_data_payload.tag();

        let tag = String::from_utf8(tag_bytes.to_vec()).expect("Found invalid UTF-8");

        let json_object = json!({
            "tag": tag,
            "data": data
        });
    
        // Convert the JSON object to a String
        let json_string = json_object.to_string();

        println!("{}", json_string);
    }

    Ok(())
}