# 数据库配置指南（MySQL 8.4 LTS + 密码123456）

## 1. 数据库配置

### 1.1 创建数据库
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

### 1.2 环境变量配置
在项目根目录创建以下文件：

**library-backend/.env**
```bash
DATABASE_URL=mysql://library_user:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

**library-frontend/.env**
```bash
VITE_API_URL=http://localhost:8080/api/v1
```

## 2. 一键配置脚本

### 2.1 创建配置脚本
创建 `setup-database.sh`：

```bash
#!/bin/bash
echo "🔧 配置图书馆管理系统数据库..."

# 创建.env文件
cat > library-backend/.env << EOF
DATABASE_URL=mysql://root:123456@localhost/library_db
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
EOF

cat > library-frontend/.env << EOF
VITE_API_URL=http://localhost:8080/api/v1
EOF

echo "✅ 配置文件已创建！"
echo "📋 下一步：运行数据库迁移"
```

### 2.2 Windows版本
创建 `setup-database.bat`：

```batch
@echo off
echo 🔧 配置图书馆管理系统数据库...

echo DATABASE_URL=mysql://root:123456@localhost/library_db > library-backend\.env
echo SERVER_HOST=127.0.0.1 >> library-backend\.env
echo SERVER_PORT=8080 >> library-backend\.env

echo VITE_API_URL=http://localhost:8080/api/v1 > library-frontend\.env

echo ✅ 配置文件已创建！
pause
```

## 3. 验证配置

### 3.1 测试数据库连接
```bash
mysql -u root -p123456 -e "USE library_db; SELECT '连接成功';"
```

### 3.2 验证环境变量
```bash
# 在library-backend目录
cat .env
# 应该显示：
# DATABASE_URL=mysql://root:123456@localhost/library_db
# SERVER_HOST=127.0.0.1
# SERVER_PORT=8080
```

## 4. 下一步行动
1. ✅ 数据库配置完成
2. 🔄 运行数据库迁移
3. 🔄 创建第一个用户故事
4. 🔄 开发基础API