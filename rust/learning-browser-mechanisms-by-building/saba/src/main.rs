#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use noli::*;
use saba_core::browser::Browser;
use saba_core::http::HttpResponse;

static TEST_HTTP_RESPONSE: &str = r#"HTTP/1.1 200 OK
Data: xx xx xx


<html>
<head></head>
<body>
  <h1 id="title">H1 title</h1>
  <h2 class="class">H2 title</h2>
  <p>Test text.</p>
  <p>
    <a href="example.com">Link1</a>
    <a href="example.com">Link2</a>
  </p>
</body>
</html>
"#;

fn main() -> u64 {
    let browser = Browser::new();

    let response =
        HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("failed to parse http response");
    let page = browser.borrow().current_page();
    let dom_string = page.borrow_mut().receive_response(response);

    for log in dom_string.lines() {
        println!("{log}");
    }

    0
}

entry_point!(main);
