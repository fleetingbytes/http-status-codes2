# http-status-codes2

Provides a representation of [IANA’s HTTP Status Code Registry][ianareg] and search functions to query it.

Also has an unofficial status code registry whech includes a few proposed or popular unofficial status codes.

A CLI is provided by the [heman][hemancrate] crate.

Usage exaple for the official registry:

```rust
use http_status_codes2::{find_by_code, status_code_registry::CODE_REGISTRY},

assert_eq!(
    find_by_code(100, &CODE_REGISTRY),
    Some((
        100,
        "Continue",
        "[RFC9110, Section 15.2.1]",
        "https://www.rfc-editor.org/rfc/rfc9110.html#section-15.2.1"
    ))
);
assert_eq!(find_by_code(600, &CODE_REGISTRY), None);
```

Usage exaple for the unofficial registry:

```rust
use http_status_codes2::{find_by_substring, status_code_registry::UNOFFICIAL_CODE_REGISTRY},

let mut it = find_by_substring("teapot", &UNOFFICIAL_CODE_REGISTRY);
assert_eq!(
    it.next(),
    Some((
        418,
        "I'm a teapot",
        "[RFC2324, Section 2.3.2]",
        "https://www.rfc-editor.org/rfc/rfc2324.html#section-2.3.2"
    ))
    .as_ref()
);
assert_eq!(it.next(), None);
```


[ianareg]: https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml
[hemancrate]: https://crates.io/crates/heman
