-- 添加续借次数字段到借阅记录表
ALTER TABLE borrow_records ADD COLUMN renewal_count INT DEFAULT 0;

-- 创建预约表
CREATE TABLE reservations (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    book_id INT NOT NULL,
    reservation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    status ENUM('active', 'cancelled', 'fulfilled') DEFAULT 'active',
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    INDEX idx_user_id (user_id),
    INDEX idx_book_id (book_id),
    INDEX idx_status (status),
    UNIQUE KEY unique_active_reservation (user_id, book_id, status)
);

-- 更新索引以包含续借次数
CREATE INDEX idx_renewal_count ON borrow_records(renewal_count);
