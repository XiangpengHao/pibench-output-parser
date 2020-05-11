# PiBench Result Parser

A simple parser to parse the text output from [pibench](https://github.com/wangtzh/pibench).

## Why
Current PiBench does not support json/csv output yet; it prints the output to `stdout` and `stderr`.

We sometimes need to process hundreds of such text files, thus need a programmatic way to extract the useful information.

## Plan
1. A shared library that can be called from Python
2. A Webassembly version to support pibench-online.
3. Native rust version for pibench backend server.
