use std::fs::File;
use futures::StreamExt;
use std::io::Write;

///Processes the multipart extractor of Actix for images only.
///Container should have the path from /temp/ onwards, including what the name of the file should be.
///Returns a vector of the processed filepaths.
///Maybe in the future return a vector of the processed FILES if needed.
pub async fn process_multipart(mut payload: actix_multipart::Multipart, container: String) -> Result<(), Box<dyn std::error::Error>>{

    
    let mut n = 0;
    // Iterate over the multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        
        let content_length = match field.headers().get("Content-Length") {
            Some(h) => match h.to_str() {
                Ok(s) => match s.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => return Err("Invalid Content-Length".into()),
                },
                Err(_) => return Err("Invalid header value".into()),
            },
            None => return Err("Missing Content-Length header".into()),
        };

        if content_length > 5 * 1024 * 1024 {
            return Err("File size limit exceeded".into());
        }

        let path = if n == 0 {format!("/tmp/{}", container)} else {format!("/tmp/{}_{}", container, n)};

        // Create a new file with the given filename
        let mut file = File::create(path.clone())?;

        // Write the bytes from the field to the file
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file.write_all(&data)?;
        }

        let img = match image::open(path.clone()) {
            Ok(img) => img,
            Err(_) => {
                std::fs::remove_file(path.clone())?;
                return Err("Only PNG and JPEG allowed!".into())
            }, // Skip this file if it's not a valid image
        };

        match img.color() {
            image::ColorType::Rgba8 | image::ColorType::Rgb8 => {},
            _ => {
                // Delete the file if it's not a PNG or JPEG
                std::fs::remove_file(path.clone())?;
                return Err("Only PNG and JPEG allowed!".into())
            },
        };

 
        // Save the converted image
        // img.save(format!("{}_converted.png", path))?;
        // upload_file(f).await;
        n += 1;
    }
    
    Ok(())
}

pub async fn upload_file(_f: File){
    //upload the file to some unknown destination (google drive, etc.)
    //next delete it when that finishes
    //return the link to where it is located within the JS (or  just come up with a coherent system of working it)
    // todo!();
    //std::fs::delete_file(path) will delete img.
    println!("A file has been deposited and created.");
}

















//img key:
//NOTE: For now, they have READ access. This can be bad in verification for example so only service what is necessary.
// let $head = window.location.href; << Or simply https:://localhost:8080 or eventually https://choredom.com
// ALL IN: $head/temp/
// User verification files: verification/{user}/
// User profile pic files: pfp/{user}/
// User bio pic files: bio/{user}/
// User chat files: chats/{uuid of Surreal chat room}/




//Scrapped:
// User pfp files and bio files: usr/{user}/