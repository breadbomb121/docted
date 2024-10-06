use std::io::Write;

use anyhow::{anyhow, Result};
use bat::{Input, PagingMode, PrettyPrinter};

use crate::docted::Docted;

fn get_selector(lang: &str)-> Result<(String, String)>{
    match lang {
        "javascript" => Ok(("! site:developer.mozilla.org".to_string(), "article.main-page-content".into())),
        "react" => Ok(("! site:react.dev/reference/react".to_string(), "article".into())),
        "rust" => Ok(("! site:docs.rs/rustc-std-workspace-std/latest/std".to_string(), "div.docblock".into())),
        "java" => Ok(("! site:docs.oracle.com".to_string(), "div.contentContainer".into())),
        _ => Err(anyhow!("No match")) 
    }
}

pub fn get_doc(item: String, overide_lang: Option<String>, no_page: bool) -> Result<()> {
    let lang = if let Some(lang) = overide_lang {
        lang
    }else {
        Docted::from_env_dir()?.project.lang.clone()
    };
    match lang.as_str() {
        "java" | "react" | "javascript" => tokio::runtime::Runtime::new()?.block_on(search(item, lang, no_page))?,
        _ => return Err(anyhow!("No language selected"))
    };
    Ok(())
}
async fn search(item: String, lang: String, no_page: bool) -> Result<()>{
    let (mut site, selector_str) = get_selector(&lang)?;
    site.push_str(&format!(" {}", item));
    let query = urlencoding::encode(&site);
    let url = format!("https://duckduckgo.com/?q={}&format=json", query);
    let response = surf::get(url.as_str()).await.map_err(|e| anyhow!("Doc Error: {}", e))?;
    let location = response
        .header("location")
        .map(|xs| xs.as_str().to_owned())
        .unwrap_or_else(Default::default);

    if location.is_empty() {
        std::io::stderr().write(b" No results.")?;
        return Ok(())
    }
    let body = surf::get(location.as_str()).recv_string().await.map_err(|e| anyhow!("{}", e))?;
    let document = scraper::Html::parse_document(body.as_str());
    let article_sel = scraper::Selector::parse(&selector_str).unwrap();

    let article = document
        .select(&article_sel)
        .next()
        .map(|xs| xs.inner_html())
        .unwrap_or_else(|| "".to_string());

    let html = format!("<article>{}</article>", article);
    let size = if let Some((width, _)) = term_size::dimensions() {width} else{120};
    let text= html2text::from_read(html.as_bytes(), size);
    let mut printer = PrettyPrinter::new();
        printer
        .pager("less")
        .header(true)
        .grid(true)
        .input(
            Input::from_bytes(text.as_bytes())
                .title(url)
                .kind("Search Result"),
        );
        if no_page {
            printer.paging_mode(PagingMode::Never)
        }else {
            printer.paging_mode(PagingMode::QuitIfOneScreen)
        };
        printer.print().unwrap();
    Ok(())
}
