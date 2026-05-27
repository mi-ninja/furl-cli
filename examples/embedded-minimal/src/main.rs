use furl_core::Downloader;

#[tokio::main]
async fn main() {
    let url = "https://raw.githubusercontent.com/ghimiresdp/furl-cli/refs/heads/main/res/images/example.png";
    let mut downloader = Downloader::new(url);
    if let Ok(_) = downloader.download(".", None, Some(4)).await {
        println!("Download completed successfully!");
    } else {
        println!("Download failed.");
    }
}
