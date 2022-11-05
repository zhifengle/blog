use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result, Context};
use base64::decode;
use image::{load_from_memory_with_format, DynamicImage, ImageFormat};
use scraper::ElementRef;

static APP_NAME: &str = "bookmark2shortcut";

#[derive(Default)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
    pub icon: Option<String>,
    pub add_date: Option<usize>,
    pub last_visit: Option<usize>,
    pub last_modified: Option<usize>,
}

#[derive(Default)]
pub struct Folder {
    pub name: String,
    pub add_date: Option<usize>,
    pub last_visit: Option<usize>,
    pub last_modified: Option<usize>,
}

fn base64_to_image(base_str: &str) -> Result<DynamicImage> {
    let end = ";base64,";
    let prefix = "data:image/";
    if !base_str.starts_with(prefix) {
        return Err(anyhow!("invalid base64 image"));
    }
    let idx = base_str.find(end).unwrap();
    let raw_data = &base_str[idx + end.len()..];
    let mime = &base_str[5..idx];
    let buf = decode(raw_data).unwrap();
    let img = load_from_memory_with_format(&buf, ImageFormat::from_mime_type(mime).unwrap())?;
    Ok(img)
}

pub fn base64_to_icon<Q>(base_str: &str, name: Q) -> Result<()>
where
    Q: AsRef<Path>,
{
    let img = base64_to_image(base_str)?;
    img.save(name)?;
    Ok(())
}

fn get_icon_folder() -> Result<PathBuf> {
    let icon_folder = dirs::data_dir().unwrap().join(APP_NAME).join("icons");
    if !icon_folder.exists() {
        fs::create_dir_all(&icon_folder)?;
    }
    Ok(icon_folder)
}
impl Bookmark {
    pub fn from_anchor(el: &ElementRef) -> Self {
        let val = el.value();
        let add_date: Option<usize> = val.attr("add_date").and_then(|s| {
            let s: usize = s.parse::<usize>().unwrap();
            return Some(s);
        });
        Self {
            title: el.inner_html(),
            url: val.attr("href").unwrap().to_string(),
            icon: val.attr("icon").map(|s| s.to_string()),
            add_date,
            ..Default::default()
        }
    }
    fn to_shortcut_string(&self) -> Result<String> {
        let host = if let Ok(url_obj) = url::Url::parse(&self.url) {
            url_obj.host_str().and_then(|s| Some(s.to_string()))
        } else {
            None
        };
        let icon_folder = get_icon_folder()?;
        let icon_path: String = self
            .icon
            .as_ref()
            .and_then(|icon| {
                if host.is_none() {
                    return None;
                }
                let icon_name = format!("{}.ico", host.unwrap());
                let icon_path = icon_folder.join(icon_name);
                if icon_path.exists() {
                    return Some(icon_path.display().to_string());
                }
                match base64_to_icon(icon.as_str(), &icon_path) {
                    Ok(_) => Some(icon_path.display().to_string()),
                    Err(_) => None,
                }
            })
            .unwrap_or("".to_string());
        let contents = format!(
            "[InternetShortcut]\nIDList=\nURL={}\nIconIndex=0\nHotKey=0\nIconFile={}",
            self.url, icon_path
        );
        Ok(contents)
    }
    pub fn write_shortcut(&self, output_path: &PathBuf) -> Result<()> {
        let contents = self.to_shortcut_string()?;
        if !output_path.exists() {
            fs::create_dir_all(&output_path).with_context(|| format!("创建文件夹失败: {}", output_path.display()))?;
        }
        let filename = format!("{}.url", sanitize_filename::sanitize(&self.title).trim());
        let res = fs::write(
            // output_path.join(format!("{}.url", self.title.replace("\\", "_").replace("|", "-").replace(":", "%3A"))),
            output_path.join(filename),
            contents,
        );
        if res.is_err() {
            eprintln!(
                "写入书签错误! 书签名称: {} ，路径: {}，错误信息: {}",
                self.title,
                output_path.display().to_string(),
                res.unwrap_err()
            );
        }
        Ok(())
    }
}

#[test]
fn t_base() {
    let s = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAADSElEQVQ4jU2TXUxbdQDFf/f2396WUsACBTIQEXAgCw4CQ2RqTEw2NwnRLW7RkZkZP5L5YPTBxCfj0/RlkmjQuRgjmLnpHCMjOhLYhIQgDMenEuxgEgotH4WWftyWW/4+INWTnMfzS05yjtLmQAIo7Cizqoas2sdZnxpnKxggfe+jaFnZxFZXWL3zG6H7s/xfSvu/AIDSM2/irKph5sJnFDefofDYSYLuGdZGhjAiEYTNRtizwOylb4mv+1EAsRvOP9pEQdNxBt5o5unLnVhdOfSdOk5kcYHs+oPYcvPQV5cxwmFc9Qfx/tqLEQ5BuwN5yZUivcODsquhSs5dvyqDngV5+UGn7HmpUW76vDIWiyUdXluV019/KX8oyZVtDqQQDgd5h44SXVlBsVpxPXuY/hcPY8nMou6rdhQhMAzjv9JWGwUnTiFNgqG3X0NkPFZNRnUtGzN/knukicCsG99AP9WffoEhJVLXSeg6wm5PMqLeJXIbXyCvqwPVXloGVhvbhkHq3nJWhwYByDhQj398lFvPP8PNJ6uYPPcRhmFgbG2RSCSY7/iRgpdPo9oKi9jaDKLlFyItGvFwCG1PAdtScv/Kd+QcasSatwdfXy8rw4ME5+6xraoE5+5hxOOo1oJC9OVlUvdVotpspJZXYMnM4u67ZxFp6WzOutnfcgFD1/HfHWErEmHtzhCW7ByCf82gyoSB/eEStmMxUoqKSauoJLPhKYJTEyx2XiX/2AlMKSlszkxj0jQSepTQrBthtxOe/xvh+b6N/a3foHuXEGYzAGXvfUDx62cxaVZUTcPX072z0po6ApNjKEhkPEZkfg51pbcb340OHIUPIYRI2ubMxGK3I4TA3dpCenkFzn2VLHZdJ6O0jMDEGMGJMVSA6XMfsnTtCqphYDabk1YTBqPvv0Pgj0kaLrYTGB0h6lkgvbgET+dPxDfWd6aciERY+uUG/t+HEY40hCON6LIPb/9tTGYLz/UMYNI0br/1KrWftOC+2Ep8Yx0FUHbfKB5wUnSyGVtOLvFQCMVsJqu6luzqGha6f2bq8/OUvHKaqM/H5PmP2X1wErANSMDxSBmuA09gdeWg+9dYGx9FURRctXX4pybw9t1KhlHgH6Ewa69LbAwMAAAAAElFTkSuQmCC";
    let b = Bookmark {
        url: "http://www.ifeng.com/".to_string(),
        icon: Some(s.to_string()),
        title: "测试名称".to_string(),
        ..Default::default()
    };
    b.write_shortcut(&PathBuf::from(".")).unwrap();
}
