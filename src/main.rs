use std::{io::{self, BufRead, BufReader, BufWriter, Write}, process::exit};

use clap::{command, Parser, ValueEnum};
use csv::Writer;
use regex::Regex;

const BUFFER_SIZE: usize = 1024 * 1024 * 16;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(long, default_value = "true")]
    header: bool,

    #[clap(short, long, default_value = "csv")]
    format: Format,

    #[clap(short, long, default_value_t = BUFFER_SIZE, help = "buffer size")]
    buffer_size: usize,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
enum Format {
    Csv,
    Tsv,
}

fn main(){
    let arg = Args::parse();

    let mut wb = csv::WriterBuilder::new();
    wb.quote_style(csv::QuoteStyle::NonNumeric);
    
    match arg.format {
        Format::Csv => {
            wb.delimiter(b',');
        }
        Format::Tsv => {
            wb.delimiter(b'\t');
        }
    }

    let r = BufReader::with_capacity(BUFFER_SIZE, io::stdin());
    let w = wb.from_writer(BufWriter::with_capacity(BUFFER_SIZE, io::stdout()));

    if let Err(e) = parse_and_write(r, w) {
        if e.kind() != io::ErrorKind::BrokenPipe {
            return;
        }
        eprintln!("Error: {}", e);
        exit(1)
    }

}


// [2024-03-19 22:04:25] 114.250.52.160 "GET /footage/mergeSnapshot/BeOBGh2aCHUhMA.jpg?iopstyle=stock05 HTTP/2.0" 
// TCP_HIT 200 28679 8611 us-stock5.xpccdn.com "https://stock.xinpianchang.com/footages/2027498/p-6/" - NONE/- 111.174.12.100 10.63.61.45:8899, 10.63.61.46 200 0 0 "Mozilla/5.0+(Macintosh;+Intel+Mac+OS+X+10_15_7)+AppleWebKit/537.36+(KHTML,+like+Gecko)+Chrome/114.0.0.0+Safari/537.36" 1710857065588 43e3f43b38193594ba86bf3c591e8a00 - 28342 28342 0 0 20000 0 0 0 https us-stock5.xpccdn.com
fn parse_and_write<W: Write>(r: impl BufRead, mut w: Writer<W>) -> io::Result<()> {
    let headers = vec![
        "date_time",
        "client_ip",
        "request_method_url_protocol",
        "hit_status",
        "response_code",
        "sent_bytes_incl_header",
        "response_delay",
        "http_host",
        "http_referer",
        "http_username",
        "origin_way_host",
        "server_ip",
        "source_ip",
        "source_response_code",
        "client_request_end",
        "source_request_end",
        "user_agent",
        "request_processing_start",
        "frontend_flow",
        "http_range",
        "sent_bytes_excl_header",
        "file_size",
        "cache_hit_bytes",
        "merged_origin_bytes",
        "internal_error_code",
        "end_status",
        "ufile_received_bytes",
        "cache_port",
        "transfer_protocol",
        "request_host",
    ];
    w.write_record(&headers)?;

    let re = Regex::new(r#"(\[.*?\])|(".*?")|([\d\.]+\:\d+,\s[\d\.]+)|(-)|(\S+)"#).unwrap();

    for line in r.lines() {
        let line = line?;
        // replace with captures_iter for speed up
        let items = re.find_iter(&line).map(|m| {
            let v = m.as_str()
                .trim_matches(|c|  c == '[' || c == ']')
                .replace(['\r', '\t', '\n'], "") 
                .replace("NONE/-", "");
            if v == "-" {
                "".to_string()
            } else {
                v.to_string()
            }
        }).collect::<Vec<_>>();
        if items.len() != headers.len() {
            
            eprintln!(
                "Error: require size: {}, get size: {}\n\n  {}\n\n  {}\n\n",
                headers.len(),
                items.len(),
                items.join(","),
                line
            );
            eprintln!("-----------------------------");
            headers.iter().zip(items.iter()).for_each(|(h, i)| {
                eprintln!("{}: {}", h, i);
            });
            eprintln!("-----------------------------");
            continue;
        }

        w.write_record(&items)?;
    }
    w.flush()
}