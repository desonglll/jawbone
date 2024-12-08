use actix_web::{HttpResponse, Responder};
use percent_encoding::percent_decode;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub async fn stream_music(req: actix_web::HttpRequest) -> actix_web::Result<impl Responder> {
    let path = req.match_info().get("path").unwrap_or("default.mp3");
    let decoded_path = percent_decode(path.as_bytes()).decode_utf8()?.to_string();
    println!("path: {}", decoded_path);
    use std::fs::File as StdFile;
    let file_path = Path::new(&decoded_path);
    let mut file = match StdFile::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // 获取文件长度
    let file_length = file.metadata()?.len();

    // 解析Range头
    let range = req.headers().get("Range").and_then(|header| {
        let range_str = header.to_str().ok()?;
        range_str.strip_prefix("bytes=").map(|r| r.to_string())
    });

    let (start, end) = if let Some(range) = range {
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap_or(0);
        let end = parts
            .get(1)
            .and_then(|&e| e.parse::<u64>().ok())
            .unwrap_or(file_length - 1);
        (start, end)
    } else {
        (0, file_length - 1)
    };

    // 设置起始位置
    file.seek(SeekFrom::Start(start))?;

    // 读取内容
    let length = end - start + 1;
    let mut buffer = vec![0; length as usize];
    file.read_exact(&mut buffer)?;

    let mut response = HttpResponse::PartialContent();

    // 设置Content-Range头
    response.insert_header((
        "Content-Range",
        format!("bytes {}-{}/{}", start, end, file_length),
    ));

    let mime_type = mime_guess::from_path(&file_path).first_or_octet_stream();
    response.content_type(mime_type.as_ref());

    Ok(response.body(buffer))
}
