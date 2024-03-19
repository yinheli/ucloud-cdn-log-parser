# ucloud-cdn-log-parser

Parse ucloud cdn log to csv/tsv format with/without header, then you can use duckdb / clickhouse local / etc to analyze the log.

## Install

Download from release page, which is built by github action.

or install via cargo or build from source.

```bash
cargo install ucloud-cdn-log-parser --locked
```

## Usage

```bash
# download logs

# parse & convert to csv / parquet

zcat *.gz | \
  ucloud-cdn-log-parser > log.csv

## use duckdb
zcat *.gz | \
  ucloud-cdn-log-parser | \
  pv | \
  duckdb -c "
    copy 
      (select * from read_csv('/dev/stdin')) 
      to 'log.parquet.zst' 
      (format parquet, compression 'zstd');"

# now you can use duckdb / clickhouse local / etc to query the log
```


Analyze log with duckdb for example:

```sql
-- get top 100 client_ip by sent_bytes_incl_header in last 6 hours
select
    client_ip,
    format_bytes(sum(sent_bytes_incl_header)::bigint) sent_bytes,
    count(*) as n
from 'log.parquet.zst'
where date_time > (now() - interval 6 hours)::timestamp
group by client_ip
order by sum(sent_bytes_incl_header) desc
limit 100;

-- get top 100 request_method_url_protocol by sent_bytes_incl_header in last 6 hours
select
    request_method_url_protocol,
    format_bytes(sum(sent_bytes_incl_header)::bigint) sent_bytes,
    count(*) as n
from 'log.parquet.zst'
where date_time > (now() - interval 6 hours)::timestamp
group by request_method_url_protocol
order by sum(sent_bytes_incl_header) desc, n desc
limit 100;
```

## Reference

- ucloud log format https://docs.ucloud.cn/ucdn/guide/LOG

