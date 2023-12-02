use std::fs::File;

///Processes the multipart extractor of Actix for images only.
///Container should not have any path slash before or after
pub async fn process_multipart(mut form: actix_multipart::Multipart, mut per_form_to_container: impl FnMut(&actix_multipart::Field) -> String) -> Result<(), Box<dyn std::error::Error>>{
    use futures::TryStreamExt;
    use futures::StreamExt;
    use std::io::Write;
    use actix_web::web;


    // iterate over multipart stream
    while let Ok(Some(mut field)) = form.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().ok_or("Filename processing error.")?;
        let filepath = format!("/temp/{}/{}", per_form_to_container(&field), sanitize_filename::sanitize(filename));

        use image::ImageFormat;
        use std::path::Path;

        //if a format can be created without issue, the file is a successful image only
        let format = ImageFormat::from_path(Path::new(&filepath))?;

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
