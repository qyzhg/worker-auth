--创建用户表
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_deleted INTEGER DEFAULT NULL
);
-- 创建约束
CREATE UNIQUE INDEX idx_name_active ON users (name) WHERE is_deleted IS NULL;
CREATE UNIQUE INDEX idx_email_active ON users (email) WHERE is_deleted IS NULL;

-- 其他索引
CREATE INDEX idx_is_deleted ON users (is_deleted);
CREATE INDEX idx_created_at ON users (created_at);
CREATE INDEX idx_updated_at ON users (updated_at);
