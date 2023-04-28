use std::fs::{File,create_dir_all};
use std::io::prelude::*;

pub fn write_conf_file(beiboot_name: String, content: &str, file_type: &str) -> Result<String, String> {

    let dir = format!("/tmp/beiboot/{}", beiboot_name);
    let dir_result = create_dir_all(dir);
    match dir_result {
        Ok(_) => (),
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why));
        }
    }

    let path = format!("/tmp/beiboot/{}/{}", beiboot_name, file_type);
    let file_result = File::create(&path);
    let mut file = match file_result {
        Ok(file) => file,
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why));
        }
    };
    file.write_all(content.as_bytes()).unwrap();

    Ok(path)
}

pub fn cleanup(beiboot_name: String) -> Result<(), String> {
    let dir = format!("/tmp/beiboot/{}", beiboot_name);
    let dir_result = std::fs::remove_dir_all(dir);
    match dir_result {
        Ok(_) => Ok(()),
        Err(why) => {
            println!("{}", why);
            Err(format!("{}", why))
        }
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
        assert_eq!(result.unwrap(), "/tmp/beiboot/test/ca.crt".to_string());
        let cleanup_result = super::cleanup(beiboot_name);
        assert!(cleanup_result.is_ok());
    }
}
