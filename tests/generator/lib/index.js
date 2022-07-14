const async = require("async");
const _ = require("lodash");
const fs = require("fs");
const path = require("path");
const { argv } = require("yargs")
  .usage("Usage: $0 --mod <xml|json>")
  .demandOption(["mod"]);

let mod = argv.mod;
if (mod !== "xml" && mod !== "json") {
  console.error("Usage: node index.js --mod <xml|json>");
  process.exit(1);
}

const xml = require(path.join(__dirname, "xml.js"));
const json = require(path.join(__dirname, "json.js"));

if (mod === "xml") {
  xml.generate();
} else 
if (mod === "json") {
  json.generate();
}
