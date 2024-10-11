use anyhow::Error;
use base64::Engine;
use base64::engine::general_purpose;

pub(crate)  fn base64_decode(data: String) -> Result<String, Error> {
    match general_purpose::STANDARD.decode(data){
        Ok(result) => match String::from_utf8(result){
            Ok(result) => Ok(result),
            Err(err) => Err(err.into())
        },
        Err(err) => Err(err.into())
    }
}
