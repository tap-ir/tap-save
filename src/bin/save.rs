use tap::session::Session;
use tap::value::Value;

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct Test
{
  pub string : String,
  pub int : u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TestEnum
{
  Int(u32),
}

fn main() 
{
  pretty_env_logger::init();

  let session = Session::new();
  println!("saving tree");

  let root_id = session.tree.root_id;
  let root = session.tree.get_node_from_id(root_id).unwrap();
  let mut attributes = root.value();

  //attributes.add_attribute("tag", "malware", None); 

  //let value : JSon::Value = json::from_str();
  //let json_value : serde_json::Value = attribute.value;

  let val_int = Value::U32(1);
  println!("{:?}", val_int);
  println!("{}", json!(val_int));
  let value : Value = serde_json::from_str(r#"{"type":"U32", "value":1}"#).unwrap();
  attributes.add_attribute("tag_id", value.clone(), None); 
  println!("Value {:?}", value);

  let val_int = Value::String("test".to_string());
  println!("{:?}", val_int);
  println!("{}", json!(val_int));
  let value : Value = serde_json::from_str(r#"{"type":"String", "value" : "malware"}"#).unwrap();
  attributes.add_attribute("tag", value.clone(), None); 
  println!("Value {:?}", value);

  //XXX does nort live long enough
  //let string =  r#"{"String":"malware"}"#.to_string();
  //let value_string : Value = serde_json::from_str(&string).unwrap();

  //XXX 
  let test = Test{ string : "test".to_string(), int : 2 };
  let test_json_value = json!(test);
  let test_json_value_string = test_json_value.to_string(); 
  let _deserialize_test : Test = serde_json::from_str(&test_json_value_string).unwrap();
  //let deserialize_test : Test = serde_json::from_value(serde_json::Value::String(test_json_value_string)).unwrap();

  let test_enum = TestEnum::Int(12);
  let test_enum_json_value = json!(test_enum);
  let test_enum_json_string = test_enum_json_value.to_string();
  let deserialize_enum : TestEnum = serde_json::from_str(&test_enum_json_string).unwrap();
  println!("test_enum {}", test_enum_json_string);
  println!("test_enum {:?}", deserialize_enum);

  let value : Value = Value::U32(2);
  let value_json_value = json!(value);
  let value_json_string = value_json_value.to_string();
  let deserialize_value : Value = serde_json::from_str(&value_json_string).unwrap();
  println!("deserialize value {:?}", deserialize_value);

  //let reader = File::open("test").unwrap();
  //let value_file : Value  = serde_json::from_reader(reader).unwrap();

  println!("------------------------------");
  println!("Tree :");
  println!("{}", session.tree);
  println!("Tree node to json");
  println!("{}", serde_json::to_string(&session.tree).unwrap());

}
