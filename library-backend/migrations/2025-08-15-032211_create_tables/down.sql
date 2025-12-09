-- 删除测试数据
DELETE FROM reservations WHERE user_id IN (SELECT id FROM users WHERE username = 'admin');
DELETE FROM borrow_records WHERE user_id IN (SELECT id FROM users WHERE username = 'admin');
DELETE FROM books WHERE isbn IN ('9787115546081', '9787111684107');
DELETE FROM users WHERE username = 'admin';

-- 删除触发器
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_books_updated_at ON books;

-- 删除触发器函数
DROP FUNCTION IF EXISTS update_updated_at_column();

-- 删除索引
DROP INDEX IF EXISTS idx_reservations_status;
DROP INDEX IF EXISTS idx_reservations_book_id;
DROP INDEX IF EXISTS idx_reservations_user_id;
DROP INDEX IF EXISTS idx_borrow_records_status;
DROP INDEX IF EXISTS idx_borrow_records_book_id;
DROP INDEX IF EXISTS idx_borrow_records_user_id;
DROP INDEX IF EXISTS idx_books_author;
DROP INDEX IF EXISTS idx_books_title;
DROP INDEX IF EXISTS idx_books_isbn;
DROP INDEX IF EXISTS idx_users_username;

-- 删除表
DROP TABLE IF EXISTS reservations;
DROP TABLE IF EXISTS borrow_records;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS users;