//! Client Requests

use url::Url;

use method::Method;
use header::Headers;
use header::Host;
use net::{NetworkConnector, DefaultConnector};
use version::HttpVersion;
use client::{get_host_and_port};



/// A client request to a remote server.
pub struct Request {
    method: Method,
    url: Url,
    version: HttpVersion,
    headers: Headers,
}

impl Request {
    /// Read the Request Url.
    #[inline]
    pub fn url(&self) -> &Url { &self.url }

    /// Readthe Request Version.
    #[inline]
    pub fn version(&self) -> &HttpVersion { &self.version }

    /// Read the Request headers.
    #[inline]
    pub fn headers(&self) -> &Headers { &self.headers }

    /// Read the Request method.
    #[inline]
    pub fn method(&self) -> &Method { &self.method }

    /// Get a mutable reference to the Request headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut Headers { &mut self.headers }

    /*
    /// Set the write timeout.
    #[inline]
    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.message.set_write_timeout(dur)
    }

    /// Set the read timeout.
    #[inline]
    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        self.message.set_read_timeout(dur)
    }
    */
}

/// Create a new client request.
pub fn new(method: Method, url: Url) -> ::Result<Request> {
    let (host, port) = try!(get_host_and_port(&url));
    let mut headers = Headers::new();
    headers.set(Host {
        hostname: host,
        port: Some(port),
    });

    Ok(Request {
        method: method,
        headers: headers,
        url: url,
        version: HttpVersion::Http11,
    })
}

#[cfg(test)]
mod tests {
    /*
    use std::io::Write;
    use std::str::from_utf8;
    use url::Url;
    use method::Method::{Get, Head, Post};
    use mock::{MockStream, MockConnector};
    use net::Fresh;
    use header::{ContentLength,TransferEncoding,Encoding};
    use url::form_urlencoded;
    use super::Request;
    use http::h1::Http11Message;

    fn run_request(req: Request<Fresh>) -> Vec<u8> {
        let req = req.start().unwrap();
        let message = req.message;
        let mut message = message.downcast::<Http11Message>().ok().unwrap();
        message.flush_outgoing().unwrap();
        let stream = *message
            .into_inner().downcast::<MockStream>().ok().unwrap();
        stream.write
    }

    fn assert_no_body(s: &str) {
        assert!(!s.contains("Content-Length:"));
        assert!(!s.contains("Transfer-Encoding:"));
    }

    #[test]
    fn test_get_empty_body() {
        let req = Request::with_connector(
            Get, Url::parse("http://example.dom").unwrap(), &mut MockConnector
        ).unwrap();
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert_no_body(s);
    }

    #[test]
    fn test_head_empty_body() {
        let req = Request::with_connector(
            Head, Url::parse("http://example.dom").unwrap(), &mut MockConnector
        ).unwrap();
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert_no_body(s);
    }

    #[test]
    fn test_url_query() {
        let url = Url::parse("http://example.dom?q=value").unwrap();
        let req = Request::with_connector(
            Get, url, &mut MockConnector
        ).unwrap();
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(s.contains("?q=value"));
    }

    #[test]
    fn test_post_content_length() {
        let url = Url::parse("http://example.dom").unwrap();
        let mut req = Request::with_connector(
            Post, url, &mut MockConnector
        ).unwrap();
        let body = form_urlencoded::serialize(vec!(("q","value")).into_iter());
        req.headers_mut().set(ContentLength(body.len() as u64));
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(s.contains("Content-Length:"));
    }

    #[test]
    fn test_post_chunked() {
        let url = Url::parse("http://example.dom").unwrap();
        let req = Request::with_connector(
            Post, url, &mut MockConnector
        ).unwrap();
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(!s.contains("Content-Length:"));
    }

    #[test]
    fn test_post_chunked_with_encoding() {
        let url = Url::parse("http://example.dom").unwrap();
        let mut req = Request::with_connector(
            Post, url, &mut MockConnector
        ).unwrap();
        req.headers_mut().set(TransferEncoding(vec![Encoding::Chunked]));
        let bytes = run_request(req);
        let s = from_utf8(&bytes[..]).unwrap();
        assert!(!s.contains("Content-Length:"));
        assert!(s.contains("Transfer-Encoding:"));
    }

    #[test]
    fn test_write_error_closes() {
        let url = Url::parse("http://hyper.rs").unwrap();
        let req = Request::with_connector(
            Get, url, &mut MockConnector
        ).unwrap();
        let mut req = req.start().unwrap();

        req.message.downcast_mut::<Http11Message>().unwrap()
            .get_mut().downcast_mut::<MockStream>().unwrap()
            .error_on_write = true;

        req.write(b"foo").unwrap();
        assert!(req.flush().is_err());

        assert!(req.message.downcast_ref::<Http11Message>().unwrap()
            .get_ref().downcast_ref::<MockStream>().unwrap()
            .is_closed);
    }
    */
}
