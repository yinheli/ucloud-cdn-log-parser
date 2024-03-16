use std::io::{self, BufRead};

use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r#"(\[.*?\])|(".*?")|(-)|(\S+)"#).unwrap();
    
    let stdin = io::stdin();
    let mut w = csv::WriterBuilder::new()
        .delimiter(b',')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .terminator(csv::Terminator::CRLF)
        .from_writer(io::stdout());

    let headers = vec![
        "date_time", "client_ip", "request_method_url_protocol", 
        "hit_status", "response_code", "sent_bytes_incl_header", 
        "response_delay", "http_host", "http_referer", "http_username", 
        "origin_way_host", "server_ip", "source_ip", "source_response_code", 
        "client_request_end", "source_request_end", "user_agent", 
        "request_processing_start", "frontend_flow", "http_range", 
        "sent_bytes_excl_header", "file_size", "cache_hit_bytes", 
        "merged_origin_bytes", "internal_error_code", "end_status",
        "ufile_received_bytes", "cache_port", "transfer_protocol", "request_host"
    ];

    w.write_record(&headers)?;

    for line in stdin.lock().lines() {
        let line = line?;

        let mats = re.captures_iter(&line);

        let items = mats.flat_map(|caps| {
            caps.iter()
                .skip(1)
                .filter(|v|v.is_some())
                .map(|v|{
                    v.unwrap().as_str()
                        .trim_matches(|c| c == '"' || c == '[' || c == ']')
                }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        if items.len() != headers.len() {
            eprintln!("Error: require size: {}, get size: {}\n\n  {}\n\n  {}", headers.len(), items.len(), items.join(","), line);
            continue;
        }

        w.write_record(&items)?;
    }

    w.flush()?;

    Ok(())
}
