import { parse_js_sys, parse_js_sys_js_value, parse_serde } from "species";
import fs from "fs";

const jsonString = fs.readFileSync(
  "/Users/harry/repo/species/large-file.json",
  { encoding: "utf-8" }
);

function measure(func: () => void) {
  const start = Date.now();
  for (let i = 0; i < 10; ++i) {
    func();
  }
  const end = Date.now();
  console.log(end - start);
}

measure(() => parse_js_sys(jsonString));
measure(() => {
  JSON.parse(jsonString);
});
measure(() => {
  parse_serde(jsonString);
});
measure(() => {
  parse_js_sys_js_value({ value: jsonString });
});
