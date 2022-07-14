use xml2json_rs::{Declaration, Encoding, Indentation, Version, XmlConfig};

use serde_json::Value as JsonValue;

use indoc::indoc;
use pretty_assertions::assert_eq;

use std::{fs::File, io::Read, path::Path};

pub fn load_json(file_path: &str) -> JsonValue {
  let absolute_path = Path::new(file_path).canonicalize().unwrap();
  let mut file = File::open(absolute_path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  serde_json::from_str(&contents).unwrap()
}

// === generated tests ===

#[test]
fn build_cds_default() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
    <rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
      <cd:artist>Death Grips</cd:artist>
      <cd:country>USA</cd:country>
      <cd:company>Third Worlds</cd:company>
      <cd:price>00.00</cd:price>
      <cd:year>2012</cd:year>
    </rdf:Description>
  </rdf:RDF>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_pretty_false() {
  let object = load_json("tests/data/cds.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\" xmlns:cd=\"http://www.recshop.fake/cd#\"><rdf:Description rdf:about=\"http://www.recshop.fake/cd/The Money Store\"><cd:artist>Death Grips</cd:artist><cd:country>USA</cd:country><cd:company>Third Worlds</cd:company><cd:price>00.00</cd:price><cd:year>2012</cd:year></rdf:Description></rdf:RDF>" ;

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_indent_tab() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
  	<rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
  		<cd:artist>Death Grips</cd:artist>
  		<cd:country>USA</cd:country>
  		<cd:company>Third Worlds</cd:company>
  		<cd:price>00.00</cd:price>
  		<cd:year>2012</cd:year>
  	</rdf:Description>
  </rdf:RDF>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_indent_empty() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
  <rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
  <cd:artist>Death Grips</cd:artist>
  <cd:country>USA</cd:country>
  <cd:company>Third Worlds</cd:company>
  <cd:price>00.00</cd:price>
  <cd:year>2012</cd:year>
  </rdf:Description>
  </rdf:RDF>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_version_11() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
    <rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
      <cd:artist>Death Grips</cd:artist>
      <cd:country>USA</cd:country>
      <cd:company>Third Worlds</cd:company>
      <cd:price>00.00</cd:price>
      <cd:year>2012</cd:year>
    </rdf:Description>
  </rdf:RDF>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_standalone_false() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
    <rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
      <cd:artist>Death Grips</cd:artist>
      <cd:country>USA</cd:country>
      <cd:company>Third Worlds</cd:company>
      <cd:price>00.00</cd:price>
      <cd:year>2012</cd:year>
    </rdf:Description>
  </rdf:RDF>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_cds_root_name_object() {
  let object = load_json("tests/data/cds.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#" xmlns:cd="http://www.recshop.fake/cd#">
      <rdf:Description rdf:about="http://www.recshop.fake/cd/The Money Store">
        <cd:artist>Death Grips</cd:artist>
        <cd:country>USA</cd:country>
        <cd:company>Third Worlds</cd:company>
        <cd:price>00.00</cd:price>
        <cd:year>2012</cd:year>
      </rdf:Description>
    </rdf:RDF>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_default() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <OwnerID>0</OwnerID>
        <Hash>browns</Hash>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_pretty_false() {
  let object = load_json("tests/data/childobj.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><SOAP-ENV:Envelope xmlns:SOAP-ENV=\"env\" \
                  xmlns:ns1=\"ns1\"><SOAP-ENV:Body><ns1:createWorkflowProof><SessionID>sid</SessionID><OwnerID>0</OwnerID><Hash>browns</\
                  Hash></ns1:createWorkflowProof></SOAP-ENV:Body></SOAP-ENV:Envelope>";

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_indent_tab() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
  	<SOAP-ENV:Body>
  		<ns1:createWorkflowProof>
  			<SessionID>sid</SessionID>
  			<OwnerID>0</OwnerID>
  			<Hash>browns</Hash>
  		</ns1:createWorkflowProof>
  	</SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_indent_empty() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
  <SOAP-ENV:Body>
  <ns1:createWorkflowProof>
  <SessionID>sid</SessionID>
  <OwnerID>0</OwnerID>
  <Hash>browns</Hash>
  </ns1:createWorkflowProof>
  </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_version_11() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <OwnerID>0</OwnerID>
        <Hash>browns</Hash>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_standalone_false() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <OwnerID>0</OwnerID>
        <Hash>browns</Hash>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_childobj_root_name_object() {
  let object = load_json("tests/data/childobj.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <SOAP-ENV:Envelope xmlns:SOAP-ENV="env" xmlns:ns1="ns1">
      <SOAP-ENV:Body>
        <ns1:createWorkflowProof>
          <SessionID>sid</SessionID>
          <OwnerID>0</OwnerID>
          <Hash>browns</Hash>
        </ns1:createWorkflowProof>
      </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_default() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <sample>
    <simpletest>data</simpletest>
    <simpletest2>data</simpletest2>
    <chartest desc="Test for CHARs">Character data here!</chartest>
    <cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
    <cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
    <nochartest desc="No data" misc="false"/>
    <nochildrentest desc="No data" misc="false"/>
    <whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
    <listtest attr="Attribute">
      <item>
        This is character data!
        <subitem>Foo(1)</subitem>
        <subitem>Foo(2)</subitem>
        <subitem>Foo(3)</subitem>
        <subitem>Foo(4)</subitem>
      </item>
      <item>Qux.</item>
      <item>Quux.</item>
      <single>Single</single>
    </listtest>
    <arraytest>
      <item>
        <subitem>Baz.</subitem>
      </item>
      <item>
        <subitem>Foo.</subitem>
        <subitem>Bar.</subitem>
      </item>
    </arraytest>
    <emptytest/>
    <tagcasetest>
      <tAg>something</tAg>
      <TAG>something else</TAG>
      <tag>something third</tag>
    </tagcasetest>
    <ordertest>
      <one>1</one>
      <one>4</one>
      <two>2</two>
      <two>5</two>
      <three>3</three>
      <three>6</three>
    </ordertest>
    <validatortest>
      <emptyarray/>
      <oneitemarray>
        <item>Bar.</item>
      </oneitemarray>
      <numbertest>42</numbertest>
      <stringtest>43</stringtest>
    </validatortest>
    <pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
      <middle xmlns="http://bar.com"/>
    </pfx:top>
    <attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
    <attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
    <tagNameProcessTest/>
    <valueProcessTest>some value</valueProcessTest>
    <textordertest>
      this is text with in the middle
      <b>markup</b>
      <em>like this</em>
    </textordertest>
    <emptytestanother>
  
      </emptytestanother>
  </sample>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_pretty_false() {
  let object = load_json("tests/data/complex.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><sample><simpletest>data</simpletest><simpletest2>data</simpletest2><chartest desc=\"Test for CHARs\">Character data here!</chartest><cdatatest desc=\"Test for CDATA\" misc=\"true\">CDATA here!</cdatatest><cdatawhitespacetest desc=\"Test for CDATA with whitespace\" misc=\"true\"/><nochartest desc=\"No data\" misc=\"false\"/><nochildrentest desc=\"No data\" misc=\"false\"/><whitespacetest desc=\"Test for       normalizing and trimming\">Line One Line Two</whitespacetest><listtest attr=\"Attribute\"><item>This is character data!<subitem>Foo(1)</subitem><subitem>Foo(2)</subitem><subitem>Foo(3)</subitem><subitem>Foo(4)</subitem></item><item>Qux.</item><item>Quux.</item><single>Single</single></listtest><arraytest><item><subitem>Baz.</subitem></item><item><subitem>Foo.</subitem><subitem>Bar.</subitem></item></arraytest><emptytest/><tagcasetest><tAg>something</tAg><TAG>something else</TAG><tag>something third</tag></tagcasetest><ordertest><one>1</one><one>4</one><two>2</two><two>5</two><three>3</three><three>6</three></ordertest><validatortest><emptyarray/><oneitemarray><item>Bar.</item></oneitemarray><numbertest>42</numbertest><stringtest>43</stringtest></validatortest><pfx:top xmlns:pfx=\"http://foo.com\" pfx:attr=\"baz\"><middle xmlns=\"http://bar.com\"/></pfx:top><attrNameProcessTest camelCaseAttr=\"camelCaseAttrValue\" lowercaseattr=\"lowercaseattrvalue\"/><attrValueProcessTest camelCaseAttr=\"camelCaseAttrValue\" lowerCaseAttr=\"lowercaseattrvalue\"/><tagNameProcessTest/><valueProcessTest>some value</valueProcessTest><textordertest>this is text with in the middle<b>markup</b><em>like this</em></textordertest><emptytestanother>

    </emptytestanother></sample>" ;

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_indent_tab() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <sample>
  	<simpletest>data</simpletest>
  	<simpletest2>data</simpletest2>
  	<chartest desc="Test for CHARs">Character data here!</chartest>
  	<cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
  	<cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
  	<nochartest desc="No data" misc="false"/>
  	<nochildrentest desc="No data" misc="false"/>
  	<whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
  	<listtest attr="Attribute">
  		<item>
  			This is character data!
  			<subitem>Foo(1)</subitem>
  			<subitem>Foo(2)</subitem>
  			<subitem>Foo(3)</subitem>
  			<subitem>Foo(4)</subitem>
  		</item>
  		<item>Qux.</item>
  		<item>Quux.</item>
  		<single>Single</single>
  	</listtest>
  	<arraytest>
  		<item>
  			<subitem>Baz.</subitem>
  		</item>
  		<item>
  			<subitem>Foo.</subitem>
  			<subitem>Bar.</subitem>
  		</item>
  	</arraytest>
  	<emptytest/>
  	<tagcasetest>
  		<tAg>something</tAg>
  		<TAG>something else</TAG>
  		<tag>something third</tag>
  	</tagcasetest>
  	<ordertest>
  		<one>1</one>
  		<one>4</one>
  		<two>2</two>
  		<two>5</two>
  		<three>3</three>
  		<three>6</three>
  	</ordertest>
  	<validatortest>
  		<emptyarray/>
  		<oneitemarray>
  			<item>Bar.</item>
  		</oneitemarray>
  		<numbertest>42</numbertest>
  		<stringtest>43</stringtest>
  	</validatortest>
  	<pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
  		<middle xmlns="http://bar.com"/>
  	</pfx:top>
  	<attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
  	<attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
  	<tagNameProcessTest/>
  	<valueProcessTest>some value</valueProcessTest>
  	<textordertest>
  		this is text with in the middle
  		<b>markup</b>
  		<em>like this</em>
  	</textordertest>
  	<emptytestanother>
  
      </emptytestanother>
  </sample>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_indent_empty() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <sample>
  <simpletest>data</simpletest>
  <simpletest2>data</simpletest2>
  <chartest desc="Test for CHARs">Character data here!</chartest>
  <cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
  <cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
  <nochartest desc="No data" misc="false"/>
  <nochildrentest desc="No data" misc="false"/>
  <whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
  <listtest attr="Attribute">
  <item>
  This is character data!
  <subitem>Foo(1)</subitem>
  <subitem>Foo(2)</subitem>
  <subitem>Foo(3)</subitem>
  <subitem>Foo(4)</subitem>
  </item>
  <item>Qux.</item>
  <item>Quux.</item>
  <single>Single</single>
  </listtest>
  <arraytest>
  <item>
  <subitem>Baz.</subitem>
  </item>
  <item>
  <subitem>Foo.</subitem>
  <subitem>Bar.</subitem>
  </item>
  </arraytest>
  <emptytest/>
  <tagcasetest>
  <tAg>something</tAg>
  <TAG>something else</TAG>
  <tag>something third</tag>
  </tagcasetest>
  <ordertest>
  <one>1</one>
  <one>4</one>
  <two>2</two>
  <two>5</two>
  <three>3</three>
  <three>6</three>
  </ordertest>
  <validatortest>
  <emptyarray/>
  <oneitemarray>
  <item>Bar.</item>
  </oneitemarray>
  <numbertest>42</numbertest>
  <stringtest>43</stringtest>
  </validatortest>
  <pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
  <middle xmlns="http://bar.com"/>
  </pfx:top>
  <attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
  <attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
  <tagNameProcessTest/>
  <valueProcessTest>some value</valueProcessTest>
  <textordertest>
  this is text with in the middle
  <b>markup</b>
  <em>like this</em>
  </textordertest>
  <emptytestanother>
  
      </emptytestanother>
  </sample>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_version_11() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <sample>
    <simpletest>data</simpletest>
    <simpletest2>data</simpletest2>
    <chartest desc="Test for CHARs">Character data here!</chartest>
    <cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
    <cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
    <nochartest desc="No data" misc="false"/>
    <nochildrentest desc="No data" misc="false"/>
    <whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
    <listtest attr="Attribute">
      <item>
        This is character data!
        <subitem>Foo(1)</subitem>
        <subitem>Foo(2)</subitem>
        <subitem>Foo(3)</subitem>
        <subitem>Foo(4)</subitem>
      </item>
      <item>Qux.</item>
      <item>Quux.</item>
      <single>Single</single>
    </listtest>
    <arraytest>
      <item>
        <subitem>Baz.</subitem>
      </item>
      <item>
        <subitem>Foo.</subitem>
        <subitem>Bar.</subitem>
      </item>
    </arraytest>
    <emptytest/>
    <tagcasetest>
      <tAg>something</tAg>
      <TAG>something else</TAG>
      <tag>something third</tag>
    </tagcasetest>
    <ordertest>
      <one>1</one>
      <one>4</one>
      <two>2</two>
      <two>5</two>
      <three>3</three>
      <three>6</three>
    </ordertest>
    <validatortest>
      <emptyarray/>
      <oneitemarray>
        <item>Bar.</item>
      </oneitemarray>
      <numbertest>42</numbertest>
      <stringtest>43</stringtest>
    </validatortest>
    <pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
      <middle xmlns="http://bar.com"/>
    </pfx:top>
    <attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
    <attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
    <tagNameProcessTest/>
    <valueProcessTest>some value</valueProcessTest>
    <textordertest>
      this is text with in the middle
      <b>markup</b>
      <em>like this</em>
    </textordertest>
    <emptytestanother>
  
      </emptytestanother>
  </sample>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_standalone_false() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <sample>
    <simpletest>data</simpletest>
    <simpletest2>data</simpletest2>
    <chartest desc="Test for CHARs">Character data here!</chartest>
    <cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
    <cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
    <nochartest desc="No data" misc="false"/>
    <nochildrentest desc="No data" misc="false"/>
    <whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
    <listtest attr="Attribute">
      <item>
        This is character data!
        <subitem>Foo(1)</subitem>
        <subitem>Foo(2)</subitem>
        <subitem>Foo(3)</subitem>
        <subitem>Foo(4)</subitem>
      </item>
      <item>Qux.</item>
      <item>Quux.</item>
      <single>Single</single>
    </listtest>
    <arraytest>
      <item>
        <subitem>Baz.</subitem>
      </item>
      <item>
        <subitem>Foo.</subitem>
        <subitem>Bar.</subitem>
      </item>
    </arraytest>
    <emptytest/>
    <tagcasetest>
      <tAg>something</tAg>
      <TAG>something else</TAG>
      <tag>something third</tag>
    </tagcasetest>
    <ordertest>
      <one>1</one>
      <one>4</one>
      <two>2</two>
      <two>5</two>
      <three>3</three>
      <three>6</three>
    </ordertest>
    <validatortest>
      <emptyarray/>
      <oneitemarray>
        <item>Bar.</item>
      </oneitemarray>
      <numbertest>42</numbertest>
      <stringtest>43</stringtest>
    </validatortest>
    <pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
      <middle xmlns="http://bar.com"/>
    </pfx:top>
    <attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
    <attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
    <tagNameProcessTest/>
    <valueProcessTest>some value</valueProcessTest>
    <textordertest>
      this is text with in the middle
      <b>markup</b>
      <em>like this</em>
    </textordertest>
    <emptytestanother>
  
      </emptytestanother>
  </sample>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_complex_root_name_object() {
  let object = load_json("tests/data/complex.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <sample>
      <simpletest>data</simpletest>
      <simpletest2>data</simpletest2>
      <chartest desc="Test for CHARs">Character data here!</chartest>
      <cdatatest desc="Test for CDATA" misc="true">CDATA here!</cdatatest>
      <cdatawhitespacetest desc="Test for CDATA with whitespace" misc="true"/>
      <nochartest desc="No data" misc="false"/>
      <nochildrentest desc="No data" misc="false"/>
      <whitespacetest desc="Test for       normalizing and trimming">Line One Line Two</whitespacetest>
      <listtest attr="Attribute">
        <item>
          This is character data!
          <subitem>Foo(1)</subitem>
          <subitem>Foo(2)</subitem>
          <subitem>Foo(3)</subitem>
          <subitem>Foo(4)</subitem>
        </item>
        <item>Qux.</item>
        <item>Quux.</item>
        <single>Single</single>
      </listtest>
      <arraytest>
        <item>
          <subitem>Baz.</subitem>
        </item>
        <item>
          <subitem>Foo.</subitem>
          <subitem>Bar.</subitem>
        </item>
      </arraytest>
      <emptytest/>
      <tagcasetest>
        <tAg>something</tAg>
        <TAG>something else</TAG>
        <tag>something third</tag>
      </tagcasetest>
      <ordertest>
        <one>1</one>
        <one>4</one>
        <two>2</two>
        <two>5</two>
        <three>3</three>
        <three>6</three>
      </ordertest>
      <validatortest>
        <emptyarray/>
        <oneitemarray>
          <item>Bar.</item>
        </oneitemarray>
        <numbertest>42</numbertest>
        <stringtest>43</stringtest>
      </validatortest>
      <pfx:top xmlns:pfx="http://foo.com" pfx:attr="baz">
        <middle xmlns="http://bar.com"/>
      </pfx:top>
      <attrNameProcessTest camelCaseAttr="camelCaseAttrValue" lowercaseattr="lowercaseattrvalue"/>
      <attrValueProcessTest camelCaseAttr="camelCaseAttrValue" lowerCaseAttr="lowercaseattrvalue"/>
      <tagNameProcessTest/>
      <valueProcessTest>some value</valueProcessTest>
      <textordertest>
        this is text with in the middle
        <b>markup</b>
        <em>like this</em>
      </textordertest>
      <emptytestanother>
  
      </emptytestanother>
    </sample>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_default() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <outer>
    <item>
      This is character data!
      <subitem>Foo(1)</subitem>
      <subitem>Foo(2)</subitem>
      <subitem>Foo(3)</subitem>
      <subitem>4</subitem>
    </item>
    <item>Qux.</item>
    <item>Quux.</item>
  </outer>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_pretty_false() {
  let object = load_json("tests/data/lists.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><outer><item>This is character \
                  data!<subitem>Foo(1)</subitem><subitem>Foo(2)</subitem><subitem>Foo(3)</subitem><subitem>4</subitem></item><item>Qux.</\
                  item><item>Quux.</item></outer>";

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_indent_tab() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <outer>
  	<item>
  		This is character data!
  		<subitem>Foo(1)</subitem>
  		<subitem>Foo(2)</subitem>
  		<subitem>Foo(3)</subitem>
  		<subitem>4</subitem>
  	</item>
  	<item>Qux.</item>
  	<item>Quux.</item>
  </outer>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_indent_empty() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <outer>
  <item>
  This is character data!
  <subitem>Foo(1)</subitem>
  <subitem>Foo(2)</subitem>
  <subitem>Foo(3)</subitem>
  <subitem>4</subitem>
  </item>
  <item>Qux.</item>
  <item>Quux.</item>
  </outer>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_version_11() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <outer>
    <item>
      This is character data!
      <subitem>Foo(1)</subitem>
      <subitem>Foo(2)</subitem>
      <subitem>Foo(3)</subitem>
      <subitem>4</subitem>
    </item>
    <item>Qux.</item>
    <item>Quux.</item>
  </outer>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_standalone_false() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <outer>
    <item>
      This is character data!
      <subitem>Foo(1)</subitem>
      <subitem>Foo(2)</subitem>
      <subitem>Foo(3)</subitem>
      <subitem>4</subitem>
    </item>
    <item>Qux.</item>
    <item>Quux.</item>
  </outer>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_lists_root_name_object() {
  let object = load_json("tests/data/lists.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <outer>
      <item>
        This is character data!
        <subitem>Foo(1)</subitem>
        <subitem>Foo(2)</subitem>
        <subitem>Foo(3)</subitem>
        <subitem>4</subitem>
      </item>
      <item>Qux.</item>
      <item>Quux.</item>
    </outer>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_default() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <root>
    <key1>Value1</key1>
    <key2>Value2</key2>
  </root>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_pretty_false() {
  let object = load_json("tests/data/multiroot.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><root><key1>Value1</key1><key2>Value2</key2></root>";

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_indent_tab() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <root>
  	<key1>Value1</key1>
  	<key2>Value2</key2>
  </root>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_indent_empty() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <root>
  <key1>Value1</key1>
  <key2>Value2</key2>
  </root>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_version_11() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <root>
    <key1>Value1</key1>
    <key2>Value2</key2>
  </root>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_standalone_false() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <root>
    <key1>Value1</key1>
    <key2>Value2</key2>
  </root>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_multiroot_root_name_object() {
  let object = load_json("tests/data/multiroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <key1>Value1</key1>
    <key2>Value2</key2>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_singleroot_default() {
  let object = load_json("tests/data/singleroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <foo>bar</foo>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_singleroot_pretty_false() {
  let object = load_json("tests/data/singleroot.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><foo>bar</foo>";

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_singleroot_version_11() {
  let object = load_json("tests/data/singleroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <foo>bar</foo>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_singleroot_standalone_false() {
  let object = load_json("tests/data/singleroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <foo>bar</foo>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_singleroot_root_name_object() {
  let object = load_json("tests/data/singleroot.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <foo>bar</foo>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_default() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <Stages>
          <item>
            <stage_reviewers>
              <item>
                <email>robert.paulson@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 1</name>
          </item>
          <item>
            <stage_reviewers>
              <item>
                <email>tyler.durden@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 2</name>
          </item>
        </Stages>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_pretty_false() {
  let object = load_json("tests/data/soap.json");
  let expected = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><SOAP-ENV:Envelope xmlns:SOAP-ENV=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:ns1=\"https://paper-street.soap.com/\"><SOAP-ENV:Body><ns1:createWorkflowProof><SessionID>sid</SessionID><Stages><item><stage_reviewers><item><email>robert.paulson@gmail.com</email></item></stage_reviewers><name>Stage 1</name></item><item><stage_reviewers><item><email>tyler.durden@gmail.com</email></item></stage_reviewers><name>Stage 2</name></item></Stages></ns1:createWorkflowProof></SOAP-ENV:Body></SOAP-ENV:Envelope>" ;

  let mut xml_builder = XmlConfig::new()
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_indent_tab() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
  	<SOAP-ENV:Body>
  		<ns1:createWorkflowProof>
  			<SessionID>sid</SessionID>
  			<Stages>
  				<item>
  					<stage_reviewers>
  						<item>
  							<email>robert.paulson@gmail.com</email>
  						</item>
  					</stage_reviewers>
  					<name>Stage 1</name>
  				</item>
  				<item>
  					<stage_reviewers>
  						<item>
  							<email>tyler.durden@gmail.com</email>
  						</item>
  					</stage_reviewers>
  					<name>Stage 2</name>
  				</item>
  			</Stages>
  		</ns1:createWorkflowProof>
  	</SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b'\t', 1))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_indent_empty() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
  <SOAP-ENV:Body>
  <ns1:createWorkflowProof>
  <SessionID>sid</SessionID>
  <Stages>
  <item>
  <stage_reviewers>
  <item>
  <email>robert.paulson@gmail.com</email>
  </item>
  </stage_reviewers>
  <name>Stage 1</name>
  </item>
  <item>
  <stage_reviewers>
  <item>
  <email>tyler.durden@gmail.com</email>
  </item>
  </stage_reviewers>
  <name>Stage 2</name>
  </item>
  </Stages>
  </ns1:createWorkflowProof>
  </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 0))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_version_11() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.1" encoding="UTF-8" standalone="yes"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <Stages>
          <item>
            <stage_reviewers>
              <item>
                <email>robert.paulson@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 1</name>
          </item>
          <item>
            <stage_reviewers>
              <item>
                <email>tyler.durden@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 2</name>
          </item>
        </Stages>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML11, Some(Encoding::UTF8), Some(true)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_standalone_false() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="no"?>
  <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
    <SOAP-ENV:Body>
      <ns1:createWorkflowProof>
        <SessionID>sid</SessionID>
        <Stages>
          <item>
            <stage_reviewers>
              <item>
                <email>robert.paulson@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 1</name>
          </item>
          <item>
            <stage_reviewers>
              <item>
                <email>tyler.durden@gmail.com</email>
              </item>
            </stage_reviewers>
            <name>Stage 2</name>
          </item>
        </Stages>
      </ns1:createWorkflowProof>
    </SOAP-ENV:Body>
  </SOAP-ENV:Envelope>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(false)))
    .root_name("root")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}

#[test]
fn build_soap_root_name_object() {
  let object = load_json("tests/data/soap.json");
  let expected = indoc!(
    r#"
  <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
  <object>
    <SOAP-ENV:Envelope xmlns:SOAP-ENV="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ns1="https://paper-street.soap.com/">
      <SOAP-ENV:Body>
        <ns1:createWorkflowProof>
          <SessionID>sid</SessionID>
          <Stages>
            <item>
              <stage_reviewers>
                <item>
                  <email>robert.paulson@gmail.com</email>
                </item>
              </stage_reviewers>
              <name>Stage 1</name>
            </item>
            <item>
              <stage_reviewers>
                <item>
                  <email>tyler.durden@gmail.com</email>
                </item>
              </stage_reviewers>
              <name>Stage 2</name>
            </item>
          </Stages>
        </ns1:createWorkflowProof>
      </SOAP-ENV:Body>
    </SOAP-ENV:Envelope>
  </object>"#
  );

  let mut xml_builder = XmlConfig::new()
    .rendering(Indentation::new(b' ', 2))
    .decl(Declaration::new(Version::XML10, Some(Encoding::UTF8), Some(true)))
    .root_name("object")
    .finalize();
  let result = xml_builder.build_from_json(&object);

  let actual = result.expect("Error building XML");
  assert_eq!(expected, actual);
}
