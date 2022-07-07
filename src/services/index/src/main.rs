use marine_rs_sdk::{marine, WasmLoggerBuilder};
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::Connection;
use marine_sqlite_connector::{State};


module_manifest!();

// static SQLITE: Lazy<Connection> = Lazy::new(|| marine_sqlite_connector::open(":memory:").unwrap());
const DB_PATH: &'static str = "/tmp/index.sqlite";

pub fn main() {
    WasmLoggerBuilder::new().build().expect("Error init logger");
    db()
        .execute(
            "
            CREATE TABLE IF NOT EXISTS ipfs_index (
                cid TEXT,
                peer_id TEXT,
                multiaddr TEXT,
                PRIMARY KEY(cid, peer_id, multiaddr)
            );
            ",
        )
        .expect("create tables");
}

/*
add: (cid, peer_id, multiaddr) => {
get: (cid) => {
remove: (cid, peer_id, multiaddr) => {
*/

fn db() -> Connection {
    marine_sqlite_connector::Connection::open(DB_PATH).expect("Error opening database")
}

#[marine]
pub fn add(cid: String, peer_id: String, multiaddr: String) {
    db().execute(format!("INSERT OR REPLACE INTO ipfs_index VALUES ('{}', '{}', '{}')", cid, peer_id, multiaddr)).expect("insert failed");
}

#[marine]
pub struct Provider {
    peer_id: String,
    multiaddr: String,
}

#[marine]
pub fn get(cid: String) -> Vec<Provider> {
    let mut st = db().prepare(format!("SELECT * FROM ipfs_index WHERE cid = '{}'", cid)).expect("prepare");

    let mut providers = vec![];
    while let State::Row = st.next().expect("select") {
        let provider = Provider {
            peer_id: st.read(1).expect("read peer_id"),
            multiaddr: st.read(2).expect("read multiaddr"),
        };
        providers.push(provider);
    }

    return providers;
}

#[marine]
pub fn delete(cid: String, peer_id: String, multiaddr: String) {
    db().execute(
        format!(
            "DELETE FROM ipfs_index WHERE cid = '{}' AND peer_id = '{}' AND multiaddr = '{}'",
            cid, peer_id, multiaddr
        )
    ).expect("remove");
}


// To run tests:
// cargo test --release
// Since the unit tests are using the Wasm module via the marine_test crate import
// the modules and Config.toml need to exist.
// That is, run ./build.sh before you run cargo test.
// Moreover, the test function(s) need to be prefixed by the wasm module namespace, which
// generally is derived from the project name.
// For example, if you name the project "greeting", e.g., cargo generate -g https:// ... --name greeting,
// the test below can be executed as is. If not, the project needs to replace the "greeting"
// reference in place. Let's say our project name is "greetproject" then you need to update the type to:
// fn test_greeting(greeting: marine_test_env::greetproject::ModuleInterface)
#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn test_add_get_delete(index: marine_test_env::index::ModuleInterface) {
        let cid = "a".to_string();
        let peer_id = "b".to_string();
        let multiaddr = "c".to_string();
        index.add(cid.clone(), peer_id.clone(), multiaddr.clone());

        let res = index.get(cid.clone());
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].peer_id, peer_id);
        assert_eq!(res[0].multiaddr, multiaddr);

        index.delete(cid.clone(), peer_id, multiaddr);
        let res = index.get(cid.clone());
        assert_eq!(res.len(), 0);
    }
}
