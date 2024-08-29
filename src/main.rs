use std::fs;
use std::path::Path;

fn main() {
    let source_path = Path::new("source.txt");
    let hard_link_path = Path::new("hardlink.txt");

    // 소스 파일 생성
    fs::write(source_path, "Hello, World!").expect("Unable to write source file");
    println!("Source file created.");

    // 하드 링크 생성 시도
    match fs::hard_link(source_path, hard_link_path) {
        Ok(_) => println!("Hard link created successfully."),
        Err(e) => {
            println!("Error creating hard link: {:?}", e);
            if e.raw_os_error() == Some(38) {
                println!("Error 38 (ENOTSUP) occurred: Operation not supported");
            }
        }
    }

    // 결과 확인
    if hard_link_path.exists() {
        println!("Hard link file exists.");
        let content = fs::read_to_string(hard_link_path).expect("Unable to read hard link file");
        println!("Hard link file content: {}", content);
    } else {
        println!("Hard link file does not exist.");
    }
}

// use object_store::{path::Path, ObjectStore};
// use std::sync::Arc;
// use std::io::Write;
// use std::env;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 현재 실행 경로 가져오기
//     let current_path = env::current_dir()?;
//     println!("Current directory: {:?}", current_path);

//     // FileStorageBackend 초기화
//     let storage = Arc::new(
//         object_store::local::LocalFileSystem::new_with_prefix(&current_path)?
//     );

//     // 소스 파일 생성
//     let source_path = current_path.join("source.txt");
//     let mut file = std::fs::File::create(&source_path)?;
//     file.write_all(b"Hello, World!")?;

//     // 소스와 대상 경로 설정
//     let from = Path::from("source.txt");
//     let to = Path::from("destination.txt");

//     // copy_if_not_exists 실행
//     println!("Attempting to copy file...");
//     match storage.copy_if_not_exists(&from, &to).await {
//         Ok(_) => println!("File copied successfully!"),
//         Err(e) => println!("Error copying file: {:?}", e),
//     }

//     // 결과 확인
//     let destination_path = current_path.join("destination.txt");
//     if destination_path.exists() {
//         println!("Destination file exists.");
//         // 파일 내용 읽기
//         let content = std::fs::read_to_string(&destination_path)?;
//         println!("Destination file content: {}", content);
//     } else {
//         println!("Destination file does not exist.");
//     }

//     Ok(())
// }