use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use chrono::prelude::*;

pub struct ResultType  {
    pub timestamp: String,
    pub length: String,
    pub jmeter_task: String,
    pub http_code: String,
    pub thread_group: String,
    pub data_type: String,
    pub success: String,
    pub fail_message: String,
    pub bytes: String,
    pub sent_bytes: String,
    pub grp_threads: String,
    pub all_threads: String,
    pub url: String,
    pub latency: String,
    pub idle_time: String,
    pub connect: String,
}

fn main() {
    let path = "/home/phiro/Dropbox/Projects/kubernetes/CKAD/training/6/scratch/testresults.jtl";
    let mut counter = 0;
    let mut ok = 0;
    let mut nok = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut cells = ip.split(",");
                //Get the timestamp that jmeter prints
                let timestamp = cells.next().unwrap();
                // Skip the fields we do not care about.
                let _ = cells.next().unwrap();
                let _ = cells.next().unwrap();
                //Get the result
                let result  = cells.next().unwrap();
                counter += 1;
                // An HTTP result of 200 is success, all other failure
                if "200".eq(result) {
                    ok += 1;
                } else {
                    nok += 1;
                    let date = timestamp_to_date(timestamp.to_string());
                    println!("--- error:: number: {} , timestamp: {} - date: {} , result: {}",counter, timestamp, date, result);
                }
                if counter % 1000000 == 0 {
                    let date = timestamp_to_date(timestamp.to_string());
                    println!("number: {} , timestamp: {} - date: {}, ok: {}, nok: {}",counter, timestamp, date, ok, nok);
                    println!("{}", " ---- ");
                }



            }
        }
        println!("Total records processed: {} , ok: {}, nok: {}",counter,  ok, nok);
    }



}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn timestamp_to_date(timestamp: String)-> String {
    let date_set = timestamp.parse::<i64>().unwrap_or(0);
    let naive = NaiveDateTime::from_timestamp_millis(date_set).unwrap_or_default();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let now = Local::now();
    let offset = now.offset();
    let tzname = offset.to_string();
    let format = format!("%Y-%m-%d %H:%M:%S {} - %s",tzname).to_string();
    let newdate = datetime.with_timezone(&Local).format(&*format);
    return newdate.to_string();
}