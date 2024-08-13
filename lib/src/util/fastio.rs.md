---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: lib/src/lib.rs
    title: lib/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: lib/src/util.rs
    title: lib/src/util.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: lib/src/lib.rs
    title: lib/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: lib/src/util.rs
    title: lib/src/util.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/src/util/fastio-i128.rs
    title: verify/src/util/fastio-i128.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.10.14/x64/lib/python3.10/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "\nuse std::io::prelude::*;\nuse std::io::StdinLock;\nconst BUFFER_SIZE: usize\
    \ = 1 << 20;\nconst CH_0: u8 = b'0' as u8;\nconst CH_9: u8 = b'9' as u8;\n\npub\
    \ trait Reader<T> {\n    fn read(&mut self) -> T;\n}\npub struct IO<'a> {\n  \
    \  reader: StdinLock<'a>,\n    buffer: [u8; BUFFER_SIZE],\n    index: usize,\n\
    \    writer: std::io::BufWriter<std::io::StdoutLock<'a>>,\n}\nimpl<'a> IO<'a>\
    \ {\n    pub fn new() -> Self {\n        let stdin = std::io::stdin();\n     \
    \   let stdout = std::io::stdout();\n        let writer = std::io::BufWriter::new(stdout.lock());\n\
    \        let io = IO {\n            reader: stdin.lock(),\n            buffer:\
    \ [0; BUFFER_SIZE],\n            index: BUFFER_SIZE,\n            writer,\n  \
    \      };\n        return io;\n    }\n    fn get_any_char(&mut self) -> u8 {\n\
    \        if self.index == self.buffer.len() {\n            self.reader.read(&mut\
    \ self.buffer).unwrap();\n            self.index = 0;\n        }\n        let\
    \ ch = self.buffer[self.index];\n        self.index += 1;\n        return ch;\n\
    \    }\n    fn get_notempty_char(&mut self) -> u8 {\n        let mut ch = self.get_any_char();\n\
    \        while ch == b' ' || ch == b'\\n' || ch == b'\\r' || ch == b'\\t' {\n\
    \            ch = self.get_any_char();\n        }\n        return ch;\n    }\n\
    \    fn get_integer_first_char(&mut self) -> (bool, u8) {\n        let mut ch\
    \ = self.get_notempty_char();\n        let is_negative = ch == b'-';\n       \
    \ if is_negative {\n            ch = self.get_any_char();\n        }\n       \
    \ return (is_negative, ch);\n    }\n    pub fn write<T: std::fmt::Display>(&mut\
    \ self, x: T) {\n        writeln!(self.writer, \"{}\", x).unwrap();\n    }\n}\n\
    impl Reader<usize> for IO<'_> {\n    fn read(&mut self) -> usize {\n        let\
    \ mut ch = self.get_notempty_char();\n        let mut x = 0;\n        while CH_0\
    \ <= ch && ch <= CH_9 {\n            x = x * 10 + (ch - CH_0) as usize;\n    \
    \        ch = self.get_any_char();\n        }\n        return x;\n    }\n}\nimpl\
    \ Reader<i32> for IO<'_> {\n    fn read(&mut self) -> i32 {\n        let (is_negative,\
    \ mut ch) = self.get_integer_first_char();\n        let mut x: i32 = 0;\n    \
    \    while CH_0 <= ch && ch <= CH_9 {\n            x = x * 10 + (ch - CH_0) as\
    \ i32;\n            ch = self.get_any_char();\n        }\n        return if is_negative\
    \ { -x } else { x };\n    }\n}\nimpl Reader<i128> for IO<'_> {\n    fn read(&mut\
    \ self) -> i128 {\n        let (is_negative, mut ch) = self.get_integer_first_char();\n\
    \        let mut x: i128 = 0;\n        while CH_0 <= ch && ch <= CH_9 {\n    \
    \        x = x * 10 + (ch - CH_0) as i128;\n            ch = self.get_any_char();\n\
    \        }\n        return if is_negative { -x } else { x };\n    }\n}\n"
  dependsOn:
  - lib/src/lib.rs
  - lib/src/util.rs
  isVerificationFile: false
  path: lib/src/util/fastio.rs
  requiredBy:
  - lib/src/lib.rs
  - lib/src/util.rs
  timestamp: '2024-07-08 02:22:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/src/util/fastio-i128.rs
documentation_of: lib/src/util/fastio.rs
layout: document
redirect_from:
- /library/lib/src/util/fastio.rs
- /library/lib/src/util/fastio.rs.html
title: lib/src/util/fastio.rs
---
