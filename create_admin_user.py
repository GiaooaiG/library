#!/usr/bin/env python3
"""
创建管理员用户的脚本
由于系统默认注册的用户都是普通用户，此脚本直接在数据库中创建管理员用户
"""

import mysql.connector
import sys
import os
from argon2 import PasswordHasher
from argon2.exceptions import VerifyMismatchError

# 数据库配置
DB_CONFIG = {
    'host': 'localhost',
    'user': 'root',
    'password': 'password',  # 请根据您的MySQL配置修改
    'database': 'library',
    'port': 3306
}

def create_admin_user(username, password):
    """创建管理员用户"""
    try:
        # 连接数据库
        conn = mysql.connector.connect(**DB_CONFIG)
        cursor = conn.cursor()
        
        # 检查用户是否已存在
        cursor.execute("SELECT id FROM users WHERE username = %s", (username,))
        if cursor.fetchone():
            print(f"用户 {username} 已存在")
            return False
        
        # 密码哈希
        ph = PasswordHasher()
        password_hash = ph.hash(password)
        
        # 插入管理员用户
        cursor.execute("""
            INSERT INTO users (username, password_hash, role, phone) 
            VALUES (%s, %s, 'admin', '12345678901')
        """, (username, password_hash))
        
        conn.commit()
        user_id = cursor.lastrowid
        
        print(f"✅ 管理员用户创建成功！")
        print(f"用户名: {username}")
        print(f"密码: {password}")
        print(f"用户ID: {user_id}")
        print(f"角色: admin")
        
        return True
        
    except mysql.connector.Error as e:
        print(f"❌ 数据库错误: {e}")
        return False
    except Exception as e:
        print(f"❌ 错误: {e}")
        return False
    finally:
        if 'cursor' in locals():
            cursor.close()
        if 'conn' in locals():
            conn.close()

def test_admin_login(username, password):
    """测试管理员登录"""
    import requests
    
    try:
        response = requests.post("http://localhost:8080/api/v1/auth/login", json={
            "username": username,
            "password": password
        })
        
        if response.status_code == 200:
            data = response.json()
            if data.get('success'):
                user = data['data']['user']
                token = data['data']['token']
                print(f"✅ 管理员登录测试成功！")
                print(f"用户角色: {user.get('role')}")
                print(f"Token: {token[:20]}...")
                return True
        
        print(f"❌ 登录测试失败: {response.text}")
        return False
        
    except Exception as e:
        print(f"❌ 登录测试错误: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("使用方法: python3 create_admin_user.py <用户名> <密码>")
        print("示例: python3 create_admin_user.py admin admin123")
        sys.exit(1)
    
    username = sys.argv[1]
    password = sys.argv[2]
    
    print("🔧 创建管理员用户...")
    
    if create_admin_user(username, password):
        print("\n🧪 测试管理员登录...")
        test_admin_login(username, password)
        
        print("\n📋 管理员账号信息:")
        print("=" * 40)
        print(f"用户名: {username}")
        print(f"密码: {password}")
        print(f"登录地址: http://localhost:5173/login")
        print("=" * 40)
    else:
        print("❌ 创建管理员用户失败")
        sys.exit(1)