# What
Firm Banking Data viewer in stdout

# Usage
```
$ cat fb.txt | cargo run
------------------------------------------------------------
 ヘッダーレコード
============================================================
 区分              1
 種別              21
 コード区分        0
 会社コード        0123456789
 会社名            ﾎｹﾞ
 振込指定日        0825
 仕向銀行番号      0001
 仕向銀行名        ﾐｽﾞﾎ
 仕向支店番号      009
 仕向支店名        ｶﾝﾀﾞｴｷﾏｴ
 預金種目          1
 口座番号          1234567
 ダミー
------------------------------------------------
 データレコード
================================================
 区分            2
 被仕向銀行番号  0005
 被仕向銀行名    ﾐﾂﾋﾞｼﾄｳｷﾖｳUFJ
 被仕向支店番号  003
 被仕向支店名    ｶﾜﾗﾏﾁ
# ...
```

or 
```
$ cargo run
# wait stdin 1 line
```

or 
```
$ cargo build
$ ./target/debug/fb_viewer
```

## Options
Let's see `cargo run -- --help` or `fb_viewer --help`

# Why
気持ちの高まり
