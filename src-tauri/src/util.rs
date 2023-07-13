use std::fs::{File,create_dir_all};
use std::io::prelude::*;
use std::env::temp_dir;
use std::path::PathBuf;
use bollard::Docker;

pub fn write_conf_file(beiboot_name: String, content: &str, file_type: &str) -> Result<String, String> {

    let tmp = temp_dir();
    let mut path = PathBuf::from(&tmp);
    path.push("beiboot");
    path.push(&beiboot_name);
    let dir_result = create_dir_all(path.clone());
    match dir_result {
        Ok(_) => (),
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why));
        }
    }

    path.push(file_type);
    let file_result = File::create(&path);
    let mut file = match file_result {
        Ok(file) => file,
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why));
        }
    };
    file.write_all(content.as_bytes()).unwrap();

    Ok(path.to_str().unwrap().to_string())
}

pub fn cleanup(beiboot_name: String) -> Result<(), String> {
    let tmp = temp_dir();
    let mut path = PathBuf::from(&tmp);
    path.push("beiboot");
    path.push(&beiboot_name);
    let dir_result = std::fs::remove_dir_all(path);
    match dir_result {
        Ok(_) => Ok(()),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why))
        }
    }
}

pub async fn check_docker_engine() -> Result<String, String> {
    let docker = Docker::connect_with_local_defaults().unwrap();
   match docker.info().await {
        Ok(_) => Ok("Docker engine is running".to_string()),
        Err(why) => Err(format!("{}", why))
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_cleanup_tls_files() {
        let beiboot_name = "test".to_string();
        let content = "test".to_string();
        let cert_type = "ca.crt".to_string();
        let result = super::write_conf_file(beiboot_name.clone(), &content, &cert_type);
        assert!(result.is_ok());

        // Note: this assertion will fail if the temp directory is not /tmp,
        // eg on windows or macos. To successfully run this on macos, set TMPDIR=/tmp
        assert_eq!(result.unwrap(), "/tmp/beiboot/test/ca.crt".to_string());
        let cleanup_result = super::cleanup(beiboot_name);
        assert!(cleanup_result.is_ok());
    }

    #[tokio::test]
    async fn test_check_docker_engine() {
        let result = super::check_docker_engine().await;
        assert!(result.is_ok());
    }
}
