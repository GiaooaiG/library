# 图书馆管理系统 - 开发环境配置指南

## 1. 数据库配置（MySQL 8.4 LTS）

### 1.1 安装MySQL 8.4 LTS
- **Windows**: 下载 [MySQL Installer](https://dev.mysql.com/downloads/installer/)
- **Mac**: `brew install mysql@8.4`
- **Linux**: `sudo apt install mysql-server-8.4`

### 1.2 数据库配置
```sql
-- 登录MySQL
mysql -u root -p

-- 创建数据库
CREATE DATABASE library_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 创建用户
CREATE USER 'library_user'@'localhost' IDENTIFIED BY '123456';
GRANT ALL PRIVILEGES ON library_db.* TO 'library_user'@'localhost';
FLUSH PRIVILEGES;
```

### 1.3 环境变量配置
在项目根目录创建 `.env` 文件：

```bash
# 数据库配置
DATABASE_URL=mysql://library_user:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
JWT_SECRET=your-secret-key-here

# 前端配置
VITE_API_URL=http://localhost:8080/api/v1
```

## 2. 快速启动

### 2.1 一键启动脚本
创建 `start-dev.sh` (Linux/Mac) 或 `start-dev.bat` (Windows):

```bash
#!/bin/bash
# start-dev.sh

echo "🚀 启动图书馆管理系统..."

# 启动后端
echo "📦 启动后端服务..."
cd library-backend
cargo run &

# 等待后端启动
sleep 5

# 启动前端
echo "🎨 启动前端服务..."
cd ../library-frontend
npm run dev &

echo "✅ 系统已启动！"
echo "📱 前端: http://localhost:5173"
echo "🔧 后端: http://localhost:8080"
```

### 2.2 Windows批处理
```batch
@echo off
echo 🚀 启动图书馆管理系统...

start cmd /k "cd library-backend && cargo run"
timeout /t 5 /nobreak >nul
start cmd /k "cd library-frontend && npm run dev"

echo ✅ 系统已启动！
pause
```

## 3. 管理员账号

系统已预设管理员账号，可直接使用：

- **用户名**: `admin`
- **密码**: `password123`
- **角色**: 管理员（拥有所有权限）

### 3.1 登录方式
1. **前端登录**: 访问 `http://localhost:5173/login`
2. **API登录**: 
   ```bash
   curl -X POST http://localhost:8080/api/v1/auth/login \
     -H "Content-Type: application/json" \
     -d '{"username":"admin","password":"password123"}'
   ```

### 3.2 管理员权限
- ✅ 添加/编辑/删除图书
- ✅ 查看所有用户
- ✅ 查看所有借阅记录
- ✅ 管理系统设置

## 4. 验证步骤

### 4.1 数据库验证
```bash
# 测试数据库连接
mysql -u library_user -p123456 -e "USE library_db; SHOW TABLES;"
```

### 4.2 后端验证
```bash
cd library-backend
cargo check
cargo run
# 访问: http://localhost:8080/api/v1/books
```

### 4.3 前端验证
```bash
cd library-frontend
npm install  # 首次运行需要
npm run dev
# 访问: http://localhost:5173
```

## 5. 首次使用

### 5.1 管理员登录
1. 启动系统后访问前端页面
2. 使用管理员账号登录
3. 验证管理员功能（如添加图书）

### 5.2 创建测试用户
```bash
# 创建普通用户
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","password":"test123","phone":"13800138000"}'
```

## 6. 常见问题

### 6.1 依赖安装
```bash
# 后端依赖
cd library-backend
cargo build

# 前端依赖
cd library-frontend
npm install
```

### 6.2 端口冲突
- 后端端口：8080
- 前端端口：5173
- 如冲突，修改.env文件中的端口配置

### 6.3 权限问题
- 确保MySQL用户有足够权限
- 检查.env文件配置是否正确