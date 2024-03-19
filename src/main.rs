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
    CSV,
    TSV,
}

fn main(){
    let arg = Args::parse();

    let mut wb = csv::WriterBuilder::new();
    wb.quote_style(csv::QuoteStyle::NonNumeric);
    
    match arg.format {
        Format::CSV => {
            wb.delimiter(b',');
        }
        Format::TSV => {
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

    let re = Regex::new(r#"(\[.*?\])|(".*?")|(-)|(\S+)"#).unwrap();

    for line in r.lines() {
        let line = line?;
        // replace with captures_iter for speed up
        let items = re.find_iter(&line).map(|m| {
            let v = m.as_str()
                .trim_matches(|c|  c == '[' || c == ']')
                .replace("\r", "")
                .replace("\t", "")
                .replace("\n", "") 
                .replace("NONE/-", "");
            if v == "-" {
                "".to_string()
            } else {
                v.to_string()
            }
        }).collect::<Vec<_>>();
        if items.len() != headers.len() {
            eprintln!(
                "Error: require size: {}, get size: {}\n\n  {}\n\n  {}",
                headers.len(),
                items.len(),
                items.join(","),
                line
            );
            continue;
        }

        w.write_record(&items)?;
    }
    w.flush()
}