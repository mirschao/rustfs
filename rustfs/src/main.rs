use warp::Filter;
use std::convert::Infallible;
use tokio::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    // 设置上传文件路径
    let upload_dir = "uploads";
    tokio::fs::create_dir_all(upload_dir).await.unwrap();

    // 上传文件的路由
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(10 * 1024 * 1024)) // 限制文件大小为 10MB
        .and(warp::any().map(move || upload_dir.to_string()))
        .and_then(handle_upload);

    // 下载文件的路由
    let download_route = warp::path("download")
        .and(warp::get())
        .and(warp::path::param::<String>())
        .and(warp::any().map(move || upload_dir.to_string()))
        .and_then(handle_download);

    // 合并路由
    let routes = upload_route.or(download_route);

    // 启动服务器
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// 上传文件的处理函数
async fn handle_upload(
    form: warp::multipart::FormData,
    upload_dir: String,
) -> Result<impl warp::Reply, Infallible> {
    let mut save_tasks = vec![];

    form.for_each(|part| {
        let upload_dir = upload_dir.clone();
        save_tasks.push(async move {
            if part.name() == "file" {
                let filename = part.filename().unwrap_or("unknown").to_string();
                let filepath = PathBuf::from(upload_dir).join(filename);
                let mut file = fs::File::create(filepath).await.unwrap();
                let mut stream = part.stream();

                while let Some(Ok(chunk)) = stream.next().await {
                    tokio::io::copy(&mut chunk.as_ref(), &mut file).await.unwrap();
                }
            }
        });

        futures::future::ready(())
    })
    .await;

    Ok(warp::reply::json(&serde_json::json!({"status": "success"})))
}

// 下载文件的处理函数
async fn handle_download(
    filename: String,
    upload_dir: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let filepath = PathBuf::from(upload_dir).join(filename);

    if !filepath.exists() {
        return Err(warp::reject::not_found());
    }

    let file = tokio::fs::File::open(filepath).await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(file);

    Ok(warp::reply::stream(stream))
}
