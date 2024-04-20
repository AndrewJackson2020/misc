# Introduction

Following commands show how to use this repo with bazel's py_runtime rule.

Use the system-wide python interpreter.

``` shell
$ bazel run :bin
INFO: Analysed target //:bin (8 packages loaded).
INFO: Found 1 target...
Target //:bin up-to-date:
  bazel-bin/bin
INFO: Elapsed time: 3.168s, Critical Path: 0.02s
INFO: Build completed successfully, 4 total actions

INFO: Running command line: bazel-bin/bin
*****************************************
* The Python version running is : 2.7.6 *
*****************************************
```

