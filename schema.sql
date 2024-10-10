--创建用户表
-- DROP TABLE IF EXISTS users;
CREATE TABLE
    users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        is_deleted INTEGER DEFAULT NULL,
        CONSTRAINT idx_name UNIQUE (name, email)
        );

CREATE INDEX idx_name ON users (name);
CREATE INDEX idx_email ON users (email);
CREATE INDEX idx_is_deleted ON users (is_deleted);
CREATE INDEX idx_created_at ON users (created_at);
CREATE INDEX idx_updated_at ON users (updated_at);