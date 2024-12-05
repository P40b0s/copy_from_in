use std::{path::{Path, PathBuf}, sync::Arc};
use futures::{future::BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use tokio::{fs::DirEntry, sync::Mutex};
use logger::{debug, error};
use transport::File;

use crate::Error;


#[derive(Debug,Serialize)]
pub struct FileService
{
    files: Vec<File>
}
impl Default for FileService
{
    fn default() -> Self 
    {
        Self
        {
            files: Vec::with_capacity(0)
        }
    }
}
impl FileService
{
    pub fn get_pdf(&self) -> Vec<&File>
    {
        let pdfs: Vec<&File> = self.files.iter().filter(|f| f.file_type == "pdf").collect();
        pdfs
    }
    pub async fn get_file_body<P: AsRef<Path> + ToString>(path: P) -> Result<String, Error>
    {
        let body = utilites::io::open_file_with_encoding(path, None).await?;
        Ok(body)
    }
    ///поиск файлов в директории
    pub async fn search(path: PathBuf) -> Self
    {
        //let path = Arc::new(Mutex::new(path));
        let files_list = Arc::new(Mutex::new(Vec::<File>::new()));
        Self::search_files(path, Arc::clone(&files_list)).await;
        //хз, работает и так и так....
        //let mut guard = files_list.lock().await;
        //let files = std::mem::replace(&mut *guard, Vec::with_capacity(0));
        let files = Arc::try_unwrap(files_list).unwrap().into_inner();
        Self
        {
            files
        }
    }
    pub async fn search_concat<P: AsRef<Path>>(path: &[P]) -> Self
    {
        let mut pb = PathBuf::new();
        for p in path
        {
            pb = pb.join(p);
        }
        Self::search(pb).await
    }
    fn search_files(path: PathBuf, files_list: Arc<Mutex<Vec<File>>>) -> BoxFuture<'static, ()>
    {
        async move {
            //let path_guard = path.lock().await;
            if let Some(files) = Self::get_entries(&path).await
            {
                //Добавляем все файлы из директории в список, добавляем отдельно потому что если будет ошибка то в этот список попадут не все файлы
                for f in files
                {
                    if let Some(file) = f.path().file_name().and_then(|fl| fl.to_str())
                    {
                        let mut flist_guard = files_list.lock().await;
                        //extensiion без точки - txt
                        if let Some(ext) = f.path().extension().and_then(|e| e.to_str())
                        {
                            let mut cloned_path = path.clone();
                            let file_name = file.to_owned();
                            cloned_path.push(&file_name);
                            flist_guard.push(File 
                            {
                                file_name,
                                file_type: ext.to_owned(),
                                path: cloned_path.display().to_string()
                            });
                            drop(flist_guard);
                        }
                        else if f.path().is_dir()
                        {
                            drop(flist_guard);
                            let mut cloned_path = path.clone();
                            cloned_path.push(file);
                            Self::search_files(cloned_path, Arc::clone(&files_list)).await;
                        }
                    }
                };
            }
        }.boxed()
    }

    // fn search_files(path: Arc<Mutex<PathBuf>>, files_list: Arc<Mutex<Vec<File>>>) -> BoxFuture<'static, ()>
    // {
    //     async move {
    //         let path_guard = path.lock().await;
    //         if let Some(files) = Self::get_entries(&path_guard).await
    //         {
    //             drop(path_guard);
    //             //Добавляем все файлы из директории в список, добавляем отдельно потому что если будет ошибка то в этот список попадут не все файлы
    //             for f in files
    //             {
    //                 if let Some(file) = f.path().file_name().and_then(|fl| fl.to_str())
    //                 {
    //                     let mut path_guard = path.lock().await;
    //                     let mut flist_guard = files_list.lock().await;
    //                     //extensiion без точки - txt
    //                     if let Some(ext) = f.path().extension().and_then(|e| e.to_str())
    //                     {
    //                         let file_name = file.to_owned();
    //                         let mut p = path_guard.clone();
    //                         p.push(&file_name);
    //                         flist_guard.push(File 
    //                         {
    //                             file_name,
    //                             file_type: ext.to_owned(),
    //                             path: p.display().to_string()
    //                         });
    //                         drop(path_guard);
    //                         drop(flist_guard);
    //                     }
    //                     else if f.path().is_dir()
    //                     {
    //                         path_guard.push(file);
    //                         let mut cloned = path_guard.clone();
    //                         drop(path_guard);
    //                         drop(flist_guard);
    //                         cloned.push(file);
                            
    //                         Self::search_files(Arc::clone(&path), Arc::clone(&files_list)).await;
    //                     }
    //                 }
    //             };
    //         }
    //     }.boxed()
    // }

    pub fn get_list(&self) -> &[File]
    {
        &self.files
    }

    async fn get_entries(path:&Path) -> Option<Vec<DirEntry>>
    {
        let paths = tokio::fs::read_dir(path).await;
        if paths.is_err()
        {
            error!("😳 Ошибка чтения директории {} - {}", path.display(), paths.err().unwrap());
            return None;
        }
        let mut paths = paths.unwrap();
        let mut dirs = vec![];
        while let Some(d) = paths.next_entry().await.ok()?
        {
            let dir = d;
            dirs.push(dir);
        }
        return Some(dirs);
    }
}

#[cfg(test)]
mod tests
{
    use std::path::PathBuf;
    #[tokio::test]
    async fn test_files()
    {   let _ = logger::StructLogger::new_default();
        let p = PathBuf::from("/hard/xar/projects/test_data/copy_from_in_test_data/out/70178878_3");
        let files =  super::FileService::search(p).await;
        assert_eq!(files.files[files.files.len()-1].path, "/hard/xar/projects/test_data/copy_from_in_test_data/out/70178878_3/container/regnum_e7e4506f121644fb9193450bcc66e7f9.png");
        logger::info!("{:?}", &files.files);
        let pdfs = files.get_pdf();
        assert_eq!(pdfs.len(), 1);

    }
    #[tokio::test]
    async fn test_files2()
    {   let _ = logger::StructLogger::new_default();
        let p = &["/hard/xar/projects/test_data/copy_from_in_test_data/in2", "38773995_1"];
        let files =  super::FileService::search_concat(p).await;
        logger::info!("{:?}", &files.files);
        let pdfs = files.get_pdf();
        assert_eq!(pdfs.len(), 1);
    }

    #[tokio::test]
    async fn test_files3()
    {   let _ = logger::StructLogger::new_default();
        let p = &["../../test_data/copy_from_in_test_data/in2", "70178878_1 copy 11"];
        let files =  super::FileService::search_concat(p).await;
        logger::info!("{:?}", &files.files);
        let pdfs = files.get_pdf();
        assert_eq!(pdfs.len(), 1);
    }
}