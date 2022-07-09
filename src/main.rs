use flowlang::datastore::DataStore;
use flowlang::generated::Generated;
use flowlang::command::Command;
use ndata::dataobject::DataObject;

fn main() {
    DataStore::init("../flow/data");
    Generated::init();
    {
        let args = DataObject::from_json(serde_json::from_str(r#"
        {
          "a": 299,
          "b": 121
        }
        "#).unwrap());
        let cmd = Command::lookup("testflow", "testflow", "test_add");
        let res = cmd.execute(args).unwrap();
        println!("Hello, my dudes! {}", res.to_json());
    }
    DataStore::gc();
}
