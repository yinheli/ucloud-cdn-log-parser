# ucloud-cdn-log-parser

Parse ucloud cdn log to csv format with header, then you can use duckdb / clickhouse local / etc to analyze the log.

## Install

Download from release page, which is built by github action.

or install via cargo or build from source.

```bash
cargo install ucloud-cdn-log-parser --locked
```

## Usage

```bash
# download logs

# parse & convert to parquet
zcat *.gz | \
  ucloud-cdn-log-parser | \
  duckdb -c "copy (select * from read_csv('/dev/stdin')) to 'log.parquet' (format parquet)"

# now you can use duckdb / clickhouse local / etc to query the log
```

## Reference

- ucloud log format https://docs.ucloud.cn/ucdn/guide/LOG

