-- 删除续借次数索引
DROP INDEX idx_renewal_count ON borrow_records;

-- 删除预约表
DROP TABLE IF EXISTS reservations;

-- 删除续借次数字段
ALTER TABLE borrow_records DROP COLUMN renewal_count;
