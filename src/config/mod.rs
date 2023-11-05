mod xdiff;
mod xreq;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use std::str::FromStr;
use tokio::fs;
use url::Url;

pub use crate::{ExtraArgs, ResponseProfile};
pub use xdiff::*;
pub use xreq::*;

pub fn is_default<T: Default + PartialEq>(v: &T) -> bool {
    v == &T::default()
}

#[async_trait]
pub trait LoadConfig
where
    Self: ValidateConfig + DeserializeOwned,
{
    /// load config from yaml file
    async fn load_yaml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    /// load config from yaml string
    fn from_yaml(content: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }
}

pub trait ValidateConfig {
    fn validate(&self) -> Result<()>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    // skip_serializing_if
    // 调用函数来确定是否跳过序列化该字段。
    // 给定的函数必须可调用为 fn(&T) -> bool，尽管它可能是T上的通用函数。
    // 例如，skip_serializing_if = "Option::is_none"将跳过为None的选项。
    #[serde(skip_serializing_if = "empty_json_value", default)]
    // #[serde(default)]: If the value is not present when deserializing, use the Default::default().
    pub params: Option<serde_json::Value>,
    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

fn empty_json_value(v: &Option<serde_json::Value>) -> bool {
    v.as_ref().map_or(true, |v| {
        v.is_null() || (v.is_object() && v.as_object().unwrap().is_empty())
    })
}

#[derive(Debug)]
pub struct ResponseExt(Response);

impl ResponseExt {
    pub fn into_inner(self) -> Response {
        self.0
    }
    pub async fn get_text(self, profile: &ResponseProfile) -> Result<String> {
        let res = self.0;
        let mut output = get_status_text(&res)?;

        write!(
            &mut output,
            "{}",
            get_headers_text(&res, &profile.skip_headers)?
        )?;

        // let mut output = get_headers_text(&res, &profile.skip_headers)?;
        // let content_type = get_content_type(res.headers());
        // let text = res.text().await?;

        // match content_type.as_deref() {
        //     Some("application/json") => {
        //         let text = filter_json(&text, &profile.skip_body)?;
        //         output.push_str(&text);
        //     }
        //     _ => {
        //         output.push_str(&text);
        //     }
        // }

        writeln!(
            &mut output,
            "{}",
            get_body_text(res, &profile.skip_body).await?
        )?;

        Ok(output)
    }

    pub fn get_header_keys(&self) -> Vec<String> {
        let res = &self.0;
        let headers = res.headers();
        headers
            .iter()
            .map(|(k, _)| k.as_str().to_string())
            .collect()
    }
}

pub async fn get_body_text(res: Response, skip_body: &[String]) -> Result<String> {
    let content_type = get_content_type(res.headers());
    let text = res.text().await?;

    // match content_type.as_deref() {
    //     Some("application/json") => {
    //         let text = filter_json(&text, &profile.skip_body)?;
    //         writeln!(&mut output, "{}", text)?;
    //     }
    //     _ => {
    //         writeln!(&mut output, "{}", text)?;
    //     }
    // }

    match content_type.as_deref() {
        Some("application/json") => filter_json(&text, skip_body),
        _ => Ok(text),
    }
}

pub fn get_status_text(res: &Response) -> Result<String> {
    Ok(format!("{:?} {}\n", res.version(), res.status()))
}

pub fn get_headers_text(res: &Response, skip_headers: &[String]) -> Result<String> {
    let mut output = String::new();
    // write!(output, "{:?} {}\r", self.0.version(), self.0.status())?;
    // output.push_str(&format!("{:?} {}\n", res.version(), res.status()));

    let headers = res.headers();
    for (k, v) in headers.iter() {
        if !skip_headers.contains(&k.to_string()) {
            // if !profile.skip_headers.iter().any(|x| x == k.as_str( ) ) {
            output.push_str(&format!("{}: {:?}\n", k, v));
            // write!(&mut output, "{}: {:?}\n", k, v)?;
        }
    }

    Ok(output)
}

fn filter_json(text: &str, skip: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;

    // match json {
    //     serde_json::Value::Object(ref mut obj) => {
    //         for key in skip {
    //             obj.remove(key);
    //         }
    //     }
    //     _ =>
    //         // for now we just ignore non_object values, we don't how to filter them
    //         //  In future, we might support array of primitives
    //         {}
    // }

    // for now we just ignore non_object values, we don't how to filter them
    // In future, we might support array of objects
    if let serde_json::Value::Object(ref mut obj) = json {
        for key in skip {
            obj.remove(key);
        }
    }

    Ok(serde_json::to_string_pretty(&json)?)
}

impl RequestProfile {
    pub fn new(
        method: Method,
        url: Url,
        params: Option<serde_json::Value>,
        headers: HeaderMap,
        body: Option<serde_json::Value>,
    ) -> Self {
        Self {
            method,
            url,
            params,
            headers,
            body,
        }
    }

    pub async fn send(&self, args: &ExtraArgs) -> Result<ResponseExt> {
        let (headers, query, body) = self.generate(args)?;
        let client = Client::new();
        let req = client
            .request(self.method.clone(), self.url.clone())
            .query(&query)
            .headers(headers)
            .body(body)
            .build()?;

        let res = client.execute(req).await?;

        Ok(ResponseExt(res))
    }

    pub fn get_url(&self, args: &ExtraArgs) -> Result<String> {
        let (_, params, _) = self.generate(args)?;
        let mut url = self.url.clone();
        if !params.as_object().unwrap().is_empty() {
            let query = serde_qs::to_string(&params)?;
            url.set_query(Some(&query));
        }
        // url.set_query(None);
        // let mut query = serde_qs::to_string(&query)?;
        // if !query.is_empty() {
        //     // url.set_query(Some(&query));
        //     write!(url, "?{}", &query)?;
        // }
        Ok(url.to_string())
    }

    fn generate(&self, args: &ExtraArgs) -> Result<(HeaderMap, serde_json::Value, String)> {
        let mut headers = self.headers.clone();
        let mut query = self.params.clone().unwrap_or_else(|| json!({}));
        let mut body = self.body.clone().unwrap_or_else(|| json!({}));

        for (k, v) in &args.headers {
            // println!("测试：{}{}", k, v);
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        if !headers.contains_key(header::CONTENT_TYPE) {
            // println!("测试：{}___{:?}", header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            headers.insert(
                header::CONTENT_TYPE,
                // 用于指示资源的媒体类型。
                // 在响应中，Content-Type 标头告诉客户端返回内容的实际内容类型。
                // 在某些情况下，浏览器会进行 MIME 嗅探，但不一定会遵循此标头的值；
                // 为了防止这种行为，可以将标头 X-Content-Type-Options 设置为 nosniff。
                // 在请求（例如 POST 或 PUT）中，客户端告诉服务器实际发送的数据类型。
                HeaderValue::from_static("application/json"),
            );
            // "Content-Type" 是 HTTP 请求头部中的一个字段，它用于指定请求或响应中携带的实体数据的媒体类型（即数据的类型和格式）
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
            // parse() -> Result<T, <T as FromStr>::Err>
            // 将此字符串切片解析为另一种类型。
            // 由于解析非常通用，因此可能会导致类型推断出现问题。
            // 因此，解析是您会看到被亲切地称为“turbofish”的语法的少数情况之一：::<>。
            // 这有助于推理算法具体了解您要解析的类型。
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?;
        }

        // println!("测试：{:?}", headers);

        let content_type = get_content_type(&headers);

        // println!("测试：{:?}", content_type);
        
        match content_type.as_deref() {
            // as_deref()是一个Rust标准库中的方法，它用于将Option<&T>转换为Option<&U>，其中T和U是具体的类型。
            Some("application/json") => {
                let body = serde_json::to_string(&body)?;
                Ok((headers, query, body))
            }
            Some("application/x-www-form-urlencoded" | "multipart/form-data") => {
                let body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, body))
            }
            _ => Err(anyhow::anyhow!("unsupported content-type")),
        }
    }
}

impl ValidateConfig for RequestProfile {
    fn validate(&self) -> Result<()> {
        if let Some(params) = self.params.as_ref() {
            if !params.is_object() {
                return Err(anyhow::anyhow!(
                    "Params must be an object but got\n{}",
                    serde_yaml::to_string(params)?
                ));
            }
        }
        if let Some(body) = self.body.as_ref() {
            if !body.is_object() {
                return Err(anyhow::anyhow!(
                    "Body must be an object but got\n{}",
                    serde_yaml::to_string(body)?
                ));
            }
        }
        Ok(())
    }
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::CONTENT_TYPE)
        // .map(|v| v.to_str().unwrap().split(';').next())
        // .flatten()
        // .map(|v| v.to_string())
        .and_then(|v| v.to_str().unwrap().split(";").next().map(|v| v.to_string()))
}

impl FromStr for RequestProfile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut url = Url::parse(s)?;
        let qs = url.query_pairs();
        let mut params = json!({});
        for (k, v) in qs {
            params[&*k] = v.parse()?;
        }

        url.set_query(None);

        Ok(RequestProfile::new(
            Method::GET,
            url,
            Some(params),
            HeaderMap::new(),
            None,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Mock};
    use reqwest::StatusCode;

    #[tokio::test]
    async fn request_profile_send_should_work() {
        let _m = mock_for_url("/todo?a=1&b=2", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo?a=1&b=2", &Default::default())
            .await
            .into_inner();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn request_profile_send_with_extra_args_should_work() {
        let _m = mock_for_url("/todo?a=1&b=3", json!({"id": 1, "title": "todo"}));

        let args = ExtraArgs::new_with_query(vec![("b".into(), "3".into())]);

        let res = get_response("/todo?a=1&b=2", &args).await.into_inner();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    fn request_profile_get_url_should_work() {
        let profile = get_profile("/todo?a=1&b=2");
        assert_eq!(
            profile.get_url(&Default::default()).unwrap(),
            get_url("/todo?a=1&b=2") // format!("{}/todo?a=1&b=2", mockito::server_url())
        );
    }

    #[test]
    fn request_profile_get_url_with_args_should_work() {
        let profile = get_profile("/todo?a=1&b=2");

        let args = ExtraArgs::new_with_query(vec![("c".into(), "3".into())]);

        assert_eq!(
            profile.get_url(&args).unwrap(),
            get_url("/todo?a=1&b=2&c=3") // format!("{}/todo?a=1&b=2&c=3", mockito::server_url())
        );
    }

    #[test]
    fn request_profile_validate_should_work() {
        let profile = get_profile("/todo?a=1&b=2");
        assert!(profile.validate().is_ok());
    }

    #[test]
    fn request_profile_with_bad_params_validate_should_fail() {
        let profile = RequestProfile::new(
            Method::GET,
            Url::parse("http://localhost:1234/todo").unwrap(),
            Some(json!([1, 2, 3])),
            HeaderMap::new(),
            None,
        );
        let result = profile.validate();
        assert!(profile.validate().is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Params must be an object but got\n- 1\n- 2\n- 3\n"
        );
    }

    #[tokio::test]
    async fn response_ext_get_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo", &Default::default()).await;

        let response_profile = ResponseProfile::new(
            vec!["connection".into(), "content-length".into()],
            vec!["title".into()],
        );
        assert_eq!(
            res.get_text(&response_profile).await.unwrap(),
            "HTTP/1.1 200 OK\ncontent-type: \"application/json\"\n{\n  \"id\": 1\n}\n"
        );
    }

    #[tokio::test]
    async fn response_ext_get_header_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo", &Default::default()).await;
        let mut sorted_header_keys = res.get_header_keys();
        sorted_header_keys.sort();
        let expected_header_keys = vec!["connection", "content-length", "content-type"];
        // assert_eq!(
        //     res.get_header_keys(),
        //     &["connection", "content-type", "content-length"]
        // );
        assert_eq!(sorted_header_keys, expected_header_keys);
    }

    #[test]
    fn test_get_content_type() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        assert_eq!(
            get_content_type(&headers),
            Some("application/json".to_string())
        );
    }

    #[tokio::test]
    async fn get_status_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(get_status_text(&res).unwrap(), "HTTP/1.1 200 OK\n");
    }

    #[tokio::test]
    async fn get_headers_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(
            get_headers_text(&res, &["connection".into(), "content-length".into()]).unwrap(),
            "content-type: \"application/json\"\n"
        );
    }

    #[tokio::test]
    async fn get_body_text_should_work() {
        let _m = mock_for_url("/todo", json!({"id": 1, "title": "todo"}));
        let res = get_response("/todo", &Default::default())
            .await
            .into_inner();
        assert_eq!(
            get_body_text(res, &["id".into()]).await.unwrap(),
            "{\n  \"title\": \"todo\"\n}"
        );
    }

    fn mock_for_url(path_and_query: &str, resp_body: serde_json::Value) -> Mock {
        mock("GET", path_and_query)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&resp_body).unwrap())
            .create()
    }

    fn get_url(path: &str) -> String {
        format!("{}{}", mockito::server_url(), path)
    }

    fn get_profile(path_and_query: &str) -> RequestProfile {
        let url = get_url(path_and_query);
        RequestProfile::from_str(&url).unwrap()
    }

    async fn get_response(path_and_query: &str, args: &ExtraArgs) -> ResponseExt {
        let profile = get_profile(path_and_query);
        profile.send(args).await.unwrap()
    }
}
