A simple Rust script thats returns the data and tag from a block's tagged data payload.

The script is currently coded to retieve the data from a chronicle node but to modify it to work with a Hornet node.

Change:

.with_permanode(&node_url, Some(node_auth))?.finish().await?;

To:

.with_node(&node_url)?.finish().await?;
