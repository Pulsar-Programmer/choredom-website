use std::fs::File;

///Processes the multipart extractor of Actix for images only.
///Container should have the path from /temp/ onwards, including what the name of the file should be.
///Returns a vector of the processed filepaths.
///Maybe in the future return a vector of the processed FILES if needed.
pub async fn process_multipart(mut form: actix_multipart::Multipart, container: String) -> Result<(), Box<dyn std::error::Error>>{
    use futures::TryStreamExt;
    use futures::StreamExt;
    use std::io::Write;
    use actix_web::web;
    let mut num = 0;
    // iterate over multipart stream
    while let Some(mut field) = form.try_next().await? {
        let filepath = format!("/temp/{}_{}", container, num);

        use image::ImageFormat;
        use std::path::Path;

        //if a format can be created without issue, the file is a successful image only
        let content_disposition = field.content_disposition();
        let format = ImageFormat::from_path(Path::new(&content_disposition.get_filename().ok_or("Filename processing error.")?))?;

        match format{
            ImageFormat::Png | ImageFormat::Jpeg => {},
            _ => return Err("Only PNG and JPEG allowed!".into()),
        }

        let file_ref = filepath.clone();
        //remember to either throw an error or change the file name when uploading file names that are different.
        let mut f = web::block(|| std::fs::File::create(file_ref)).await??;
        
        while let Some(Ok(chunk)) = field.next().await {
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }

        upload_file(f).await;
        num += 1;
    }
    Ok(())
}

pub async fn upload_file(_f: File){
    //upload the file to some unknown destination (google drive, etc.)
    //next delete it when that finishes
    //return the link to where it is located within the JS (or  just come up with a coherent system of working it)
    // todo!();
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
