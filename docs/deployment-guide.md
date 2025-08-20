# 图书馆管理系统部署指南

## 系统要求

### 后端要求
- Rust 1.75+
- MySQL 8.0+
- Diesel CLI

### 前端要求
- Node.js 18+
- npm 或 yarn

## 快速部署

### 1. 数据库设置

#### 1.1 创建数据库和用户
```sql
CREATE DATABASE library_db;
CREATE USER 'library_user'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON library_db.* TO 'library_user'@'localhost';
FLUSH PRIVILEGES;
```

#### 1.2 运行数据库迁移
```bash
cd library-backend
cargo install diesel_cli --no-default-features --features mysql
diesel migration run
```

### 2. 环境配置

#### 2.1 后端配置
创建 `library-backend/.env` 文件：
```env
DATABASE_URL=mysql://library_user:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
JWT_SECRET=your-secret-key-here
```

#### 2.2 前端配置
创建 `library-frontend/.env` 文件：
```env
VITE_API_URL=http://localhost:8080/api/v1
```

### 3. 启动服务

#### 3.1 启动后端
```bash
cd library-backend
cargo run
# 后端服务将在 http://localhost:8080 启动
```

#### 3.2 启动前端
```bash
cd library-frontend
npm install
npm run dev
# 前端应用将在 http://localhost:5173 启动
```

#### 3.3 生产环境构建
```bash
# 前端生产构建
cd library-frontend
npm run build
# 构建后的文件在 dist/ 目录
```

## 管理员账号

系统已预设管理员账号，部署完成后可直接使用：

- **用户名**: `admin`
- **密码**: `123456`
- **权限**: 完整管理员权限

### 验证管理员功能
```bash
# 管理员登录
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"123456"}'

# 使用返回的token测试管理员功能
curl -X GET http://localhost:8080/api/v1/admin/users \
  -H "Authorization: Bearer <admin_token>"
```

## 功能验证

### 1. 基础功能测试
```bash
# 测试图书列表
curl http://localhost:8080/api/v1/books

# 测试管理员添加图书
curl -X POST http://localhost:8080/api/v1/books \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "isbn": "9787123456789",
    "title": "测试图书",
    "author": "测试作者",
    "total_copies": 5,
    "available_copies": 5
  }'
```

### 2. 权限测试
```bash
# 创建普通用户
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"test123","phone":"13800138000"}'

# 普通用户尝试管理员操作（应失败）
curl -X POST http://localhost:8080/api/v1/books \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json" \
  -d '{...}'
```

## 故障排除

### 常见问题

#### 1. 数据库连接失败
- 检查MySQL服务是否运行
- 确认用户名密码正确
- 检查数据库权限

#### 2. 端口冲突
- 后端端口：8080
- 前端端口：5173
- 使用 `lsof -i :8080` 检查端口占用

#### 3. 权限问题
- 确认管理员账号存在：检查数据库users表
- 验证JWT_SECRET配置正确
- 检查CORS配置

### 系统检查清单
- [ ] MySQL服务运行正常
- [ ] 数据库迁移成功
- [ ] 后端服务启动无错误
- [ ] 前端服务启动无错误
- [ ] 管理员账号可正常登录
- [ ] 权限系统工作正常