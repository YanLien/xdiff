# HTTP request and diff tools

There're two separate CLIs provided:

- xdiff: A diff tool for comparing HTTP requests. It could be used to compare the difference between production staging or two versions of the same API.
- xreq: A tool to build HTTP requests based on predefined profiles. It could be used to replace curl/httpie for building complicated HTTP requests.

## xdiff

### Configuration

You can configure multiple profiles for xdiff. Each profile is identified by a name. Inside a profile you can define the details of the two requests (method, url, query params, request headers, request body), and also what part of the response should be skipped for comparison (currently only headers could be skipped).

```yaml
---
rust:
  request1:
    method: GET
    url: https://www.rust-lang.org/
    headers:
        user-agent: Aloha
    params:
      hello: world
  request2:
    method: GET
    url: https://www.rust-lang.org/
    params: {}
  response:
    skip_headers:
      - set-cookie
      - date
      - via
      - x-amz-cf-id
```

### How to use xdiff?

You can use `cargo install xdiff-live` to install it (need help to [install rust toolchain](https://rustup.rs/)?). Once finished you shall be able to use it.

``` trycmd
$ xdiff-live --help
Diff two http requests and compare the difference of the responses

Usage: xdiff-live <COMMAND>

Commands:
  run    Diff two API responses based on given profile
  parse  
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

``` trycmd
$ xdiff-live run --help
Diff two API responses based on given profile

Usage: xdiff-live run [OPTIONS] --profile <PROFILE>

Options:
  -p, --profile <PROFILE>            Profile Name
  -e, --extra-params <EXTRA_PARAMS>  Overrides Args. Could be used to override the query, headers, and body of the request for query params, use `-e key=value` for headers, use `-e %key=value` for body, use `-e @key=value`
  -c, --config <CONFIG>              Configuration to use
  -h, --help                         Print help

```
