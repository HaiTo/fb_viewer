use std::io;
use std::iter::Iterator;
use std::process;

// 3rd party libraries
//// prettytable
#[macro_use] extern crate prettytable;
use prettytable::Table;
use prettytable::format;

#[macro_use]
extern crate serde_derive;
extern crate docopt;
use docopt::Docopt;

const USAGE: &'static str = "\
Usage:
    fb_viewer (--help)
    fb_viewer [-h -d -t -e --no-header --no-data --no-trailer --no-end]
Options:
    --help      Show this screen.
    -h, --no-header    Dont show Header Record.
    -d, --no-data      Dont show Data Record.
    -t, --no-trailer   Dont show Trailer Record.
    -e, --no-end       Dont show End Record.
";
#[derive(Debug, Deserialize)]
struct Args {
    flag_help: bool,
    flag_no_header: bool, flag_h: bool,
    flag_no_data: bool, flag_d: bool,
    flag_no_trailer: bool, flag_t: bool,
    flag_no_end: bool, flag_e: bool,
}
impl Args {
    fn is_print_header(&self) -> bool {
        return !(self.flag_no_header || self.flag_h)
    }
    fn is_print_data(&self) -> bool {
        return !(self.flag_no_data || self.flag_d)
    }
    fn is_print_trailer(&self) -> bool {
        return !(self.flag_no_trailer || self.flag_t)
    }
    fn is_print_end(&self) -> bool {
        return !(self.flag_no_end || self.flag_e)
    }
}


/// char_iter を進めながら size 個の文字を返す
/// char_iter を進めてしまうので注意する
/// # Panics
/// iter の残数を超えて size を指定すると読み込めない。
/// どう表示するのか考えてないので落とす。本当は Result<String, None> を返す？
fn take_str(char_iter: &mut std::str::Chars, size: i32) -> String {
    let mut buffer = String::new();
    for _ in 0..size {
        let c = match char_iter.nth(0) {
            Some(x) => x,
            None => panic!("Load string Nothing! inputted string is shorty")
        };
        buffer.push(c);
    }
    return buffer
}

fn print_usage(message: String) {
    print!("{:?}", message);
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        print_usage(USAGE.to_string());
        process::exit(0);
    }

    // read_to_string はなぜか mutably でなければならない
    let stdin = io::stdin();
    // 読み込んだものを詰め込むbuffer
    let mut buffer: String = String::new();

    stdin
        .read_line(&mut buffer)
        .expect("ERROR can't reading!");

    let mut chars = buffer.chars();

    let build_format = format::FormatBuilder::new()
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('=', '+', '+', '+')
        )
        .separator(
            format::LinePosition::Top,
            format::LineSeparator::new('-', '+', '+', '+')
        )
        .build();
    
    let mut header_record_table: Table = Table::new();
    header_record_table.set_format(build_format);
    header_record_table.set_titles(row!["ヘッダーレコード"]);
    header_record_table.add_row(row!["区分", take_str(&mut chars, 1)]);
    header_record_table.add_row(row!["種別", take_str(&mut chars, 2)]);
    header_record_table.add_row(row!["コード区分", take_str(&mut chars, 1)]);
    header_record_table.add_row(row!["会社コード", take_str(&mut chars, 10)]);
    header_record_table.add_row(row!["会社名", take_str(&mut chars, 40)]);
    header_record_table.add_row(row!["振込指定日", take_str(&mut chars, 4)]);
    header_record_table.add_row(row!["仕向銀行番号", take_str(&mut chars, 4)]);
    header_record_table.add_row(row!["仕向銀行名", take_str(&mut chars, 15)]);
    header_record_table.add_row(row!["仕向支店番号", take_str(&mut chars, 3)]);
    header_record_table.add_row(row!["仕向支店名", take_str(&mut chars, 15)]);
    header_record_table.add_row(row!["預金種目", take_str(&mut chars, 1)]);
    header_record_table.add_row(row!["口座番号", take_str(&mut chars, 7)]);
    header_record_table.add_row(row!["ダミー", take_str(&mut chars, 17)]);
    if args.is_print_header() {
        header_record_table.printstd();
    }

    loop {
        let identifier: char = match chars.nth(0) {
            Some(i) => i,
            None => {
                panic!("文字数が足りないよ＞＜；");
            }
        };

        if identifier == '2' {
            // data record の生成と挿入
            let mut table: Table = Table::new();
            table.set_format(build_format);
            table.set_titles(row!["データレコード"]);
            // すでに nth でiterを進めてしまっているので、
            // take_str を使わないで一つ目のレコードを挿入しなければならない
            table.add_row(row!["区分", identifier]);
            table.add_row(row!["被仕向銀行番号", take_str(&mut chars, 4)]);
            table.add_row(row!["被仕向銀行名", take_str(&mut chars, 15)]);
            table.add_row(row!["被仕向支店番号",  take_str(&mut chars, 3)]);
            table.add_row(row!["被仕向支店名", take_str(&mut chars, 15)]);
            table.add_row(row!["手形交換所番号",  take_str(&mut chars, 4)]);
            table.add_row(row!["預金種目",  take_str(&mut chars, 1)]);
            table.add_row(row!["口座番号",  take_str(&mut chars, 7)]);
            table.add_row(row!["受取人名", take_str(&mut chars, 30)]);
            table.add_row(row!["振込金額", take_str(&mut chars, 10)]);
            table.add_row(row!["新規コード",  take_str(&mut chars, 1)]);
            table.add_row(row!["ＥＤＩ情報", take_str(&mut chars, 20)]);
            table.add_row(row!["振込指定区分",  take_str(&mut chars, 1)]);
            table.add_row(row!["識別表示",  take_str(&mut chars, 1)]);
            table.add_row(row!["ダミー",  take_str(&mut chars, 7)]);

            if args.is_print_data() {
                table.printstd();
            }
        } else {
            // trailer record の作成
            // trailer record は一つしか無いので else で入って break する
            let mut table: Table = Table::new();
            table.set_format(build_format);
            table.set_titles(row!["トレーラレコード"]);
            table.add_row(row!["区分", identifier]);
            table.add_row(row!["合計件数", take_str(&mut chars, 6)]);
            table.add_row(row!["合計金額", take_str(&mut chars, 12)]);
            table.add_row(row!["ダミー", take_str(&mut chars, 101)]);
            if args.is_print_trailer() {
                table.printstd();
            }
            
            break;
        };
    }
    let mut table: Table = Table::new();
    table.set_format(build_format);
    table.set_titles(row!["エンドレコード"]);
    table.add_row(row!["区分", take_str(&mut chars, 1)]);
    table.add_row(row!["ダミー", take_str(&mut chars, 119)]);

    if args.is_print_end() {
        table.printstd();
    }
}
