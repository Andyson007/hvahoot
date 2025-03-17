use std::{
    collections::HashMap, error::Error, fmt::Display, mem::MaybeUninit, path::PathBuf,
    ptr::addr_of_mut, str::FromStr,
};

use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Method {
    Get,
    Post,
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
pub struct Header {
    method: Method,
    uri: PathBuf,
    cookies: HashMap<String, String>,
    version: (u8, u8),
}

#[derive(Debug)]
pub enum HeaderError {
    BadRequest = 400,
    NotImplemented = 501,
}

impl Error for HeaderError {}

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Header {
    pub async fn new(stream: &mut TcpStream) -> Result<Self, HeaderError> {
        match HeaderParser::new(&mut BufReader::new(&mut *stream))
            .parse()
            .await
        {
            Ok(x) => Ok(x),
            Err(x) => {
                let response = "HTTP/1.1 500 BAD REQUEST\r\n\r\nhello";
                // if whis is an error we don't really care anymore
                // its not really our problem anymore
                let _ = stream.write(response.as_bytes()).await;
                Err(x)
            }
        }
        // Ok(ret)
    }
}

struct HeaderParser<'a, T> {
    data: MaybeUninit<Header>,
    reader: &'a mut T,
}

impl<'a, T> HeaderParser<'a, T> {
    pub fn new(reader: &'a mut T) -> Self {
        Self {
            data: MaybeUninit::uninit(),
            reader,
        }
    }

    pub async fn parse(mut self) -> Result<Header, HeaderError>
    where
        T: AsyncBufReadExt + AsyncBufRead + Unpin,
    {
        self.parse_method().await?;
        self.parse_the_rest().await?;
        Ok(unsafe { self.data.assume_init() })
    }

    async fn parse_method(&mut self) -> Result<(), HeaderError>
    where
        T: AsyncBufReadExt + AsyncBufRead + Unpin,
    {
        let mut buf = String::new();
        let _ = self.reader.read_line(&mut buf).await;
        let mut split = buf.split(' ');
        let Some(Ok(method)) = split.next().map(Method::from_str) else {
            return Err(HeaderError::NotImplemented);
        };

        let Some(Ok(path)) = split.next().map(PathBuf::from_str) else {
            return Err(HeaderError::NotImplemented);
        };

        let Some(Some(version)) = split.next().map(|x| {
            let text = x.strip_prefix("HTTP/")?;
            let mut iter = text.chars();
            let major = iter.next()?.to_digit(10)? as u8;
            let '.' = iter.next()? else { return None };
            let minor = iter.next()?.to_digit(10)? as u8;

            Some((major, minor))
        }) else {
            return Err(HeaderError::NotImplemented);
        };

        if split.next().is_some() {
            return Err(HeaderError::NotImplemented);
        }

        unsafe {
            addr_of_mut!((*self.data.as_mut_ptr()).method).write(method);
            addr_of_mut!((*self.data.as_mut_ptr()).uri).write(path);
            addr_of_mut!((*self.data.as_mut_ptr()).version).write(version);
        };
        Ok(())
    }

    async fn parse_the_rest(&mut self) -> Result<(), HeaderError>
    where
        T: AsyncBufReadExt + AsyncBufRead + Unpin,
    {
        let mut buf = String::new();
        while self.reader.read_line(&mut buf).await.is_ok() {
            println!("{:?}", buf.as_bytes());
            if buf == "\r\n" {
                break;
            }

            match dbg!(buf.split_once(": ")).ok_or(HeaderError::BadRequest)? {
                ("Cookie", rest) => {
                    println!("test");
                    let cookies = Self::parse_cookies(rest);
                    unsafe {
                        addr_of_mut!((*self.data.as_mut_ptr()).cookies).write(cookies);
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Ok(())
    }

    fn parse_cookies(rest: &str) -> HashMap<String, String> {
        rest.split("; ")
            .flat_map(|x| x.split_once("="))
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect()
    }
}
