use std::str::FromStr;

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};

use crate::errors::OpenError;

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub(crate) struct PathWithName {
    pub path: String,
    pub filename: String,
}
#[async_trait]
impl<B> FromRequest<B> for PathWithName
where
    B: Send,
{
    type Rejection = OpenError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let path = req.uri().path();
        path.parse()
    }
}

impl FromStr for PathWithName {
    type Err = OpenError;

    fn from_str(source_path: &str) -> Result<Self, Self::Err> {
        // 判断是否是合法的路径
        let cs = source_path.chars().collect::<Vec<_>>();
        match (cs.get(0), cs.last()) {
            (Some('/'), Some(last)) if last != &'/' => {}
            _ => return Err(Self::Err::NotObjectPath(source_path.to_string())),
        };
        let path_list: Vec<_> = source_path.split('/').collect();

        let path = format!("/{}", path_list[1..path_list.len() - 1].join("/"));
        match path_list.last().cloned() {
            Some(filename) => {
                let filename = urlencoding::decode(filename)
                    .map(|x| x.to_string())
                    .unwrap_or_else(|_| filename.to_string());
                Ok(Self { filename, path })
            }
            None => Err(Self::Err::NotObjectPath(source_path.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::errors::OpenError;
    use crate::middleware::path_with_name::PathWithName;

    #[test]
    fn test() {
        let path = "/".parse::<PathWithName>();
        assert_eq!(path, Err(OpenError::NotObjectPath("/".to_string())));
        let path = "/test/".parse::<PathWithName>();
        assert_eq!(path, Err(OpenError::NotObjectPath("/test/".to_string())));
        let path = "/test/sushao.txt".parse::<PathWithName>();
        assert_eq!(
            path,
            Ok(PathWithName {
                filename: "sushao.txt".to_string(),
                path: "/test".to_string(),
            })
        );
        let path = "/test/www/sushao.txt".parse::<PathWithName>();
        assert_eq!(
            path,
            Ok(PathWithName {
                filename: "sushao.txt".to_string(),
                path: "/test/www".to_string(),
            })
        );
        let path = "/test/www/xixi/sushao.txt".parse::<PathWithName>();
        assert_eq!(
            path,
            Ok(PathWithName {
                filename: "sushao.txt".to_string(),
                path: "/test/www/xixi".to_string(),
            })
        );
        let path = "/sushao.txt".parse::<PathWithName>();
        assert_eq!(
            path,
            Ok(PathWithName {
                filename: "sushao.txt".to_string(),
                path: "/".to_string(),
            })
        );
    }
}
