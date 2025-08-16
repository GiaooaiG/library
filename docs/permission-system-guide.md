# 权限系统使用指南

## 概述
本指南介绍了完整的用户/管理员权限控制系统的使用方法，包括后端权限中间件、管理员专属功能和前端权限控制。

## 系统架构

### 角色定义
- **管理员 (Admin)**: 拥有系统所有权限
- **普通用户 (User)**: 只能查看和操作自己的数据

### 权限控制层级
1. **后端权限中间件**: 基于JWT令牌的RBAC系统
2. **API权限检查**: 每个敏感操作都进行权限验证
3. **前端权限控制**: 根据角色显示不同界面

## 后端权限中间件

### 权限检查函数
- `check_admin_role(claims)`: 检查是否为管理员
- `check_user_ownership(claims, target_user_id)`: 检查用户数据访问权限
- `is_admin(claims)`: 判断是否为管理员

### 受保护的路由
- `POST /api/v1/books`: 仅管理员可添加图书
- `GET /api/v1/admin/users`: 仅管理员可查看所有用户
- `GET /api/v1/admin/borrow-records`: 仅管理员可查看所有借阅记录
- `GET /api/v1/admin/users/:id/borrow-records`: 管理员可查看任意用户记录，普通用户只能查看自己的

## 管理员专属功能

### 1. 添加图书
```bash
# 管理员添加图书
curl -X POST http://localhost:8080/api/v1/books \
  -H "Authorization: Bearer <admin_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "isbn": "9781234567890",
    "title": "新图书",
    "author": "作者名",
    "total_copies": 10,
    "available_copies": 10
  }'
```

### 2. 查看所有用户
```bash
# 管理员查看所有用户
curl -X GET http://localhost:8080/api/v1/admin/users \
  -H "Authorization: Bearer <admin_token>"
```

### 3. 查看所有借阅记录
```bash
# 管理员查看所有借阅记录
curl -X GET http://localhost:8080/api/v1/admin/borrow-records \
  -H "Authorization: Bearer <admin_token>"
```

## 用户权限限制

### 普通用户权限
- ✅ 查看图书列表
- ✅ 借阅图书
- ✅ 查看自己的借阅历史
- ✅ 归还图书
- ✅ 续借图书
- ❌ 添加图书
- ❌ 查看其他用户记录
- ❌ 查看所有用户列表

### 权限验证示例
```bash
# 普通用户尝试添加图书（会失败）
curl -X POST http://localhost:8080/api/v1/books \
  -H "Authorization: Bearer <user_token>" \
  -H "Content-Type: application/json" \
  -d '{...}'
# 返回: 401 Unauthorized
```

## 前端权限控制

### 导航菜单
- **管理员**: 显示"添加图书"、"用户管理"、"借阅记录"
- **普通用户**: 仅显示"借阅历史"

### 路由保护
- `/admin/*`: 仅管理员可访问
- `/borrow-history`: 登录用户可访问
- `/`: 根据角色显示不同内容

### 权限工具函数
```typescript
// 检查是否为管理员
authUtils.isAdmin()

// 检查是否可以访问用户数据
authUtils.canAccessUserData(targetUserId)
```

## 测试验证

### 运行权限测试
```bash
# 使用Python测试脚本
python3 test_permission_system.py

# 使用Shell测试脚本
chmod +x test_permission_system.sh
./test_permission_system.sh
```

### 手动测试步骤
1. **启动后端服务**:
   ```bash
   cd library-backend
   cargo run
   ```

2. **注册管理员用户**:
   ```bash
   curl -X POST http://localhost:8080/api/v1/auth/register \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "admin123", "phone": "12345678901"}'
   ```

3. **注册普通用户**:
   ```bash
   curl -X POST http://localhost:8080/api/v1/auth/register \
     -H "Content-Type: application/json" \
     -d '{"username": "user", "password": "user123", "phone": "12345678901"}'
   ```

4. **测试权限**:
   - 管理员登录后尝试添加图书
   - 普通用户登录后尝试添加图书（应失败）
   - 普通用户查看自己的借阅历史
   - 管理员查看所有用户的借阅记录

## 数据库权限设置

### 创建管理员用户
```sql
-- 手动创建管理员用户（需要直接在数据库中设置role为'admin'）
INSERT INTO users (username, password_hash, role, phone) 
VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$...', 'admin', '12345678901');
```

## 常见问题解决

### 1. 权限验证失败
- 检查JWT令牌是否正确
- 确认用户角色是否正确设置
- 检查Authorization头格式

### 2. 管理员功能无法使用
- 确认用户role字段为'admin'
- 检查数据库中用户记录
- 验证token中的角色信息

### 3. 跨域问题
- 确保前端和后端配置正确的CORS设置
- 检查API_URL环境变量

## 安全最佳实践

1. **令牌管理**: JWT令牌应设置合理的过期时间
2. **密码安全**: 使用强密码策略
3. **权限最小化**: 只授予必要的权限
4. **审计日志**: 记录重要操作
5. **输入验证**: 所有输入都应进行验证

## API响应示例

### 成功响应
```json
{
  "success": true,
  "message": "操作成功",
  "data": {...}
}
```

### 权限错误响应
```json
{
  "success": false,
  "message": "未授权访问"
}
```

## 环境变量配置

### 后端
```bash
JWT_SECRET=your-secret-key
DATABASE_URL=mysql://user:password@localhost/library
```

### 前端
```bash
VITE_API_URL=http://localhost:8080/api/v1
```

## 总结
本权限系统实现了完整的RBAC（基于角色的访问控制），确保：
- 管理员拥有完整的管理权限
- 普通用户只能访问自己的数据
- 系统具有良好的安全性和可扩展性
- 前后端权限控制保持一致