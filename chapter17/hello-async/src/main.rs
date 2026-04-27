use std::future::Future;
use trpl::{Html,block_on};

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let url = &args[1];
	block_on(async {
		match page_title(url).await {
			Some(title) => println!("the title for {url} is {title}"),
			None => println!("{url} has no title."),
		}
	})
}


async fn page_title(url: &str) -> Option<String> {
	let response_text = trpl::get(url).await.text().await;
	Html::parse(&response_text)
		.select_first("title")
		.map(|title| title.inner_html())
}

fn compiled_page_title(url: &str) -> impl Future <Output = Option<String>> {
	async move {
		let response_text = trpl::get(url).await.text().await;
		Html::parse(&response_text)
			.select_first("title")
			.map(|title| title.inner_html())
	}
}
