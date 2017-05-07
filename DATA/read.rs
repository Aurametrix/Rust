#[macro_use]
extern crate serde_derive;
extern crate toml;

#[derive(Deserialize)]
struct MyConfiguration {
  jenkins_host: String,
  jenkins_username: String,
  jenkins_token: String
}

fn gimme_config(some_filename: &str) -> MyConfiguration {
  let mut file = File::open(some_filename).unwrap();
  let mut s = String::new();
  file.read_to_string(&mut s).unwrap();
  let my_config: MyConfiguration = toml::from_str(s).unwrap();
  my_config
}
