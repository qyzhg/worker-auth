// use argon2::{
//     password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
//     Argon2,
// };
// use rand_core::OsRng; // 用于生成安全的随机数
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 用户密码（实际应用中请使用更复杂的密码）
//     let password = b"hunter42";
//
//     // 生成一个随机盐（每个密码应该有唯一的盐）
//     let salt = SaltString::generate(&mut OsRng);
//
//     // 创建 Argon2 默认实例，使用 Argon2id v1.3 算法（推荐用于密码哈希）
//     let argon2 = Argon2::default();
//
//     // 将密码和盐一起哈希，生成符合 PHC 规范的字符串 ($argon2id$v=19$...)
//     let password_hash = argon2.hash_password(password, &salt)?.to_string();
//
//     println!("Hashed password: {}", password_hash);
//
//     // 验证密码
//     let parsed_hash = PasswordHash::new(&password_hash)?;
//
//     // 使用 Argon2 验证用户输入的密码是否正确
//     assert!(argon2.verify_password(password, &parsed_hash).is_ok());
//     println!("Password verified successfully!");
//
//     Ok(())
// }
