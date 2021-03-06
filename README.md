# PiBench Output Parser
[![](https://meritbadge.herokuapp.com/pibench-parser)](https://crates.io/crates/pibench-parser)
[![npm version](https://badge.fury.io/js/pibench-parser.svg)](https://badge.fury.io/js/pibench-parser)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A simple parser to parse the text output from [pibench](https://github.com/wangtzh/pibench).

## Why
Current PiBench does not support json/csv output yet; it prints the output to `stdout` and `stderr`.

We sometimes need to process hundreds of such text files, thus need a programmatic way to extract the useful information.

## Plan
1. A shared library that can be called from Python. ✔️
2. A Webassembly version to support pibench-online. ✔️
3. Native rust version for pibench backend server. ✔️


## Usage (Rust)
```rust
let data = PiBenchData::from_text(text);
```

## Work with Javascript/NodeJS
```bash
npm install pibench-parser
```

```js
import { PiBenchData } from "pibench-parser";
const text = "YOUR PIBENCH RESULT";
const result = PiBenchData.from_text(text).to_js_value();
```



## Work with Python

```python
import ctypes
from ctypes import c_char_p, c_void_p, CDLL

so_file = "/path/to/libpibench_parser.so"
functions = CDLL(so_file)

def text_to_json(text):
    json_str = functions.text_to_json(text.encode("utf-8"))
    try:
        return ctypes.cast(json_str, c_char_p).value.decode("utf-8")
    finally:
        functions.free_json_str(json_str)
```