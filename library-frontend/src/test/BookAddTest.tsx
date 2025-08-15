import { useState } from 'react';
import { bookService } from '../services/api';
import type { NewBook } from '../services/api';

const BookAddTest: React.FC = () => {
  const [testResults, setTestResults] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  const addTestResult = (message: string) => {
    setTestResults(prev => [...prev, `${new Date().toLocaleTimeString()}: ${message}`]);
  };

  const runTests = async () => {
    setTestResults([]);
    setLoading(true);
    
    try {
      // 测试1: 测试API连接
      addTestResult('开始测试API连接...');
      const healthCheck = await fetch('http://localhost:8080/api/v1/books');
      if (healthCheck.ok) {
        addTestResult('✅ API连接成功');
      } else {
        addTestResult('❌ API连接失败');
        return;
      }

      // 测试2: 测试获取图书列表
      addTestResult('测试获取图书列表...');
      const booksResponse = await bookService.getBooks();
      if (booksResponse.success) {
        addTestResult(`✅ 获取图书列表成功，共${booksResponse.data?.length || 0}本图书`);
      } else {
        addTestResult('❌ 获取图书列表失败');
      }

      // 测试3: 测试添加有效图书
      addTestResult('测试添加有效图书...');
      const validBook: NewBook = {
        isbn: '9787123456789',
        title: '测试图书',
        author: '测试作者',
        category: '测试分类',
        publisher: '测试出版社',
        total_copies: 5,
        available_copies: 5
      };

      const addResponse = await bookService.createBook(validBook);
      if (addResponse.success && addResponse.data) {
        addTestResult(`✅ 添加图书成功，图书ID: ${addResponse.data.id}`);
      } else {
        addTestResult(`❌ 添加图书失败: ${addResponse.message}`);
      }

      // 测试4: 测试重复ISBN
      addTestResult('测试重复ISBN...');
      const duplicateResponse = await bookService.createBook(validBook);
      if (!duplicateResponse.success && duplicateResponse.message.includes('已存在')) {
        addTestResult('✅ 重复ISBN检测成功');
      } else {
        addTestResult('❌ 重复ISBN检测失败');
      }

      // 测试5: 测试验证错误
      addTestResult('测试验证错误...');
      const invalidBook: NewBook = {
        isbn: '123', // 无效的ISBN
        title: '', // 空标题
        author: '', // 空作者
        total_copies: 0, // 无效的数量
        available_copies: -1 // 无效的可用数量
      };

      try {
        await bookService.createBook(invalidBook);
        addTestResult('❌ 验证错误检测失败');
      } catch (error: any) {
        if (error.response?.status === 400) {
          addTestResult('✅ 验证错误检测成功');
        } else {
          addTestResult('❌ 验证错误检测失败');
        }
      }

      addTestResult('🎉 所有测试完成！');

    } catch (error: any) {
      addTestResult(`❌ 测试过程中发生错误: ${error.message}`);
    } finally {
      setLoading(false);
    }
  };

  const clearTestData = async () => {
    // 注意：这里只是演示，实际项目中应该通过API删除测试数据
    addTestResult('测试数据清理功能需要后端支持');
  };

  return (
    <div className="test-container">
      <h2>图书添加功能集成测试</h2>
      
      <div className="test-actions">
        <button 
          onClick={runTests} 
          disabled={loading}
          className="btn btn-primary"
        >
          {loading ? '测试中...' : '运行测试'}
        </button>
        
        <button 
          onClick={clearTestData}
          className="btn btn-secondary"
        >
          清理测试数据
        </button>
        
        <button 
          onClick={() => setTestResults([])}
          className="btn btn-secondary"
        >
          清空结果
        </button>
      </div>

      <div className="test-results">
        <h3>测试结果</h3>
        {testResults.length === 0 ? (
          <p>点击"运行测试"开始测试</p>
        ) : (
          <ul>
            {testResults.map((result, index) => (
              <li key={index} className={result.includes('❌') ? 'error' : result.includes('✅') ? 'success' : ''}>
                {result}
              </li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default BookAddTest;