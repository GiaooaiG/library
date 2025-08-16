#!/bin/bash

# 续借功能测试脚本
echo "🧪 开始测试续借功能..."

# 设置变量
BASE_URL="http://localhost:8080/api/v1"
USERNAME="test_renewal_user"
PASSWORD="test123456"
BOOK_ISBN="9787121399999"

# 颜色输出
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 检查服务是否运行
echo "检查服务状态..."
if ! curl -s "$BASE_URL/books" > /dev/null; then
    echo -e "${RED}❌ 服务未运行，请先启动后端服务${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 服务运行正常${NC}"

# 注册用户
echo "注册用户..."
curl -s -X POST "$BASE_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\",\"phone\":\"13800138000\"}" > /dev/null

# 登录获取token
echo "登录获取token..."
TOKEN=$(curl -s -X POST "$BASE_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"$USERNAME\",\"password\":\"$PASSWORD\"}" | \
  jq -r '.data.token')

if [ "$TOKEN" == "null" ] || [ -z "$TOKEN" ]; then
    echo -e "${RED}❌ 登录失败${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 登录成功${NC}"

# 创建测试图书
echo "创建测试图书..."
BOOK_ID=$(curl -s -X POST "$BASE_URL/books" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "{\"isbn\":\"$BOOK_ISBN\",\"title\":\"续借测试图书\",\"author\":\"测试作者\",\"category\":\"测试\",\"total_copies\":5,\"available_copies\":5}" | \
  jq -r '.data.id')

if [ "$BOOK_ID" == "null" ] || [ -z "$BOOK_ID" ]; then
    echo -e "${RED}❌ 创建图书失败${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 创建图书成功，ID: $BOOK_ID${NC}"

# 借阅图书
echo "借阅图书..."
BORROW_ID=$(curl -s -X POST "$BASE_URL/borrow" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d "{\"book_id\":$BOOK_ID}" | \
  jq -r '.data.id')

if [ "$BORROW_ID" == "null" ] || [ -z "$BORROW_ID" ]; then
    echo -e "${RED}❌ 借阅图书失败${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 借阅成功，记录ID: $BORROW_ID${NC}"

# 测试续借
echo "测试续借功能..."
RESPONSE=$(curl -s -X POST "$BASE_URL/renew/$BORROW_ID" \
  -H "Authorization: Bearer $TOKEN")

if echo "$RESPONSE" | grep -q '"success":true'; then
    echo -e "${GREEN}✅ 续借成功${NC}"
    echo "响应: $RESPONSE"
else
    echo -e "${RED}❌ 续借失败${NC}"
    echo "响应: $RESPONSE"
    exit 1
fi

# 测试重复续借
echo "测试重复续借..."
RESPONSE2=$(curl -s -X POST "$BASE_URL/renew/$BORROW_ID" \
  -H "Authorization: Bearer $TOKEN")

if echo "$RESPONSE2" | grep -q '"续借次数已达上限"'; then
    echo -e "${GREEN}✅ 重复续借被拒绝，符合预期${NC}"
else
    echo -e "${RED}❌ 重复续借测试失败${NC}"
    echo "响应: $RESPONSE2"
fi

# 归还图书
echo "归还图书..."
RESPONSE3=$(curl -s -X POST "$BASE_URL/return/$BORROW_ID" \
  -H "Authorization: Bearer $TOKEN")

if echo "$RESPONSE3" | grep -q '"success":true'; then
    echo -e "${GREEN}✅ 归还成功${NC}"
else
    echo -e "${RED}❌ 归还失败${NC}"
    echo "响应: $RESPONSE3"
fi

echo -e "${GREEN}🎉 续借功能测试完成！${NC}"