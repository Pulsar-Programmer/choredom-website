use std::fs::File;
use futures::StreamExt as _;
use futures_util::TryStreamExt as _;
use std::io::Write as _;



    
    // Iterate over the multipart stream
    while let Some(mut field) = payload.try_next().await.unwrap() {
        // let mut field = item?;
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

        let path = if n == 0 {format!("/tmp/{}", container)} else {format!("/tmp/{}_{}", container, n)};
        // Write the bytes from the field to the file
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            // Write bytes to file using spawn_blocking
            let path = path.clone();
            let mut f = File::create(path).map_err(|e|e.to_string())?;
            //we previously used actix_rt::task::spawn_blocking
            //what the hell is the verdict on this: we can remove it?
            let _ = actix_rt::task::spawn_blocking(move|| -> Result<(), String> {
                f.write_all(&data).map_err(|e|e.to_string())?;
                println!("4");
                Ok(())
            }).await?;
        }

        // Create a new file with the given filename
        let file = File::create(path.clone())?;
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
        upload_file(file).await;
        // Save the converted image
        // img.save(format!("{}_converted.png", path))?;
    }

use actix_multipart::form::{tempfile::TempFile, MultipartForm};

#[derive(MultipartForm)]
struct ImageUpload {
    image: TempFile,
}

#[derive(Debug, MultipartForm)]
pub struct ImageUploads{
    #[multipart(rename="file")]
    images: Vec<TempFile>,
}

async fn process_images(form: MultipartForm<ImageUploads>, container: String) -> Result<(), Box<dyn std::error::Error>> {


    let mut n = 0;
    for file in form.images {
        if form.image.size > 20 * 1024 * 1024 { // 20 MB
            return Err("File is too large!".into());
        }
        
        let mime_type = form.into_inner().image.content_type.unwrap();
        println!("{}", mime_type);
        // if  {
        //     return Err("Invalid file type".into());
        // }
        // let file = File::create(path.clone())?;
        // upload_file(file).await;
        n += 1;
    }
    

    Ok(())
}


async fn process_image(form: MultipartForm<ImageUpload>, container: String) -> Result<(), Box<dyn std::error::Error>> {

    if form.image.size > 20 * 1024 * 1024 { // 20 MB
        return Err("File is too large!".into());
    }
    
    let mime_type = form.into_inner().image.content_type.unwrap();
    println!("{}", mime_type);
    // if  {
    //     return Err("Invalid file type".into());
    // }
    // let file = File::create(path.clone())?;
    // upload_file(file).await;
    

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