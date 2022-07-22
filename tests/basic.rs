use avro_schema_gen::AvroSchema;
use pretty_assertions::assert_eq;
use serde_json::json;

trait AvroSchema {
    fn to_schema(&self) -> serde_json::Value;
}

#[derive(AvroSchema)]
struct Foo {
    bar: String,
    baz: String,
}

#[test]
fn it_serializes_a_struct() {
    let expected = r#"
      {
        "name": "Foo",
        "type": "Record",
        "fields": [
          "bar": "String"
        ]
      }
    "#;

    let thing = Foo {
        bar: "bar".to_string(),
        baz: "thing".to_string(),
    };

    assert_eq!(
        expected,
        serde_json::to_string_pretty(&thing.to_schema()).unwrap()
    );
}
