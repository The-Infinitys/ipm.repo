use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use url::Url as ExternalUrl; // `url` crate's Url type is aliased to avoid name collision

/// URLの解析と操作のための構造体
pub struct Url {
    // Internally store the `url` crate's Url object
    inner: ExternalUrl,
}

impl Url {
    /// 新しいURLオブジェクトを作成します。
    /// 文字列からURLをパースします。
    pub fn new(url_string: &str) -> Result<Self, String> {
        let inner = ExternalUrl::parse(url_string)
            .map_err(|e| format!("URL parsing error: {}", e))?;
        Ok(Url { inner })
    }

    /// URLのプロトコルを取得します (例: "https:").
    pub fn protocol(&self) -> String {
        // `url` crate's scheme() returns "http" or "https", add ":" for JS compatibility
        format!("{}:", self.inner.scheme())
    }

    /// URLのホスト名を取得します (ポート番号なし).
    pub fn hostname(&self) -> Option<&str> {
        self.inner.host_str()
    }

    /// URLのホストを取得します (ポート番号を含む可能性あり).
    pub fn host(&self) -> Option<String> {
        // JavaScript's URL.host property returns the hostname and port (if non-default)
        // or just the hostname (if port is default or not present).
        // `url` crate's `domain()` method (which includes subdomain) is more like hostname here.
        // `url` crate's `port()` returns `None` for default ports *unless explicitly specified*.
        // `url` crate's `port_or_known_default()` returns the port even if not explicit.
        
        let host_str = self.inner.host_str()?; // Get hostname string

        if let Some(port) = self.inner.port() {
            // Port is explicitly specified and non-default (e.g., :8080)
            Some(format!("{}:{}", host_str, port))
        } else if self.inner.port_or_known_default().is_some() && self.inner.port().is_none() {
            // Port is default but not explicitly specified (e.g., http://example.com)
            // In this case, JS URL.host does not include the port.
            Some(host_str.to_string())
        } else {
            // No port at all or it's an opaque origin (should be handled by host_str())
            Some(host_str.to_string())
        }
    }


    /// URLのポート番号を取得します。
    pub fn port(&self) -> Option<u16> {
        self.inner.port() // `url` crate returns u16 for ports, None for default not specified
    }

    /// URLのパスを取得します (例: "/path/to/resource").
    pub fn pathname(&self) -> &Path {
        Path::new(self.inner.path())
    }

    /// URLのクエリ文字列を取得します (例: "?key1=value1&key2=value2").
    pub fn search(&self) -> String {
        self.inner.query().map_or_else(String::new, |q| format!("?{}", q))
    }

    /// URLのハッシュ（フラグメント）を取得します (例: "#section1").
    pub fn hash(&self) -> Option<&str> {
        self.inner.fragment()
    }

    /// URL全体を文字列として構築します (href).
    pub fn href(&self) -> String {
        self.inner.to_string()
    }

    /// URLのオリジンを取得します (例: "https://www.example.com:8080").
    pub fn origin(&self) -> String {
        self.inner.origin().ascii_serialization()
    }

    /// クエリパラメータを変更するための可変参照を取得します。
    pub fn search_params_mut(&mut self) -> HashMap<String, String> {
        let mut params_map: HashMap<String, String> = self.inner.query_pairs().into_owned().collect();
        params_map
    }

    /// 変更されたクエリパラメータをURLに適用します。
    pub fn set_search_params(&mut self, params_map: HashMap<String, String>) {
        let mut ser = url::form_urlencoded::Serializer::new(String::new());
        for (key, value) in params_map {
            ser.append_pair(&key, &value);
        }
        self.inner.set_query(Some(&ser.finish()));
    }
}

// Debugトレイトを実装
impl fmt::Debug for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Url")
            .field("href", &self.href())
            .field("protocol", &self.protocol())
            .field("hostname", &self.hostname())
            .field("host", &self.host())
            .field("port", &self.port())
            .field("pathname", &self.pathname())
            .field("search", &self.search())
            .field("hash", &self.hash())
            .field("origin", &self.origin())
            .finish()
    }
}

// Displayトレイトを実装
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.href())
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet; // For unordered query parameter comparison

    #[test]
    fn test_url_parsing_and_properties_extended() {
        let url_str = "https://user:pass@www.example.com:8080/path/to/resource?name=John+Doe&age=30#section1";
        let url = Url::new(url_str).unwrap();

        assert_eq!(url.protocol(), "https:");
        assert_eq!(url.hostname(), Some("www.example.com"));
        assert_eq!(url.host(), Some("www.example.com:8080".to_string()));
        assert_eq!(url.port(), Some(8080));
        assert_eq!(url.pathname(), Path::new("/path/to/resource"));
        assert_eq!(url.search(), "?name=John+Doe&age=30");
        assert_eq!(url.hash(), Some("section1"));
        assert_eq!(url.href(), url_str);
        assert_eq!(url.origin(), "https://www.example.com:8080");

        let query_map: HashMap<String, String> = url.inner.query_pairs().into_owned().collect();
        assert_eq!(query_map.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(query_map.get("age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_url_encoding_and_display() {
        let url_str = "http://example.com/search?q=日本語&param=値";
        let url = Url::new(url_str).unwrap();
        let expected_encoded_query = "q=%E6%97%A5%E6%9C%AC%E8%AA%9E&param=%E5%80%A4";
        
        let href = url.href();
        assert!(href.starts_with("http://example.com/search?"));
        let href_query_parts: HashSet<_> = href.split('?').nth(1).unwrap_or("").split('&').collect();
        let expected_query_parts: HashSet<_> = expected_encoded_query.split('&').collect();
        assert_eq!(href_query_parts, expected_query_parts);
        
        let to_string_output = url.to_string();
        assert!(to_string_output.starts_with("http://example.com/search?"));
        let to_string_query_parts: HashSet<_> = to_string_output.split('?').nth(1).unwrap_or("").split('&').collect();
        assert_eq!(to_string_query_parts, expected_query_parts);
    }

    #[test]
    fn test_search_params_mut_and_set() {
        let mut url = Url::new("http://example.com/?a=1&b=2").unwrap();
        
        let mut params = url.search_params_mut();
        params.insert("c".to_string(), "3".to_string());
        params.remove("a");
        url.set_search_params(params);

        let search_result = url.search();
        let search_parts: HashSet<_> = search_result.trim_start_matches('?').split('&').collect();
        assert!(search_parts.contains("b=2"));
        assert!(search_parts.contains("c=3"));
        assert!(!search_parts.contains("a=1"));

        let href_result = url.href();
        let href_query_parts: HashSet<_> = href_result.split('?').nth(1).unwrap_or("").split('&').collect();
        assert!(href_query_parts.contains("b=2"));
        assert!(href_query_parts.contains("c=3"));
        assert!(!href_query_parts.contains("a=1"));
    }

    #[test]
    fn test_default_path() {
        let url = Url::new("http://example.com").unwrap();
        assert_eq!(url.pathname(), Path::new("/"));
        assert_eq!(url.href(), "http://example.com/");
    }

    #[test]
    fn test_protocol_format() {
        let url = Url::new("http://example.com").unwrap();
        assert_eq!(url.protocol(), "http:");
    }

    #[test]
    fn test_url_with_fragment_only() {
        let url_str = "http://example.com/#section";
        let url = Url::new(url_str).unwrap();
        assert_eq!(url.hash(), Some("section"));
        assert_eq!(url.href(), "http://example.com/#section");
    }

    #[test]
    fn test_url_with_default_https_port() {
        // If the port is explicitly specified, `url.port()` should return it.
        let url = Url::new("https://example.com:443/").unwrap();
        assert_eq!(url.href(), "https://example.com/");
        assert_eq!(url.port(), None); 
    }

    #[test]
    fn test_url_with_non_default_port() {
        let url = Url::new("http://example.com:8080/").unwrap();
        assert_eq!(url.href(), "http://example.com:8080/");
        assert_eq!(url.port(), Some(8080));
    }

    #[test]
    fn test_url_with_no_path_and_no_trailing_slash() {
        let url = Url::new("http://example.com").unwrap();
        assert_eq!(url.pathname(), Path::new("/"));
        assert_eq!(url.href(), "http://example.com/");
    }

    #[test]
    fn test_url_without_explicit_default_port() {
        let url = Url::new("http://example.com/").unwrap();
        assert_eq!(url.port(), None); // `url.port()` returns None if default port is not explicit
        assert_eq!(url.host(), Some("example.com".to_string())); // host without default port
        assert_eq!(url.href(), "http://example.com/");
    }
}