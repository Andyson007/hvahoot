use std::{collections::HashMap, convert::identity, path::PathBuf, str::FromStr};

use tokio::io::{AsyncBufRead, AsyncBufReadExt};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Delete,
    Put,
    Connect,
    Head,
    Options,
    Trace,
    Patch,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct Http {
    method: Method,
    path: PathBuf,
    cookies: HashMap<String, String>,
}

impl Http {
    pub async fn new<T>(reader: &mut T) -> Result<Self, ()>
    where
        T: AsyncBufRead + AsyncBufReadExt + Unpin,
    {
        let mut iter = reader.lines();
        let http_request = {
            let mut ret = vec![];
            while let Ok(Some(next)) = iter.next_line().await {
                if next.is_empty() {
                    break;
                }
                ret.push(next);
            }
            ret
        };
        println!("{http_request:#?}");
        let (method, path) = parse_method(&http_request[0]).ok_or(())?;
        let cookies = get_cookies(&http_request);
        let ret = Self {
            method,
            path,
            cookies,
        };
        Ok(ret)
    }
}

fn parse_method(line: impl AsRef<str>) -> Option<(Method, PathBuf)> {
    let mut split = line.as_ref().split_whitespace();
    let method = Method::from_str(split.next()?).ok()?;
    let path = PathBuf::from_str(split.next()?).ok()?;
    // TODO: parse version information
    Some((method, path))
}

fn get_cookies<T: AsRef<[String]>>(list: T) -> HashMap<String, String> {
    list.as_ref()
        .iter()
        .filter_map(|x| x.strip_prefix("Cookie: "))
        .flat_map(|line| line.split_terminator("; "))
        .filter_map(|x| x.split_once('='))
        .map(|(x, y)| (x.to_string(), y.to_string()))
        .collect()
}
