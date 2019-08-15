mod rpc_client;

use crate::rpc_client::RpcClient;
fn main() {
    let m = RpcClient {};
    m.run();
}
