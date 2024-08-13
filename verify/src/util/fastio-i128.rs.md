---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: lib/src/lib.rs
    title: lib/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: lib/src/util.rs
    title: lib/src/util.rs
  - icon: ':heavy_check_mark:'
    path: lib/src/util/fastio.rs
    title: lib/src/util/fastio.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/many_aplusb_128bit
    links:
    - https://judge.yosupo.jp/problem/many_aplusb_128bit
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb_128bit\n\
    \nuse lib::util::fastio::*;\n\nfn main() {\n    let mut io = IO::new();\n    let\
    \ n: usize = io.read();\n    for _ in 0..n {\n        let a: i128 = io.read();\n\
    \        let b: i128 = io.read();\n        io.write(a + b);\n    }\n}\n"
  dependsOn:
  - lib/src/lib.rs
  - lib/src/util/fastio.rs
  - lib/src/util.rs
  isVerificationFile: true
  path: verify/src/util/fastio-i128.rs
  requiredBy: []
  timestamp: '2024-08-13 11:21:36+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/src/util/fastio-i128.rs
layout: document
redirect_from:
- /verify/verify/src/util/fastio-i128.rs
- /verify/verify/src/util/fastio-i128.rs.html
title: verify/src/util/fastio-i128.rs
---
