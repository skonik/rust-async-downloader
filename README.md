# Rusty Download

<p align="center">
  <img src="./img/rusty_downloader.png" height=432 width=396 />
</p>


CLI program to asynchronously download files from specified urls in file.


## Usage

`$ cargo run https://www.softcover.io/download/88e295ad/GoBootcamp/ebooks/GoBootcamp.pdf "."`

As a result content of the page will be saved under `go-book.pdf`.

You can also set multiple urls with `,` as delimiter.

![downloader_showcase.gif](./img/downloader_showcase.gif)
