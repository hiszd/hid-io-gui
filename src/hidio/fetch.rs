use hid_client_stdout::Node;

pub async fn fetch_nodes() -> Option<Vec<Node>> {
    let out =
        tokio::process::Command::new("/home/zion/programming/rust/hidiokb/target/release/hidiokb")
            .arg("list")
            .output()
            .await
            .unwrap()
            .stdout;

    let strout = String::from_utf8(out).unwrap();
    let nodes = strout
        .split("\n")
        .into_iter()
        .fold(Vec::new(), |mut acc, s| {
            if s.is_empty() {
                return acc;
            }
            acc.push(Node::try_from(s.to_string()).unwrap());
            acc
        });

    println!("Nodes: {:?}", nodes);
    Some(nodes)
}
