use std::fs::{File,create_dir_all};
use std::io::prelude::*;

pub fn write_conf_file(beiboot_name: String, content: &str, file_type: &str) -> Result<String, String> {

    let dir = format!("/tmp/beiboot/{}", beiboot_name);
    let dir_result = create_dir_all(&dir);
    match dir_result {
        Ok(_) => (),
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why).into());
        }
    }

    let path = format!("/tmp/beiboot/{}/{}", beiboot_name, file_type);
    let file_result = File::create(&path);
    let mut file = match file_result {
        Ok(file) => file,
        Err(why) => {
            println!("{}", why);
            return Err(format!("{}", why).into());
        }
    };
    file.write_all(content.as_bytes()).unwrap();

    Ok(path.into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_tls_files() {
        let beiboot_name = "test".to_string();
        let content = "test".to_string();
        let cert_type = "ca.crt".to_string();
        let result = super::write_conf_file(beiboot_name, &content, &cert_type);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "/tmp/beiboot/test/ca.crt".to_string());
    }
}
