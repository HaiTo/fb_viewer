// http://docs.oracle.com/cd/A60725_05/html/comnls/us/ja/jpn00020.htm
use std::string::String;

use std::slice;
use std::str;

use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub struct HeaderRecord {
    pub record_identifier: i8,
    pub classification: i8,
    pub code_type: i8,
    pub eft_requestor_id: String,
    pub account_holders_name: String,
    pub transfer_date: String,
    pub remitting_bank_number: String,
    pub remitting_bank_name: String,
    pub remitting_branch_number: String,
    pub remitting_branch_name: String,
    pub account_type: i8,
    pub account_number: i32,
    pub dummy: String
}

impl HeaderRecord {
    pub fn new(non_parsed_string: &str) -> HeaderRecord {
        HeaderRecord {
            record_identifier: 1i8, // index: 0
            classification: 21i8, // index 1..2
            code_type: 0i8, // index 3
            eft_requestor_id: gets_and_to_string(non_parsed_string, 4, 13),
            account_holders_name: non_parsed_string.get(14..43).unwrap().to_string(),
            transfer_date: non_parsed_string.get(44..47).unwrap().to_string(),
            remitting_bank_number: non_parsed_string.get(48..51).unwrap().to_string(),
            remitting_bank_name: non_parsed_string.get(52..66).unwrap().to_string(),
            remitting_branch_number: non_parsed_string.get(66..68).unwrap().to_string(),
            remitting_branch_name: non_parsed_string.get(69..83).unwrap().to_string(),
            account_type: non_parsed_string[84..84].parse().unwrap(),
            account_number: non_parsed_string[85..91].parse().unwrap(),
            dummy: "                 ".to_string(),
        }
    }

    pub fn gets_and_to_string(&self, text: &str, start: i32, end: i32) -> string {
        let chars = text.chars();
        let mut collections: Vec<char> = std::vec::Vec::new();

        for (index, c) in chars.enumerate() {
            if start <= index || index <= end {
                collections.push(c);
            }
        };
        return match str::from_utf8(collections).expect("error")
    }
}

// impl HeaderRecord
impl HeaderRecord {
    pub fn to_table_style_array(&self) ->Vec<Row> {
        vec![
            row!["区分".to_string(), self.record_identifier.to_string()],
            row!["種別".to_string(), self.classification.to_string()],
            row!["コード区分".to_string(), self.code_type.to_string()],
            row!["会社コード".to_string(), self.eft_requestor_id.to_string()],
            row!["会社名".to_string(), self.account_holders_name.to_string()],
            row!["振込指定日".to_string(), self.transfer_date.to_string()],
            row!["仕向銀行番号".to_string(), self.remitting_bank_number.to_string()],
            row!["仕向銀行名".to_string(), self.remitting_bank_name.to_string()],
            row!["仕向支店番号".to_string(), self.remitting_branch_number.to_string()],
            row!["仕向支店名".to_string(), self.remitting_branch_name.to_string()],
            row!["預金種目".to_string(), self.account_type.to_string()],
            row!["口座番号".to_string(), self.account_number.to_string()],
            row!["ダミー".to_string(), self.dummy.to_string()]
        ]
    }
}
