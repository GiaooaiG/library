# OpenGauss数据库迁移指南

## 使用Diesel进行OpenGauss数据库初始化

### 1. Diesel与OpenGauss兼容性

Diesel ORM完全支持OpenGauss，因为：
- OpenGauss兼容PostgreSQL协议
- Diesel的PostgreSQL后端可以直接连接OpenGauss
- 迁移系统保持不变，只需更新SQL语法

### 2. 数据库初始化步骤

#### 2.1 安装Diesel CLI工具
```bash
# 安装支持PostgreSQL的Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

#### 2.2 配置环境变量
确保你的`.env`文件配置正确：
```bash
DATABASE_URL=postgres://opengauss:opengaussDB@123456@localhost:15432/library_db
```

#### 2.3 创建数据库
```bash
# 使用psql或docker exec连接到OpenGauss创建数据库
docker exec -it opengauss gosu opengauss psql -U opengauss -c "CREATE DATABASE library_db;"
```

#### 2.4 运行迁移
```bash
cd library-backend

# 运行所有迁移
diesel migration run

# 或者重新做迁移
diesel migration revert
diesel migration run

# 验证迁移状态
diesel migration list
```

#### 2.5 生成新的schema.rs
```bash
# 重新生成schema文件
diesel print-schema > src/schema.rs
```

### 3. 迁移脚本验证

检查你的迁移文件是否正确适配OpenGauss：

#### 3.1 表创建迁移
```sql
-- 2025-08-15-032211_create_tables/up.sql
-- 确保使用PostgreSQL语法
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    isbn VARCHAR(13) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    category VARCHAR(100),
    publisher VARCHAR(255),
    total_copies INTEGER DEFAULT 1,
    available_copies INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_isbn ON books(isbn);
CREATE INDEX idx_category ON books(category);
```

#### 3.2 数据类型映射
| MySQL类型 | OpenGauss类型 | 说明 |
|-----------|---------------|------|
| INT | INTEGER | 直接映射 |
| VARCHAR | VARCHAR | 直接映射 |
| TIMESTAMP | TIMESTAMP | 直接映射 |
| ENUM | VARCHAR + CHECK | 需要约束替代 |

### 4. 常见问题解决

#### 4.1 连接问题
```bash
# 测试连接
psql postgres://opengauss:opengaussDB@123456@localhost:15432/library_db

# 如果连接失败，检查Docker容器状态
docker ps
docker logs opengauss
```

#### 4.2 权限问题
```sql
-- 确保opengauss用户有创建数据库的权限
GRANT ALL PRIVILEGES ON DATABASE library_db TO opengauss;
```

#### 4.3 迁移失败
```bash
# 查看详细的Diesel错误信息
RUST_BACKTRACE=1 diesel migration run

# 手动清理数据库重新迁移
diesel migration revert --all
diesel migration run
```

### 5. 验证数据库初始化

#### 5.1 检查表结构
```bash
# 连接到OpenGauss
docker exec -it opengauss gosu opengauss psql -U opengauss -d library_db

# 查看表列表
\dt

# 查看表结构
\d books
\d users
\d borrow_records
\d reservations
```

#### 5.2 验证数据插入
```sql
-- 测试数据插入
INSERT INTO books (isbn, title, author, category, publisher, total_copies, available_copies) 
VALUES ('9787115546081', 'Rust权威指南', 'Steve Klabnik', '编程', '人民邮电出版社', 5, 5);

-- 验证数据
SELECT * FROM books;
```

### 6. 后端服务启动

#### 6.1 编译检查
```bash
cd library-backend
cargo check
cargo build
```

#### 6.2 启动服务
```bash
cargo run
```

#### 6.3 测试API
```bash
# 测试健康检查端点
curl http://localhost:8080/health

# 测试用户注册
curl -X POST http://localhost:8080/api/users/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"password123","phone":"13800138000"}'
```

### 7. 完整验证流程

1. ✅ Docker环境准备
2. ✅ OpenGauss服务启动
3. ✅ 数据库创建
4. ✅ Diesel迁移执行
5. ✅ 表结构验证
6. ✅ 后端服务启动
7. ✅ API功能测试
8. ✅ 前端连接测试

### 8. 故障排除

#### 8.1 Diesel连接失败
- 检查数据库URL格式
- 确认OpenGauss服务运行状态
- 验证用户名密码正确性

#### 8.2 迁移脚本错误
- 检查SQL语法是否符合PostgreSQL标准
- 确认表名、字段名大小写一致性
- 验证外键约束定义

#### 8.3 编译错误
- 检查Diesel特性是否正确设置为postgres
- 确认schema.rs文件已更新
- 验证模型定义与数据库结构匹配

### 9. 最佳实践

1. **备份策略**: 迁移前备份所有数据
2. **版本控制**: 将迁移文件纳入Git管理
3. **环境隔离**: 开发、测试、生产环境分离
4. **监控日志**: 启用详细日志记录
5. **性能测试**: 迁移后进行性能基准测试