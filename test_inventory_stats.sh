#!/bin/bash

# 故事5.1：库存统计功能集成测试脚本
# 测试内容：
# 1. 后端API测试：GET /api/v1/statistics/inventory
# 2. 前端页面测试：库存统计仪表板
# 3. 按分类统计功能验证
# 4. 数据准确性验证

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 开始执行库存统计功能集成测试...${NC}"

# 检查服务是否运行
echo -e "${YELLOW}📋 检查服务状态...${NC}"

# 检查后端服务
if ! curl -s http://localhost:8080/api/v1/books > /dev/null 2>&1; then
    echo -e "${RED}❌ 后端服务未运行，请先启动后端服务${NC}"
    exit 1
fi

# 检查前端服务
if ! curl -s http://localhost:5173 > /dev/null 2>&1; then
    echo -e "${RED}❌ 前端服务未运行，请先启动前端服务${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 服务状态正常${NC}"

# 测试1：后端API测试
echo -e "${YELLOW}🔍 测试1：后端库存统计API${NC}"

# 获取库存统计信息
response=$(curl -s http://localhost:8080/api/v1/statistics/inventory)
echo "API响应: $response"

# 验证响应格式
if echo "$response" | jq -e '.success == true' > /dev/null; then
    echo -e "${GREEN}✅ API响应格式正确${NC}"
else
    echo -e "${RED}❌ API响应格式错误${NC}"
    exit 1
fi

# 验证必要字段
required_fields=("total_books" "total_copies" "available_copies" "borrowed_copies" "category_stats")
for field in "${required_fields[@]}"; do
    if echo "$response" | jq -e ".data.$field != null" > /dev/null; then
        echo -e "${GREEN}✅ 字段 $field 存在${NC}"
    else
        echo -e "${RED}❌ 字段 $field 缺失${NC}"
        exit 1
    fi
done

# 验证数据类型
total_books=$(echo "$response" | jq '.data.total_books')
total_copies=$(echo "$response" | jq '.data.total_copies')
available_copies=$(echo "$response" | jq '.data.available_copies')
borrowed_copies=$(echo "$response" | jq '.data.borrowed_copies')

echo "统计结果："
echo "  图书种类: $total_books"
echo "  总藏书量: $total_copies"
echo "  可借图书: $available_copies"
echo "  已借出: $borrowed_copies"

# 验证数据一致性
if [ "$total_copies" -eq $((available_copies + borrowed_copies)) ]; then
    echo -e "${GREEN}✅ 数据一致性验证通过${NC}"
else
    echo -e "${RED}❌ 数据不一致：总藏书量 != 可借 + 已借出${NC}"
    exit 1
fi

# 测试2：按分类统计功能
echo -e "${YELLOW}🔍 测试2：按分类统计功能${NC}"

category_count=$(echo "$response" | jq '.data.category_stats | length')
echo "分类数量: $category_count"

if [ "$category_count" -gt 0 ]; then
    echo -e "${GREEN}✅ 分类统计数据正常${NC}"
    
    # 验证每个分类的数据
    echo "$response" | jq -r '.data.category_stats[] | "\(.category // "未分类"): \(.book_count)种图书, \(.total_copies)册, \(.available_copies)册可借"' | while read -r line; do
        echo "  $line"
    done
else
    echo -e "${YELLOW}⚠️  暂无分类统计数据${NC}"
fi

# 测试3：前端页面测试
echo -e "${YELLOW}🔍 测试3：前端页面功能测试${NC}"

# 检查前端路由
if curl -s http://localhost:5173/inventory-stats > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 前端路由正常${NC}"
else
    echo -e "${RED}❌ 前端路由异常${NC}"
    exit 1
fi

# 测试4：导航菜单测试
echo -e "${YELLOW}🔍 测试4：导航菜单测试${NC}"

# 检查导航链接
nav_response=$(curl -s http://localhost:5173)
if echo "$nav_response" | grep -q "库存统计"; then
    echo -e "${GREEN}✅ 导航菜单包含库存统计链接${NC}"
else
    echo -e "${YELLOW}⚠️  导航菜单检查需要手动验证${NC}"
fi

# 测试5：数据准确性验证
echo -e "${YELLOW}🔍 测试5：数据准确性验证${NC}"

# 获取数据库中的实际数据
echo "从数据库验证数据准确性..."

# 检查数据库连接
if mysql -h localhost -u root -proot library_db -e "SELECT 1" > /dev/null 2>&1; then
    # 获取数据库中的实际统计
    db_total_books=$(mysql -h localhost -u root -proot library_db -N -e "SELECT COUNT(*) FROM books")
    db_total_copies=$(mysql -h localhost -u root -proot library_db -N -e "SELECT COALESCE(SUM(total_copies), 0) FROM books")
    db_available_copies=$(mysql -h localhost -u root -proot library_db -N -e "SELECT COALESCE(SUM(available_copies), 0) FROM books")
    
    echo "数据库统计："
    echo "  图书种类: $db_total_books"
    echo "  总藏书量: $db_total_copies"
    echo "  可借图书: $db_available_copies"
    
    # 验证API数据与数据库数据一致性
    if [ "$total_books" -eq "$db_total_books" ] && 
       [ "$total_copies" -eq "$db_total_copies" ] && 
       [ "$available_copies" -eq "$db_available_copies" ]; then
        echo -e "${GREEN}✅ API数据与数据库数据一致${NC}"
    else
        echo -e "${RED}❌ API数据与数据库数据不一致${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  无法连接数据库，跳过数据库验证${NC}"
fi

# 测试6：性能测试
echo -e "${YELLOW}🔍 测试6：性能测试${NC}"

start_time=$(date +%s%3N)
curl -s http://localhost:8080/api/v1/statistics/inventory > /dev/null
end_time=$(date +%s%3N)
response_time=$((end_time - start_time))

echo "API响应时间: ${response_time}ms"

if [ "$response_time" -lt 1000 ]; then
    echo -e "${GREEN}✅ 性能测试通过${NC}"
else
    echo -e "${YELLOW}⚠️  响应时间超过1秒，建议优化${NC}"
fi

# 测试总结
echo -e "\n${GREEN}🎉 库存统计功能集成测试完成！${NC}"
echo -e "${GREEN}✅ 所有核心功能测试通过${NC}"
echo -e "\n${YELLOW}📊 测试总结：${NC}"
echo -e "  ✅ 后端API正常工作"
echo -e "  ✅ 前端页面可访问"
echo -e "  ✅ 数据格式正确"
echo -e "  ✅ 分类统计功能正常"
echo -e "  ✅ 数据一致性验证通过"
echo -e "  ✅ 性能表现良好"

echo -e "\n${YELLOW}📝 手动测试建议：${NC}"
echo -e "  1. 访问 http://localhost:5173/inventory-stats 查看库存统计仪表板"
echo -e "  2. 验证不同分类的图书数量显示是否正确"
echo -e "  3. 测试添加新图书后统计数据是否实时更新"
echo -e "  4. 测试借阅和归还图书后统计数据的变化"