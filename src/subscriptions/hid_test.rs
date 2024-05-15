use std::sync::Arc;

use capnp::traits::IntoInternalStructReader;
use hid_io_client::capnp;
use hid_io_client::capnp::capability::Promise;
use hid_io_client::capnp_rpc;
use hid_io_client::common_capnp::NodeType;
use hid_io_client::keyboard_capnp;
use hid_io_client::setup_logging_lite;
use rand::Rng;

#[derive(Default)]
pub struct HidTest {
    auth: Option<hid_io_client::hidio_capnp::hid_io::Client>,
    server: Option<hid_io_client::hidio_capnp::hid_io_server::Client>,
    rng: rand::rngs::ThreadRng,
    serial: String,
}

impl HidTest {
    pub async fn run() {
        let rt: Arc<tokio::runtime::Runtime> = Arc::new(
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap(),
        );

        rt.block_on(async {
            // Prepare hid-io-core connection
            let mut hidio_conn = hid_io_client::HidioConnection::new().unwrap();
            let mut rng = rand::thread_rng();

            // Serial is used for automatic reconnection if hid-io goes away and comes back
            let serial = "".to_string();

            // Connect and authenticate with hid-io-core
            let (hidio_auth, hidio_server) = hidio_conn
                .connect(
                    hid_io_client::AuthType::Priviledged,
                    NodeType::HidioApi,
                    "HID-IO ErgoOne".to_string(),
                    format!("{:x} - pid:{}", rng.gen::<u64>(), std::process::id()),
                    true,
                    std::time::Duration::from_millis(1000),
                )
                .await
                .unwrap();
            let hidio_auth = hidio_auth.expect("Could not authenticate to hid-io-core");

            let nodes_resp = {
                let request = hidio_auth.nodes_request();
                request.send().promise.await.unwrap()
            };
            let nodes = nodes_resp.get().unwrap().get_nodes().unwrap();
            nodes.iter().for_each(|n| {
                println!("Node: {:?}", hid_io_client::format_node(n));
            });
        });
    }
}

impl keyboard_capnp::keyboard::subscriber::Server for HidTest {
    fn update(
        &mut self,
        params: keyboard_capnp::keyboard::subscriber::UpdateParams,
        _results: keyboard_capnp::keyboard::subscriber::UpdateResults,
    ) -> Promise<(), capnp::Error> {
        // println!("Data: {}", params.get().unwrap().get_signal().unwrap().get_data().into_internal_struct_reader().get_data_field::<u16>(1));
        let data = params
            .get()
            .unwrap()
            .get_signal()
            .unwrap()
            .get_data()
            .into_internal_struct_reader()
            .get_data_field::<u16>(1);
        let params = capnp_rpc::pry!(capnp_rpc::pry!(params.get()).get_signal())
            .get_data()
            .to_owned();
        // println!("{:?}", data);
        match params.which().unwrap() {
            hid_io_client::keyboard_capnp::keyboard::signal::data::Which::Volume(v) => {
                let v = v.unwrap();
                let cmd = v.get_cmd().unwrap();
                let vol = v.get_vol();
                let app_raw = v.get_app().unwrap();
                let app = match app_raw.len() {
                    0 => None,
                    _ => Some(app_raw),
                };
                let app_msg = match app_raw.len() {
                    0 => None,
                    _ => Some(app_raw.to_string()),
                };
                let msg = hid_client_stdout::Messages::Volume(cmd, vol, app_msg);
                let str = String::try_from(msg).unwrap();
                println!("{}", str);
            }
            hid_io_client::keyboard_capnp::keyboard::signal::data::Which::LayerChanged(l) => {
                let l = l.unwrap();
                let msg = hid_client_stdout::Messages::LayerChanged(l.get_layer());
                let str = String::try_from(msg).unwrap();
                println!("{}", str);
            }
            #[allow(unreachable_patterns)]
            _ => {
                println!("Unknown signal");
            }
        }
        Promise::ok(())
    }
}
