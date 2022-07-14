const _ = require("lodash");
const xml2js = require("xml2js");
const fs = require("fs");
const path = require("path");
const async = require("async");
const crypto = require("crypto");

const defaultOptionValues = {
  charkey: '_',
  attrkey: '$',
  emptyTag: '',
  explicitRoot: true,
  explicitCharkey: false,
  trim: false,
  ignoreAttrs: false,
  mergeAttrs: false,
  normalize: false,
  normalizeTags: false,
  explicitArray: true
};

const variations = {
  charkey: ['c', "charkey"],
  attrkey: ['a', "attrkey"],
  emptyTag: ['e', "empty"],
  explicitRoot: [false],
  explicitCharkey: [true],
  trim: [true],
  ignoreAttrs: [true],
  mergeAttrs: [true],
  normalize: [true],
  normalizeTags: [true],
  explicit_array: [true]
};

const mapVname = (name) => {
  switch(name) {
    case true:
      return "";
      break;
    case "":
      return "empty";
      break;
    default:
      return name;
  }
}

const getOptionVariations = (fileName) => {
  let fname = path.parse(fileName).base.replace(".xml", "").replace(".json", "");
  let defaultName = ["build", fname, "default"].join("_");

  let optionVariations = [_.merge(defaultOptionValues, {
    name: defaultName,
    fname: fname
  })];

  _.each(variations, (arr, key) => {
    _.each(arr, (value) => {
      let options = _.clone(defaultOptionValues);
      let vname = _.toString(value).replace(" ", "space").replace("\t", "tab").replace(".", "");
      vname = mapVname(vname);
      let name = ["build", fname, _.snakeCase(key), vname].join("_");
      options[key] = value;
      optionVariations.push(_.merge(options, {
        name: name,
        fname: fname
      }));
    });
  });
  return optionVariations;
};

const writeTest = (input, output) => {
  console.log("#[test]")
  console.log(`fn ${input.name}() {`);
  console.log(`  let xml = load_xml("tests/data/${input.fname}.xml");`);
  console.log(`  let expected: JsonValue = serde_json::from_str(r#"${JSON.stringify(output)}"#).unwrap();`);
  console.log("");
  console.log("  let json_builder = JsonConfig::new()");
  if (input.attrkey !== defaultOptionValues.attrkey)
    console.log(`    .attrkey("${input.attrkey}")`);
  if (input.charkey !== defaultOptionValues.charkey)
    console.log(`    .charkey("${input.charkey}")`);
  if (input.emptyTag !== defaultOptionValues.emptyTag)
    console.log(`    .empty_tag("${input.emptyTag}")`);
  if (input.explicitRoot !== defaultOptionValues.explicitRoot)
    console.log(`    .explicit_root(${input.explicitRoot})`);
  if (input.explicitCharkey !== defaultOptionValues.explicitCharkey)
    console.log(`    .explicit_charkey(${input.explicitCharkey})`);
  if (input.trim !== defaultOptionValues.trim)
    console.log(`    .trim(${input.trim})`);
  if (input.ignoreAttrs !== defaultOptionValues.ignoreAttrs)
    console.log(`    .ignore_attrs(${input.ignoreAttrs})`);
  if (input.mergeAttrs !== defaultOptionValues.mergeAttrs)
    console.log(`    .merge_attrs(${input.mergeAttrs})`);
  if (input.normalize !== defaultOptionValues.normalize)
    console.log(`    .normalize_text(${input.normalize})`);
  if (input.normalizeTags !== defaultOptionValues.normalizeTags)
    console.log(`    .lowercase_tags(${input.normalizeTags})`);
  if (input.explicitArray !== defaultOptionValues.explicitArray)
    console.log(`    .explicit_array(${input.explicitArray})`);
  console.log("    .finalize();");
  console.log("  let result = json_builder.build_from_xml(&xml);");
  console.log("");
  console.log(`  let actual = result.expect("Error building JSON.");`);
  console.log("  assert_eq!(expected, actual);");
  console.log("}");
};

module.exports = {

  generate: () => {
    const dataDir = path.join(__dirname, "../../data");

    const xmlFiles = _.filter(fs.readdirSync(dataDir), (file) => {
      return _.last(file.split(".")) === "xml";
    });

    async.forEach(xmlFiles, (fileName, nextOuter) => {
      const data = fs.readFileSync(path.join(dataDir, fileName)).toString();
      const variations = getOptionVariations(fileName);
      let hashes = new Set();

      async.forEach(variations, (options, nextInner) => {
        let parser = new xml2js.Parser(_.omit(options, ["name", "fname"]));
        parser.parseString(data, (err, result) => {
          if (err) {
            throw err;
          }

          // Hash the results, check agains the hashes set to see if we've seen the output before
          // Only produce tests for unique JSON output strings. Making the assumption that if an option
          // (such as lowercase_tags) does not create an output different from others (such as the default)
          // then we want to skip "writing" tests for it.
          const hash = crypto.createHash("sha256");
          hash.update(JSON.stringify(result));
          let jsonHash = hash.digest("hex");
          if (hashes.has(jsonHash)) {
            return nextInner();
          } else {
            hashes.add(jsonHash);
          }

          console.log("");
          writeTest(options, result);
          console.log("");

          return nextInner();
        });
      }, nextOuter);

    });

  }

};
