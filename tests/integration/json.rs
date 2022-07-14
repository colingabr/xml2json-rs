use xml2json::JsonConfig;

use serde_json::Value as JsonValue;

use pretty_assertions::assert_eq;

use std::{fs::File, io::Read, path::Path};

pub fn load_xml(file_path: &str) -> String {
  let absolute_path = Path::new(file_path).canonicalize().unwrap();
  let mut file = File::open(absolute_path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

// === generated tests ===

#[test]
fn build_cds_default() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"$":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:Description":[{"$":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_attrkey_a() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"a":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:Description":[{"a":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("a").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_attrkey_attrkey() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"attrkey":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:Description":[{"attrkey":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("attrkey").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_explicit_root_false() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"$":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:Description":[{"$":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_root(false).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_explicit_charkey_true() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"$":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:Description":[{"$":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":[{"_":"Death Grips"}],"cd:country":[{"_":"USA"}],"cd:company":[{"_":"Third Worlds"}],"cd:price":[{"_":"00.00"}],"cd:year":[{"_":"2012"}]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_charkey(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_ignore_attrs_true() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"rdf:Description":[{"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().ignore_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_merge_attrs_true() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:RDF":{"xmlns:rdf":["http://www.w3.org/1999/02/22-rdf-syntax-ns#"],"xmlns:cd":["http://www.recshop.fake/cd#"],"rdf:Description":[{"rdf:about":["http://www.recshop.fake/cd/The Money Store"],"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().merge_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_normalize_tags_true() {
  let xml = load_xml("tests/data/cds.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"rdf:rdf":{"$":{"xmlns:rdf":"http://www.w3.org/1999/02/22-rdf-syntax-ns#","xmlns:cd":"http://www.recshop.fake/cd#"},"rdf:description":[{"$":{"rdf:about":"http://www.recshop.fake/cd/The Money Store"},"cd:artist":["Death Grips"],"cd:country":["USA"],"cd:company":["Third Worlds"],"cd:price":["00.00"],"cd:year":["2012"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().lowercase_tags(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_default() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_charkey_c() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"c":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"c":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"c":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"c":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"c":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"c":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().charkey("c").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_charkey_charkey() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"charkey":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"charkey":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"charkey":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"charkey":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"charkey":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"charkey":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().charkey("charkey").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_attrkey_a() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","a":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","a":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","a":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"a":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"a":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","a":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"a":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"a":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"a":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"a":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"a":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("a").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_attrkey_attrkey() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","attrkey":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","attrkey":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","attrkey":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"attrkey":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"attrkey":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","attrkey":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"attrkey":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"attrkey":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"attrkey":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"attrkey":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"attrkey":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("attrkey").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_empty_tag_e() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":["e"],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":["e"],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":["e"],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["e"]}}"#).unwrap();

  let json_builder = JsonConfig::new().empty_tag("e").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_empty_tag_empty() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":["empty"],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":["empty"],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":["empty"],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["empty"]}}"#).unwrap();

  let json_builder = JsonConfig::new().empty_tag("empty").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_explicit_root_false() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_root(false).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_explicit_charkey_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":[{"_":"\n      data\n    "}],"simpletest2":[{"_":"\n      data\n    "}],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":[{"_":"Foo(1)"},{"_":"Foo(2)"},{"_":"Foo(3)"},{"_":"Foo(4)"}]},{"_":"Qux."},{"_":"Quux."}],"single":[{"_":"Single"}]}],"arraytest":[{"item":[{"subitem":[{"_":"Baz."}]},{"subitem":[{"_":"Foo."},{"_":"Bar."}]}]}],"emptytest":[""],"tagcasetest":[{"tAg":[{"_":"something"}],"TAG":[{"_":"something else"}],"tag":[{"_":"something third"}]}],"ordertest":[{"one":[{"_":"1"},{"_":"4"}],"two":[{"_":"2"},{"_":"5"}],"three":[{"_":"3"},{"_":"6"}]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":[{"_":"Bar."}]}],"numbertest":[{"_":"42"}],"stringtest":[{"_":"43"}]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":[{"_":"some value"}],"textordertest":[{"_":"this is text with     in the middle","b":[{"_":"markup"}],"em":[{"_":"like this"}]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_charkey(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_trim_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["data"],"simpletest2":["data"],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"Line One\n        Line Two","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"This  is\n            \n            character\n            \n            data!","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().trim(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_ignore_attrs_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":["Character data here!"],"cdatatest":["CDATA here!"],"cdatawhitespacetest":["   "],"nochartest":[""],"nochildrentest":[""],"whitespacetest":["\n        Line One\n        Line Two\n    "],"listtest":[{"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"middle":[""]}],"attrNameProcessTest":[""],"attrValueProcessTest":[""],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().ignore_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_merge_attrs_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","desc":["Test for CHARs"]}],"cdatatest":[{"_":"CDATA here!","desc":["Test for CDATA"],"misc":["true"]}],"cdatawhitespacetest":[{"_":"   ","desc":["Test for CDATA with whitespace"],"misc":["true"]}],"nochartest":[{"desc":["No data"],"misc":["false"]}],"nochildrentest":[{"desc":["No data"],"misc":["false"]}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","desc":["Test for       normalizing and trimming"]}],"listtest":[{"attr":["Attribute"],"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"xmlns:pfx":["http://foo.com"],"pfx:attr":["baz"],"middle":[{"xmlns":["http://bar.com"]}]}],"attrNameProcessTest":[{"camelCaseAttr":["camelCaseAttrValue"],"lowercaseattr":["lowercaseattrvalue"]}],"attrValueProcessTest":[{"camelCaseAttr":["camelCaseAttrValue"],"lowerCaseAttr":["lowercaseattrvalue"]}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().merge_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_normalize_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["data"],"simpletest2":["data"],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"Line One Line Two","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"This is character data!","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tAg":["something"],"TAG":["something else"],"tag":["something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrNameProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrValueProcessTest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagNameProcessTest":[""],"valueProcessTest":["some value"],"textordertest":[{"_":"this is text with in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().normalize_text(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_normalize_tags_true() {
  let xml = load_xml("tests/data/complex.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"sample":{"simpletest":["\n      data\n    "],"simpletest2":["\n      data\n    "],"chartest":[{"_":"Character data here!","$":{"desc":"Test for CHARs"}}],"cdatatest":[{"_":"CDATA here!","$":{"desc":"Test for CDATA","misc":"true"}}],"cdatawhitespacetest":[{"_":"   ","$":{"desc":"Test for CDATA with whitespace","misc":"true"}}],"nochartest":[{"$":{"desc":"No data","misc":"false"}}],"nochildrentest":[{"$":{"desc":"No data","misc":"false"}}],"whitespacetest":[{"_":"\n        Line One\n        Line Two\n    ","$":{"desc":"Test for       normalizing and trimming"}}],"listtest":[{"$":{"attr":"Attribute"},"item":[{"_":"\n            This  is\n            \n            character\n            \n            data!\n            \n        ","subitem":["Foo(1)","Foo(2)","Foo(3)","Foo(4)"]},"Qux.","Quux."],"single":["Single"]}],"arraytest":[{"item":[{"subitem":["Baz."]},{"subitem":["Foo.","Bar."]}]}],"emptytest":[""],"tagcasetest":[{"tag":["something","something else","something third"]}],"ordertest":[{"one":["1","4"],"two":["2","5"],"three":["3","6"]}],"validatortest":[{"emptyarray":[""],"oneitemarray":[{"item":["Bar."]}],"numbertest":["42"],"stringtest":["43"]}],"pfx:top":[{"$":{"xmlns:pfx":"http://foo.com","pfx:attr":"baz"},"middle":[{"$":{"xmlns":"http://bar.com"}}]}],"attrnameprocesstest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowercaseattr":"lowercaseattrvalue"}}],"attrvalueprocesstest":[{"$":{"camelCaseAttr":"camelCaseAttrValue","lowerCaseAttr":"lowercaseattrvalue"}}],"tagnameprocesstest":[""],"valueprocesstest":["some value"],"textordertest":[{"_":"this is text with     in the middle","b":["markup"],"em":["like this"]}],"emptytestanother":["\n\n    "]}}"#).unwrap();

  let json_builder = JsonConfig::new().lowercase_tags(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_default() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_charkey_c() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"c":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().charkey("c").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_charkey_charkey() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"charkey":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().charkey("charkey").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_attrkey_a() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"a":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("a").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_attrkey_attrkey() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"attrkey":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().attrkey("attrkey").finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_explicit_root_false() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_root(false).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_explicit_charkey_true() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":[{"_":"value"}],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":[{"_":"Foo(1)"},{"_":"Foo(2)"},{"_":"Foo(3)"},{"_":"4"}]},{"_":"Qux."},{"_":"Quux."}],"single":[{"_":"Single"}]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().explicit_charkey(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_trim_true() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"This  is\n          \n          character\n          \n          data!","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().trim(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_ignore_attrs_true() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().ignore_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_merge_attrs_true() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"attr":["Attribute"],"another":["attrbte"],"item":[{"_":"\n          This  is\n          \n          character\n          \n          data!\n          \n      ","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().merge_attrs(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_normalize_true() {
  let xml = load_xml("tests/data/lists.xml");
  let expected: JsonValue = serde_json::from_str(r#"{"outer":{"key":["value"],"listtest":[{"$":{"attr":"Attribute","another":"attrbte"},"item":[{"_":"This is character data!","subitem":["Foo(1)","Foo(2)","Foo(3)","4"]},"Qux.","Quux."],"single":["Single"]}]}}"#).unwrap();

  let json_builder = JsonConfig::new().normalize_text(true).finalize();
  let result = json_builder.build_from_xml(&xml);

  let actual = result.expect("Error building JSON.");
  assert_eq!(expected, actual);
}
