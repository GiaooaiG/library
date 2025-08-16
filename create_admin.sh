#!/bin/bash

# 创建管理员用户的Shell脚本
# 使用curl直接调用API创建管理员

set -e

# 颜色输出
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# 配置
BASE_URL="http://localhost:8080/api/v1"
ADMIN_USERNAME="admin"
ADMIN_PASSWORD="admin123"

echo -e "${YELLOW}🔧 创建管理员用户...${NC}"

# 检查服务是否运行
if ! curl -s -f "$BASE_URL/books" > /dev/null 2>&1; then
    echo -e "${RED}❌ 后端服务未运行，请先启动服务${NC}"
    echo "运行: cd library-backend && cargo run"
    exit 1
fi

# 检查是否安装了jq
if ! command -v jq &> /dev/null; then
    echo -e "${RED}❌ 请先安装 jq${NC}"
    echo "macOS: brew install jq"
    echo "Ubuntu: sudo apt-get install jq"
    exit 1
fi

# 注册管理员用户（默认是普通用户）
echo "📝 注册管理员账号..."
register_response=$(curl -s -X POST "$BASE_URL/auth/register" \
    -H "Content-Type: application/json" \
    -d "{
        \"username\": \"$ADMIN_USERNAME\",
        \"password\": \"$ADMIN_PASSWORD\",
        \"phone\": \"12345678901\"
    }")

# 检查注册结果
if echo "$register_response" | jq -r '.success' | grep -q "true"; then
    echo -e "${GREEN}✅ 用户注册成功${NC}"
else
    echo -e "${RED}❌ 注册失败: $register_response${NC}"
    exit 1
fi

# 登录获取token
echo "🔑 登录管理员账号..."
login_response=$(curl -s -X POST "$BASE_URL/auth/login" \
    -H "Content-Type: application/json" \
    -d "{
        \"username\": \"$ADMIN_USERNAME\",
        \"password\": \"$ADMIN_PASSWORD\"
    }")

if echo "$login_response" | jq -r '.success' | grep -q "true"; then
    token=$(echo "$login_response" | jq -r '.data.token')
    echo -e "${GREEN}✅ 登录成功${NC}"
else
    echo -e "${RED}❌ 登录失败: $login_response${NC}"
    exit 1
fi

# 注意：由于系统默认注册的用户都是普通用户，我们需要手动修改数据库
echo -e "${YELLOW}⚠️  注意：由于系统限制，注册的用户默认为普通用户${NC}"
echo -e "${YELLOW}   需要手动将用户角色改为admin${NC}"
echo ""
echo "请执行以下SQL命令将用户角色改为admin："
echo "================================"
echo "USE library;"
echo "UPDATE users SET role = 'admin' WHERE username = '$ADMIN_USERNAME';"
echo "================================"
echo ""
echo "或者使用Python脚本："
echo "python3 create_admin_user.py admin admin123"
echo ""
echo "📋 管理员账号信息："
echo "用户名: $ADMIN_USERNAME"
echo "密码: $ADMIN_PASSWORD"
echo "登录地址: http://localhost:5173/login"