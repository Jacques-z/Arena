万物轮回

**游戏刚开始开发**

# 目标

- 随机性
- 竞技性
  - 劣势的翻盘空间

# 规则

## 目标

## 生命周期

根据材料与咒语来诞生
自然成长
死亡后分解为材料

# 架构与发布

项目包括前端（用godot编写）和后端，这使得大家可以自由地修改自己的客户端（比如做CLI）或者部署自己的游戏服务器。

## 玩家

- 联机客户端：较轻的客户端，为了避免不必要的更新、减少更新体积以及自定义游戏的兼容性等原因，采取内核与数据库分离的形式
- 单机客户端：在联机客户端的基础上，同时打包进了服务器与AI以便本地运行

## 服务器

只进行必要的数据存储与运算，限制进出数据量
