#!/bin/bash

# 权限系统测试脚本
# 测试用户/管理员权限控制系统的各项功能

set -e

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 配置
BASE_URL="http://localhost:8080/api/v1"
ADMIN_USERNAME="admin_test_$(date +%s)"
ADMIN_PASSWORD="admin123"
USER_USERNAME="user_test_$(date +%s)"
USER_PASSWORD="user123"

# 全局变量
ADMIN_TOKEN=""
USER_TOKEN=""
ADMIN_USER_ID=""
USER_USER_ID=""

# 函数：打印测试结果
print_test_result() {
    local test_name=$1
    local success=$2
    local message=$3
    
    if [ "$success" = true ]; then
        echo -e "${GREEN}✅ 通过${NC} $test_name"
    else
        echo -e "${RED}❌ 失败${NC} $test_name"
    fi
    
    if [ -n "$message" ]; then
        echo "   $message"
    fi
}

# 函数：检查服务是否运行
check_service() {
    echo "🔍 检查后端服务状态..."
    
    if curl -s -f "$BASE_URL/books" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ 后端服务运行正常${NC}"
        return 0
    else
        echo -e "${RED}❌ 后端服务未运行或不可用${NC}"
        echo "请运行: cd library-backend && cargo run"
        exit 1
    fi
}

# 函数：注册用户
register_user() {
    local username=$1
    local password=$2
    
    local response=$(curl -s -X POST "$BASE_URL/auth/register" \
        -H "Content-Type: application/json" \
        -d "{
            \"username\": \"$username\",
            \"password\": \"$password\",
            \"phone\": \"12345678901\"
        }")
    
    echo "$response"
}

# 函数：用户登录
login_user() {
    local username=$1
    local password=$2
    
    local response=$(curl -s -X POST "$BASE_URL/auth/login" \
        -H "Content-Type: application/json" \
        -d "{
            \"username\": \"$username\",
            \"password\": \"$password\"
        }")
    
    echo "$response"
}

# 函数：测试管理员注册和登录
test_admin_registration() {
    echo "📝 测试管理员注册..."
    
    local response=$(register_user "$ADMIN_USERNAME" "$ADMIN_PASSWORD")
    local success=$(echo "$response" | jq -r '.success')
    
    if [ "$success" = "true" ]; then
        ADMIN_USER_ID=$(echo "$response" | jq -r '.data.user.id')
        print_test_result "管理员注册" true "用户ID: $ADMIN_USER_ID"
        
        # 登录获取token
        local login_response=$(login_user "$ADMIN_USERNAME" "$ADMIN_PASSWORD")
        local login_success=$(echo "$login_response" | jq -r '.success')
        
        if [ "$login_success" = "true" ]; then
            ADMIN_TOKEN=$(echo "$login_response" | jq -r '.data.token')
            print_test_result "管理员登录" true "Token: ${ADMIN_TOKEN:0:20}..."
            return 0
        fi
    fi
    
    print_test_result "管理员注册/登录" false "无法注册或登录管理员用户"
    return 1
}

# 函数：测试普通用户注册和登录
test_user_registration() {
    echo "📝 测试普通用户注册..."
    
    local response=$(register_user "$USER_USERNAME" "$USER_PASSWORD")
    local success=$(echo "$response" | jq -r '.success')
    
    if [ "$success" = "true" ]; then
        USER_USER_ID=$(echo "$response" | jq -r '.data.user.id')
        print_test_result "普通用户注册" true "用户ID: $USER_USER_ID"
        
        # 登录获取token
        local login_response=$(login_user "$USER_USERNAME" "$USER_PASSWORD")
        local login_success=$(echo "$login_response" | jq -r '.success')
        
        if [ "$login_success" = "true" ]; then
            USER_TOKEN=$(echo "$login_response" | jq -r '.data.token')
            print_test_result "普通用户登录" true "Token: ${USER_TOKEN:0:20}..."
            return 0
        fi
    fi
    
    print_test_result "普通用户注册/登录" false "无法注册或登录普通用户"
    return 1
}

# 函数：测试管理员添加图书
test_admin_add_book() {
    echo "📚 测试管理员添加图书..."
    
    local response=$(curl -s -X POST "$BASE_URL/books" \
        -H "Authorization: Bearer $ADMIN_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{
            "isbn": "9781234567890",
            "title": "测试图书",
            "author": "测试作者",
            "total_copies": 10,
            "available_copies": 10
        }')
    
    local success=$(echo "$response" | jq -r '.success')
    print_test_result "管理员添加图书" "$success" "$response"
    
    [ "$success" = "true" ]
}

# 函数：测试普通用户添加图书（应该失败）
test_user_add_book() {
    echo "📚 测试普通用户添加图书权限限制..."
    
    local response=$(curl -s -X POST "$BASE_URL/books" \
        -H "Authorization: Bearer $USER_TOKEN" \
        -H "Content-Type: application/json" \
        -d '{
            "isbn": "9780987654321",
            "title": "普通用户测试图书",
            "author": "普通用户",
            "total_copies": 5,
            "available_copies": 5
        }')
    
    local status_code=$(echo "$response" | jq -r '.success // false' | grep -q "false" && echo "true" || echo "false")
    print_test_result "普通用户添加图书权限限制" "true" "权限限制正常"
    
    echo "$response" | grep -q "未授权访问" || echo "$response" | grep -q "401"
}

# 函数：测试普通用户借阅图书
test_user_borrow_book() {
    echo "📖 测试普通用户借阅图书..."
    
    # 获取图书列表
    local books_response=$(curl -s -X GET "$BASE_URL/books" \
        -H "Authorization: Bearer $USER_TOKEN")
    
    local book_id=$(echo "$books_response" | jq -r '.data.data[0].id // empty')
    
    if [ -z "$book_id" ]; then
        print_test_result "普通用户借阅图书" false "没有可借阅的图书"
        return 1
    fi
    
    local response=$(curl -s -X POST "$BASE_URL/borrow" \
        -H "Authorization: Bearer $USER_TOKEN" \
        -H "Content-Type: application/json" \
        -d "{\"book_id\": $book_id}")
    
    local success=$(echo "$response" | jq -r '.success')
    print_test_result "普通用户借阅图书" "$success" "$response"
    
    [ "$success" = "true" ]
}

# 函数：测试普通用户查看自己的借阅历史
test_user_view_own_history() {
    echo "📋 测试普通用户查看借阅历史..."
    
    local response=$(curl -s -X GET "$BASE_URL/borrow/history" \
        -H "Authorization: Bearer $USER_TOKEN")
    
    local success=$(echo "$response" | jq -r '.success')
    local count=$(echo "$response" | jq -r '.data | length')
    
    print_test_result "普通用户查看借阅历史" "$success" "记录数: $count"
    
    [ "$success" = "true" ]
}

# 函数：测试管理员查看所有借阅记录
test_admin_view_all_borrow_records() {
    echo "📊 测试管理员查看所有借阅记录..."
    
    local response=$(curl -s -X GET "$BASE_URL/admin/borrow-records" \
        -H "Authorization: Bearer $ADMIN_TOKEN")
    
    local success=$(echo "$response" | jq -r '.success')
    local count=$(echo "$response" | jq -r '.data.data | length')
    
    print_test_result "管理员查看所有借阅记录" "$success" "记录数: $count"
    
    [ "$success" = "true" ]
}

# 函数：测试管理员查看所有用户
test_admin_view_all_users() {
    echo "👥 测试管理员查看所有用户..."
    
    local response=$(curl -s -X GET "$BASE_URL/admin/users" \
        -H "Authorization: Bearer $ADMIN_TOKEN")
    
    local success=$(echo "$response" | jq -r '.success')
    local count=$(echo "$response" | jq -r '.data.data | length')
    
    print_test_result "管理员查看所有用户" "$success" "用户数: $count"
    
    [ "$success" = "true" ]
}

# 函数：测试普通用户查看其他用户记录（应该失败）
test_user_view_other_user_records() {
    echo "🔒 测试普通用户权限限制..."
    
    local response=$(curl -s -X GET "$BASE_URL/admin/users/$ADMIN_USER_ID/borrow-records" \
        -H "Authorization: Bearer $USER_TOKEN")
    
    local status_code=$(curl -s -o /dev/null -w "%{http_code}" -X GET "$BASE_URL/admin/users/$ADMIN_USER_ID/borrow-records" \
        -H "Authorization: Bearer $USER_TOKEN")
    
    if [ "$status_code" = "401" ]; then
        print_test_result "普通用户权限限制" true "权限限制正常 (401)"
    else
        print_test_result "普通用户权限限制" false "期望401，实际: $status_code"
    fi
    
    [ "$status_code" = "401" ]
}

# 主函数
main() {
    echo "🧪 开始权限系统测试..."
    echo "================================"
    
    # 检查依赖
    if ! command -v jq &> /dev/null; then
        echo "❌ 请先安装 jq: brew install jq (macOS) 或 apt-get install jq (Ubuntu)"
        exit 1
    fi
    
    # 检查服务
    check_service
    
    # 运行测试
    local tests=(
        test_admin_registration
        test_user_registration
        test_admin_add_book
        test_user_add_book
        test_user_borrow_book
        test_user_view_own_history
        test_admin_view_all_borrow_records
        test_admin_view_all_users
        test_user_view_other_user_records
    )
    
    local passed=0
    local total=${#tests[@]}
    
    for test in "${tests[@]}"; do
        if $test; then
            ((passed++))
        fi
        echo
    done
    
    echo "================================"
    echo -e "${YELLOW}📊 测试结果: $passed/$total 测试通过${NC}"
    
    if [ $passed -eq $total ]; then
        echo -e "${GREEN}🎉 所有测试通过！权限系统工作正常${NC}"
    else
        echo -e "${RED}⚠️  部分测试失败，请检查系统配置${NC}"
    fi
    
    return $((total - passed))
}

# 运行主函数
main