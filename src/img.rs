use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use anyhow::anyhow;
// #[derive(MultipartForm)]
// pub struct ImageUpload {
//     image: TempFile,
// }

#[derive(Debug, MultipartForm)]
pub struct ImageUploads{
    #[multipart(rename="file")]
    images: Vec<TempFile>,
}

pub async fn process_images(form: MultipartForm<ImageUploads>, container: String) -> Result<(), Box<dyn std::error::Error>> {
    let images = form.into_inner().images;
    for (n, file) in images.into_iter().enumerate() {
        if file.size > 20 * 1024 * 1024 { // 20 MB
            return Err("File is too large!".into());
        }

        let mime_type = file.content_type.as_ref().ok_or(anyhow::anyhow!("No content_type found."))?.to_string();
        match mime_type.as_str() {
            "image/png" | "image/jpg" | "image.jpeg" => {}
            _ => {return Err("File is not JPG or PNG!".into())} //test this and make sure this works - cause I have a feeling it will upload a file that is not
        }
            
        let path = if n == 0 { format!("./tmp/{}", container) } else {
            let mut c = container.split('.');
            let before = c.next().ok_or(anyhow!("Internal server error."))?;
            let after = c.next().ok_or(anyhow!("Internal server error."))?;
            format!("./tmp/{}_{}.{}", before, n, after)
        };
        upload_file(file, &path).await?;
    }

    Ok(())
}

pub async fn upload_file(f: TempFile, path: &str) -> Result<(), Box<dyn std::error::Error>>{
    f.file.persist(path)?;
    //upload the file to some unknown destination (google drive, etc.)
    //next delete it when that finishes
    //return the link to where it is located within the JS (or  just come up with a coherent system of working it)
    // todo!();
    //std::fs::delete_file(path) will delete img.
    println!("A file has been deposited and created.");
    Ok(())
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