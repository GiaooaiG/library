#!/usr/bin/env python3
"""
权限系统测试验证脚本
测试用户/管理员权限控制系统的各项功能
"""

import requests
import json
import sys
from datetime import datetime

# 配置
BASE_URL = "http://localhost:8080/api/v1"
ADMIN_USERNAME = "admin_test"
ADMIN_PASSWORD = "admin123"
USER_USERNAME = "user_test"
USER_PASSWORD = "user123"

# 全局变量
admin_token = None
user_token = None
admin_user_id = None
user_user_id = None

def print_test_result(test_name, success, message=""):
    """打印测试结果"""
    status = "✅ 通过" if success else "❌ 失败"
    print(f"{status} {test_name}")
    if message:
        print(f"   {message}")

def register_user(username, password, role=None):
    """注册用户"""
    data = {
        "username": username,
        "password": password,
        "phone": "12345678901"
    }
    
    response = requests.post(f"{BASE_URL}/auth/register", json=data)
    if response.status_code == 201:
        return response.json()
    return None

def login_user(username, password):
    """用户登录"""
    data = {
        "username": username,
        "password": password
    }
    
    response = requests.post(f"{BASE_URL}/auth/login", json=data)
    if response.status_code == 200:
        return response.json()
    return None

def test_admin_registration():
    """测试管理员注册"""
    global admin_token, admin_user_id
    
    # 注册管理员用户
    result = register_user(ADMIN_USERNAME, ADMIN_PASSWORD)
    if result and result.get('success'):
        admin_user_id = result['data']['user']['id']
        print_test_result("管理员注册", True, f"用户ID: {admin_user_id}")
        
        # 登录获取token
        login_result = login_user(ADMIN_USERNAME, ADMIN_PASSWORD)
        if login_result and login_result.get('success'):
            admin_token = login_result['data']['token']
            print_test_result("管理员登录", True, f"Token: {admin_token[:20]}...")
            return True
    
    print_test_result("管理员注册/登录", False, "无法注册或登录管理员用户")
    return False

def test_user_registration():
    """测试普通用户注册"""
    global user_token, user_user_id
    
    # 注册普通用户
    result = register_user(USER_USERNAME, USER_PASSWORD)
    if result and result.get('success'):
        user_user_id = result['data']['user']['id']
        print_test_result("普通用户注册", True, f"用户ID: {user_user_id}")
        
        # 登录获取token
        login_result = login_user(USER_USERNAME, USER_PASSWORD)
        if login_result and login_result.get('success'):
            user_token = login_result['data']['token']
            print_test_result("普通用户登录", True, f"Token: {user_token[:20]}...")
            return True
    
    print_test_result("普通用户注册/登录", False, "无法注册或登录普通用户")
    return False

def test_admin_add_book():
    """测试管理员添加图书"""
    if not admin_token:
        print_test_result("管理员添加图书", False, "管理员未登录")
        return False
    
    headers = {"Authorization": f"Bearer {admin_token}"}
    book_data = {
        "isbn": "9781234567890",
        "title": "测试图书",
        "author": "测试作者",
        "total_copies": 10,
        "available_copies": 10
    }
    
    response = requests.post(f"{BASE_URL}/books", json=book_data, headers=headers)
    success = response.status_code == 201
    
    print_test_result("管理员添加图书", success, 
                     f"状态码: {response.status_code}, 响应: {response.text}")
    return success

def test_user_add_book():
    """测试普通用户添加图书（应该失败）"""
    if not user_token:
        print_test_result("普通用户添加图书", False, "普通用户未登录")
        return False
    
    headers = {"Authorization": f"Bearer {user_token}"}
    book_data = {
        "isbn": "9780987654321",
        "title": "普通用户测试图书",
        "author": "普通用户",
        "total_copies": 5,
        "available_copies": 5
    }
    
    response = requests.post(f"{BASE_URL}/books", json=book_data, headers=headers)
    success = response.status_code == 401
    
    print_test_result("普通用户添加图书权限限制", success,
                     f"状态码: {response.status_code}, 响应: {response.text}")
    return success

def test_user_borrow_book():
    """测试普通用户借阅图书"""
    if not user_token:
        print_test_result("普通用户借阅图书", False, "普通用户未登录")
        return False
    
    headers = {"Authorization": f"Bearer {user_token}"}
    
    # 先获取图书列表
    response = requests.get(f"{BASE_URL}/books", headers=headers)
    if response.status_code != 200:
        print_test_result("普通用户借阅图书", False, "无法获取图书列表")
        return False
    
    books = response.json().get('data', {}).get('data', [])
    if not books:
        print_test_result("普通用户借阅图书", False, "没有可借阅的图书")
        return False
    
    book_id = books[0]['id']
    
    # 借阅图书
    response = requests.post(f"{BASE_URL}/borrow", json={"book_id": book_id}, headers=headers)
    success = response.status_code == 201
    
    print_test_result("普通用户借阅图书", success,
                     f"状态码: {response.status_code}, 响应: {response.text}")
    return success

def test_user_view_own_history():
    """测试普通用户查看自己的借阅历史"""
    if not user_token:
        print_test_result("普通用户查看借阅历史", False, "普通用户未登录")
        return False
    
    headers = {"Authorization": f"Bearer {user_token}"}
    response = requests.get(f"{BASE_URL}/borrow/history", headers=headers)
    success = response.status_code == 200
    
    print_test_result("普通用户查看借阅历史", success,
                     f"状态码: {response.status_code}, 记录数: {len(response.json().get('data', []))}")
    return success

def test_admin_view_all_borrow_records():
    """测试管理员查看所有借阅记录"""
    if not admin_token:
        print_test_result("管理员查看所有借阅记录", False, "管理员未登录")
        return False
    
    headers = {"Authorization": f"Bearer {admin_token}"}
    response = requests.get(f"{BASE_URL}/admin/borrow-records", headers=headers)
    success = response.status_code == 200
    
    print_test_result("管理员查看所有借阅记录", success,
                     f"状态码: {response.status_code}, 记录数: {len(response.json().get('data', {}).get('data', []))}")
    return success

def test_admin_view_all_users():
    """测试管理员查看所有用户"""
    if not admin_token:
        print_test_result("管理员查看所有用户", False, "管理员未登录")
        return False
    
    headers = {"Authorization": f"Bearer {admin_token}"}
    response = requests.get(f"{BASE_URL}/admin/users", headers=headers)
    success = response.status_code == 200
    
    print_test_result("管理员查看所有用户", success,
                     f"状态码: {response.status_code}, 用户数: {len(response.json().get('data', {}).get('data', []))}")
    return success

def test_user_view_other_user_records():
    """测试普通用户查看其他用户记录（应该失败）"""
    if not user_token:
        print_test_result("普通用户查看其他用户记录", False, "普通用户未登录")
        return False
    
    headers = {"Authorization": f"Bearer {user_token}"}
    response = requests.get(f"{BASE_URL}/admin/users/{admin_user_id}/borrow-records", headers=headers)
    success = response.status_code == 401
    
    print_test_result("普通用户权限限制", success,
                     f"状态码: {response.status_code}, 响应: {response.text}")
    return success

def run_all_tests():
    """运行所有测试"""
    print("🧪 开始权限系统测试...")
    print("=" * 50)
    
    tests = [
        test_admin_registration,
        test_user_registration,
        test_admin_add_book,
        test_user_add_book,
        test_user_borrow_book,
        test_user_view_own_history,
        test_admin_view_all_borrow_records,
        test_admin_view_all_users,
        test_user_view_other_user_records,
    ]
    
    passed = 0
    total = len(tests)
    
    for test in tests:
        try:
            if test():
                passed += 1
        except Exception as e:
            print_test_result(test.__name__, False, str(e))
    
    print("=" * 50)
    print(f"📊 测试结果: {passed}/{total} 测试通过")
    
    if passed == total:
        print("🎉 所有测试通过！权限系统工作正常")
    else:
        print("⚠️  部分测试失败，请检查系统配置")
    
    return passed == total

if __name__ == "__main__":
    # 检查服务是否可用
    try:
        response = requests.get(f"{BASE_URL}/books", timeout=5)
        if response.status_code == 200:
            run_all_tests()
        else:
            print("❌ 后端服务未启动或不可用")
            sys.exit(1)
    except requests.exceptions.RequestException:
        print("❌ 无法连接到后端服务，请确保服务已启动")
        print("请运行: cd library-backend && cargo run")
        sys.exit(1)