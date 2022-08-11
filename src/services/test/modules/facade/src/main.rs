use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
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
    fn test_greeting(greeting: marine_test_env::greeting::ModuleInterface) {
        let name = "Marine";
        let res = greeting.greeting(name.to_string());
        assert_eq!(res, format!("Hi, {}", name));
    }
}
