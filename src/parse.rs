use pulldown_cmark::{html, Parser};
use std::{fs, io::Write, path::Path};

const HEADER: &str = include_str!("../template/header.html");
const FOOTER: &str = include_str!("../template/footer.html");

pub(crate) fn generate_page(content: &str) -> String {
    format!("{}\n{}\n{}", HEADER, content, FOOTER)
}

pub(crate) fn markdown_to_html<P: AsRef<Path>>(file_path: P) -> String {
    let markdown = fs::read_to_string(&file_path).expect("Cannot find `index.md`.");
    let parser = Parser::new(&markdown);

    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

pub(crate) fn save_html<P: AsRef<Path>>(content: &str, file_path: P) {
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap()
}

pub(crate) fn process_articles<P: AsRef<Path>>(directory: P) -> String {
    let mut article_list = String::from("<h3>articles</h3>\n<ul>\n");

    let article_paths = fs::read_dir(directory)
        .unwrap()
        .flatten()
        .filter_map(|article| {
            let article_path = article.path();

            let is_markdown = article_path.extension() == Some("md".as_ref());
            let is_not_index = article_path.file_stem() != Some("index".as_ref());

            (is_markdown && is_not_index).then_some(article_path)
        });

    for article_path in article_paths {
        let article_name = article_path.file_stem().unwrap().to_string_lossy();

        let html_content = markdown_to_html(&article_path);
        let html_content = generate_page(&html_content);
        let output_file = format!("_site/{}.html", article_name);

        save_html(&html_content, &output_file);

        article_list.push_str(&format!(
            "<li><a href=\"/{}.html\">{}</a></li>\n",
            article_name, article_name
        ));
    }

    article_list.push_str("</ul>");
    article_list
}

pub(crate) fn process_index(directory: &str) {
    let index_path = format!("{directory}/index.md");
    let mut index_html = markdown_to_html(index_path);

    let blog_list = process_articles(directory);
    index_html.push_str(&format!("<div>\n{}\n</div>\n", blog_list));
    let html_content = generate_page(&index_html);
    save_html(&html_content, "./_site/index.html");
}
