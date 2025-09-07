# SharePoker 服务端

一个用Rust编写的简单扑克牌局管理服务端，支持上传用户手牌和查询牌局信息。

## 功能特性

- 上传用户名和手牌到指定牌局
- 通过牌局代码查询所有玩家信息
- **设备唯一标识支持** - 解决OCR识别错误问题
- 基于内存的数据存储（使用DashMap）
- 无需用户认证，简单易用
- 支持Web界面查看牌局

## 技术栈

- **Rust** - 主要编程语言
- **Axum** - Web框架
- **Tokio** - 异步运行时
- **DashMap** - 并发安全的HashMap

## 快速开始

### 方法1：使用Docker部署已编译的exe文件（推荐）

确保您已安装Docker，然后运行：

```bash
# 1. 先编译项目
cargo build --release

# 2. 启动Docker容器
docker compose up -d

# 或者使用启动脚本
chmod +x start-docker.sh
./start-docker.sh
```

**管理命令：**
```bash
# 查看服务状态
docker compose ps

# 查看日志
docker compose logs -f

# 停止服务
docker compose down

# 重启服务
docker compose restart
```

服务器将在 `http://localhost:3001` 启动。

### 访问Web界面

- **主界面**: http://localhost:3001/ (查看牌局)
- **演示界面**: http://localhost:3001/demo.html (完整功能演示)

### 方法2：本地运行

#### 安装依赖

确保你已经安装了Rust。然后运行：

```bash
cargo build --release
```

#### 启动服务器

```bash
# Windows
target\release\sharepoker.exe

# 或使用Cargo
cargo run --release
```

服务器将在 `http://localhost:3001` 启动。

## API接口

### 上传玩家手牌
```
POST /api/game/upload
Content-Type: application/json

{
    "game_code": "123456",
    "username": "player1",
    "device_id": "device_001",  // 设备唯一标识
    "image": "data:image/png;base64,iVBORw0KGgo..." // Base64编码的图片
}
```

### 获取牌局信息
```
GET /api/game/{code}
```

返回指定牌局代码的所有玩家信息，包括用户名、设备ID和手牌。

## 设备唯一标识功能

### 功能说明
- 每个设备都有唯一的标识符（device_id）
- 当相同设备ID上传新的手牌时，会覆盖该设备之前的玩家名称
- 解决OCR识别玩家名称错误的问题
- 支持同一设备多次上传，自动更新玩家信息

### 使用场景
1. **首次上传**：设备上传手牌，创建新的玩家记录
2. **OCR错误修正**：当OCR识别错误时，使用相同设备ID重新上传，自动覆盖错误的玩家名称
3. **手牌更新**：同一设备可以多次上传手牌，更新手牌图片

### 设备ID生成
- 可以使用任意字符串作为设备ID
- 建议使用有意义的标识符，如：`device_001`、`phone_123`、`tablet_abc`等
- Demo页面提供自动生成功能

## 数据结构

### 牌局 (Game)
- `code`: 牌局代码
- `players`: 玩家列表
- `created_at`: 创建时间

### 玩家 (Player)
- `username`: 用户名
- `device_id`: 设备唯一标识
- `hand_image`: 手牌图片（Base64或URL）
- `uploaded_at`: 上传时间

## 使用示例

1. **上传玩家手牌**：
```bash
curl -X POST http://localhost:3001/api/game/upload \
  -H "Content-Type: application/json" \
  -d '{
    "game_code": "123456",
    "username": "player1",
    "device_id": "device_001",
    "image": "data:image/png;base64,iVBORw0KGgo..."
  }'
```

2. **修正OCR识别错误**：
```bash
# 使用相同设备ID，但不同的用户名
curl -X POST http://localhost:3001/api/game/upload \
  -H "Content-Type: application/json" \
  -d '{
    "game_code": "123456",
    "username": "corrected_player_name",
    "device_id": "device_001",
    "image": "data:image/png;base64,iVBORw0KGgo..."
  }'
```

3. **查看牌局信息**：
```bash
curl -X GET http://localhost:3001/api/game/123456
```

## 文件说明

### 部署文件
- `Dockerfile` - Windows容器构建文件
- `docker-compose.yml` - Docker Compose配置文件

### 应用文件
- `target/release/sharepoker.exe` - Windows 64位可执行文件
- `index.html` - 主界面（查看牌局）
- `demo.html` - 完整功能演示界面（支持设备ID）
- `api_test.http` - REST Client测试文件（包含设备ID测试用例）
- `README.md` - 项目说明文档

## 注意事项

- 这是一个演示项目，数据存储在内存中，服务器重启后数据会丢失
- 无需用户认证，任何人都可以上传和查看牌局信息
- 牌局代码可以是任意字符串，如果牌局不存在会自动创建
- **设备唯一标识功能**：相同设备ID会覆盖之前的玩家名称和手牌
- 设备ID用于识别同一设备，解决OCR识别错误问题

### Docker部署注意事项

- 确保Docker已安装
- 首次构建可能需要较长时间，因为需要下载Ubuntu镜像和安装Wine
- 使用Wine在Linux容器中运行Windows exe文件
- 容器会自动重启（除非手动停止）
- 健康检查会定期验证服务状态
- 服务日志可通过 `docker compose logs -f` 查看

## 开发计划

- [x] 添加设备唯一标识支持
- [ ] 添加数据库持久化
- [ ] 添加WebSocket支持实时通信
- [ ] 添加牌局管理功能（删除、清空等）
- [ ] 添加输入验证和错误处理
- [ ] 添加单元测试 