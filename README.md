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

### examples

测试样例：
```
$ cargo run --bin xdiff-live run -p rust -c  fixtures/test.yml -e a=100 -e @b=2 -e m=10
```
测试样例：
```
$ cargo run --bin xdiff-live -- parse
$ https://jsonplaceholder.typicode.com/todos/1?a=1&b=2
$ https://jsonplaceholder.typicode.com/todos/2?a=2&b=3
$ todo
```
测试样例：cargo run --bin xreq-live -- run -p todo -c fixtures/xreq_test.yml


