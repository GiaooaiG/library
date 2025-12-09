-- 创建用户表
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(6) DEFAULT 'user' CHECK (role IN ('admin', 'user')),
    phone VARCHAR(20),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 创建图书表
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    isbn VARCHAR(13) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    category VARCHAR(100),
    publisher VARCHAR(255),
    total_copies INTEGER DEFAULT 1 CHECK (total_copies >= 0),
    available_copies INTEGER DEFAULT 1 CHECK (available_copies >= 0),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_books_available CHECK (available_copies <= total_copies)
);

-- 创建借阅记录表
CREATE TABLE borrow_records (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    book_id INTEGER NOT NULL,
    borrow_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMP NOT NULL,
    return_date TIMESTAMP,
    status VARCHAR(8) DEFAULT 'borrowed' CHECK (status IN ('borrowed', 'returned', 'overdue')),
    renewal_count INTEGER DEFAULT 0 CHECK (renewal_count >= 0),
    CONSTRAINT fk_borrow_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_borrow_book FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE
);

-- 创建索引优化查询
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_books_isbn ON books(isbn);
CREATE INDEX idx_books_title ON books(title);
CREATE INDEX idx_books_author ON books(author);
CREATE INDEX idx_borrow_records_user_id ON borrow_records(user_id);
CREATE INDEX idx_borrow_records_book_id ON borrow_records(book_id);
CREATE INDEX idx_borrow_records_status ON borrow_records(status);

-- 创建复合索引
CREATE INDEX idx_books_category_author ON books(category, author);
CREATE INDEX idx_borrow_records_user_status ON borrow_records(user_id, status);
CREATE INDEX idx_borrow_records_book_status ON borrow_records(book_id, status);
CREATE INDEX idx_borrow_records_date_status ON borrow_records(borrow_date, status);
-- 创建触发器函数，用于更新 updated_at 字段
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为用户表添加触发器
CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE update_updated_at_column();

-- 为图书表添加触发器
CREATE TRIGGER update_books_updated_at 
    BEFORE UPDATE ON books 
    FOR EACH ROW 
    EXECUTE PROCEDURE update_updated_at_column();

-- 插入测试数据
INSERT INTO books (isbn, title, author, category, publisher, total_copies, available_copies) VALUES
('9787115546081', 'Rust权威指南', 'Steve Klabnik', '编程', '人民邮电出版社', 5, 5),
('9787111684107', 'Rust编程之道', '张汉东', '编程', '机械工业出版社', 3, 3);

INSERT INTO users (username, password_hash, role, phone) VALUES
('admin', '$argon2id$v=19$m=19456,t=2,p=1$TThr0srQoYuCaEIcgAhLWw$JMi77O7Kf1aZQkZFiCknxpeO+O09vpvHy0zltHiBq3g', 'admin', '13800138000');