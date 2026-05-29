use furl_core::Downloader;

#[tokio::main]
async fn main() {
    let url = "https://raw.githubusercontent.com/ghimiresdp/furl-cli/refs/heads/main/res/images/example.png";
    let mut downloader = Downloader::new(url);
    if downloader.download(".", None, Some(4)).await.is_ok() {
        println!("Download completed successfully!");
    } else {
        println!("Download failed.");
    }
}
