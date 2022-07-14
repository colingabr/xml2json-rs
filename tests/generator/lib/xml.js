const _ = require("lodash");
const xml2js = require("xml2js");
const fs = require("fs");
const path = require("path");
const async = require("async");
const crypto = require("crypto");

const defaultOptionValues = {
  pretty: true,
  indent: "  ",
  newline: "\n",
  version: "1.0",
  encoding: "UTF-8",
  standalone: true,
  rootName: "root",
  doctype: null,
  headless: true
};

const variations = {
  pretty: [false],
  indent: ["\t", ""],
  newline: [],
  version: ["1.1"],
  encoding: [],
  standalone: [false],
  rootName: ["object"],
  doctype: []
};

const nestedOptsFromFlat = (opts) => {
  return {
    renderOptions: {
      pretty: opts.pretty,
      indent: opts.indent,
      newline: opts.newline
    },
    xmldec: {
      version: opts.version,
      encoding: opts.encoding,
      standalone: opts.standalone
    },
    options: {
      rootName: opts.rootName,
      doctype: opts.doctype
    }
  }
}

const getOptionVariations = (fileName) => {
  let fname = path.parse(fileName).base.replace(".json", "").replace(".xml", "");
  let name = ["build", fname, "default"].join("_");

  let optionVariations = [_.merge({
    name: name,
    fname: fname,
  }, nestedOptsFromFlat(defaultOptionValues))];

  _.each(variations, (arr, key) => {
    _.each(arr, (value) => {
      let options = _.clone(defaultOptionValues);
      let vname = _.toString(value).replace(" ", "space").replace("\t", "tab").replace(".", "");
      if (vname === "") {
        vname = "empty";
      }
      let name = ["build", fname, _.snakeCase(key), vname].join("_");
      options[key] = value;
      optionVariations.push(_.merge({
        name: name,
        fname: fname
      }, nestedOptsFromFlat(options)));
    });
  });
  return optionVariations;
};

const printRustRender = (state) => {
  if(_.keys(state.render).length === 3) {
    if(state.render.pretty) {
      let len = state.render.indent.length;
      let ch = _.first(state.render.indent.split(""));
      if (!ch) {
        ch = " ";
        len = 0;
      }
      if(ch == "\t") {
        ch = "\\t";
      }
      console.log("    .rendering(Indentation::new(b'" + ch + "',", len + "))");
    }
  }
};

const encodingToRust = (str) => {
  if (str === null) {
    return "None";
  } else {
    if (str === "UTF8" || str === "UTF-8") {
      return "Some(Encoding::UTF8)";
    }
    if (str === "ASCII") {
      return "Some(Encoding::ASCII)";
    }
    return "Some(Encoding::" + str + ")";
  }
}

const versionToRust = (str) => {
  if (!str) {
    return null;
  }
  if (str === "1.0") {
    return "Version::XML10";
  }
  if (str === "1.1") {
    return "Version::XML11";
  }
}

const printRustDecl = (state) => {
  if(_.keys(state.decl).length === 3) {
    let encoding = encodingToRust(state.decl.encoding);
    let version = versionToRust(state.decl.version);
    let standalone = "None"
    if(state.decl.standalone === true) {
      standalone = "Some(" + true + ")";
    } else {
      standalone = "Some(" + false + ")";
    }
    console.log("    .decl(Declaration::new(" + version + ",", encoding + ",", standalone + "))");
  }
};

const printRustOption = (state, key, value) => {
  switch(key) {
    case "pretty":
      state.render.pretty = value;
      printRustRender(state);
      break;
    case "indent":
      state.render.indent = value;
      printRustRender(state);
      break;
    case "newline":
      state.render.newline = value;
      printRustRender(state);
      break;
    case "version":
      state.decl.version = value;
      printRustDecl(state);
      break;
    case "encoding":
      state.decl.encoding = value;
      printRustDecl(state);
      break;
    case "standalone":
      state.decl.standalone = value;
      printRustDecl(state);
      break;
    case "rootName":
      console.log("    .root_name(\"" + value + "\")");
      break;
    case "doctype":
      //state.opts.doctype = value;
      //console.log("    .doctype(\"" + value + "\")");
      break;
    default:
      console.error("bad option");
      process.exit(1)
  }
}

module.exports = {
  generate: () => {
    const dataDir = path.join(__dirname, "../../data");

    const jsonFiles = _.filter(fs.readdirSync(dataDir), (file) => {
      return _.last(file.split(".")) === "json";
    });

    async.forEach(jsonFiles, (fileName, nextOuter) => {
      const data = require(path.join(dataDir, fileName));
      let variations = getOptionVariations(fileName);
      let hashes = new Set();
      async.forEach(variations, (input, nextInner) => {
        let options = _.clone(input);
        let builder = new xml2js.Builder({
          rootName: input.options.rootName,
          renderOpts: {
            pretty: input.renderOptions.pretty,
            indent: input.renderOptions.indent,
            newline: input.renderOptions.newline
          },
          xmldec: {
            version: input.xmldec.version,
            encoding: input.xmldec.encoding,
            standalone: input.xmldec.standalone
          },
          doctype: input.options.doctype,
          headless: input.options.headless
        });

        let result = builder.buildObject(data);
        const hash = crypto.createHash("sha256");
        hash.update(result);
        let xmlHash = hash.digest("hex");
        if(hashes.has(xmlHash)) {
          console.error("Skipping", input.name);
          return nextInner();
        } else {
          hashes.add(xmlHash);
        }

        console.log("#[test]");
        console.log(`fn ${input.name}() {`);
        console.log(`  let object = load_json("tests/data/${input.fname}.json");`);
        if(options.renderOptions.pretty === false) {
          console.log(`  let expected = "${result.split("\"").join("\\\"")}" ;`);
        } else {
          console.log("  let expected = indoc!(r#\"");
          console.log("  " + result.split("\n").join("\n  ") + "\"#);");
        }
        console.log("");
        console.log("  let mut xml_builder = XmlConfig::new()");

        let flat = _.merge({}, options.renderOptions, options.xmldec, options.options);
        let state = { decl: {}, opts: {}, render: {} };
        _.each(flat, (v, k) => {
          printRustOption(state, k, v);
        });
        console.log("    .finalize();");

        console.log("  let result = xml_builder.build_from_json(&object);");
        console.log("");
        console.log("  let actual = result.expect(\"Error building XML\");");
        console.log("  assert_eq!(expected, actual);");
        console.log("}");
        console.log("");
        nextInner();
      }, nextOuter);

    });
  }
}

