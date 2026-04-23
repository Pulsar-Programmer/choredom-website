use actix_multipart::form::{tempfile::TempFile, MultipartForm};
// #[derive(MultipartForm)]
// pub struct ImageUpload {
//     image: TempFile,
// }

#[derive(Debug, MultipartForm)]
pub struct ImageUploads{
    #[multipart(rename="file")]
    pub images: Vec<TempFile>,
}

pub async fn process_images(form: MultipartForm<ImageUploads>, container: String) -> Result<(), Box<dyn std::error::Error>> {
    let images = form.into_inner().images;
    for (n, file) in images.into_iter().enumerate() {

        verify_img(&file)?;

        let path = format!("./tmp/{container}/{n}.png");
        upload_file(file, &path).await?;

    }
    Ok(())
}

pub fn verify_img(file: &TempFile) -> Result<(), Box<dyn std::error::Error>>{
    if file.size > 10 * 1024 * 1024{
        return Err("File is too large (over 10MB)!".into())
    }

    verify_type_img(file)?;

    Ok(())
}

pub fn verify_type_img(file: &TempFile) -> Result<(), Box<dyn std::error::Error>> {
    //No content_type found.
    let mime_type = file.content_type.as_ref().ok_or(anyhow::anyhow!("An error occured with the file type."))?.to_string();
    match mime_type.as_str() {
        "image/png" | "image/jpg" | "image.jpeg" => Ok(()),
        _ => Err("File is not JPG or PNG!".into()) //test this and make sure this works - cause I have a feeling it will upload a file that is not
    }
}



pub async fn upload_file(f: TempFile, path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let mut items: Vec<String> = path.split('/').map(ToString::to_string).collect();
    _ = items.pop();
    let prepath = items.into_iter().reduce(|a,b|format!("{a}/{b}")).ok_or("Error parsing path.")?;
    std::fs::create_dir_all(prepath)?;
    f.file.persist(path)?;
    //upload the file to some unknown destination (google drive, etc.)
    //next delete it when that finishes
    //return the link to where it is located within the JS (or  just come up with a coherent system of working it)
    // todo!();
    //std::fs::delete_file(path) will delete img.
    println!("A file has been deposited and created.");
    Ok(())
}


use std::{fs, path::Path, io};

pub fn clear_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
   // Remove the directory and all its contents
   fs::remove_dir_all(&path)?;

   // Recreate the directory
   fs::create_dir(&path)?;

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
