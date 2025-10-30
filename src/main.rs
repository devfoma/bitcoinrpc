use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    // Connect to Bitcoin Core RPC in regtest mode
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_user = "smair";
    let rpc_pass = "smair123";

    // Create the RPC client
    let rpc = Client::new(rpc_url, Auth::UserPass(rpc_user.to_string(), rpc_pass.to_string()))
        .expect("Failed to create RPC client");

    // Example 1: Get basic blockchain info
    let info = rpc.get_blockchain_info().expect("RPC call failed");
    println!("Blockchain info: {:#?}", info);

    // Example 2: Get network info
    let network_info = rpc.get_network_info().expect("RPC call failed");
    println!("Network info: {:#?}", network_info);

    
}
