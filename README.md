万物轮回

**游戏刚开始开发**

# 目标

- 随机性
- 竞技性
  - 劣势的翻盘空间

# 规则

有若干地点，分布若干资源。
打出生物卡牌可以在非敌方控制区域消耗资源预备召唤
打出建筑卡可以获得对应的分数，建立控制等，但是也会积累仇恨值（也许可以给对手造）

## 目标

抢先获得7分者胜

## 控制

如果一个地点只有某一玩家的建筑且分数不少于2，那么此地点被该玩家控制

注意，如果一个地点同时有双方的建筑，且分差不小于2，则会引发动乱

## 生命周期

根据材料与咒语来诞生
自然成长，消耗与产出资源或建筑图纸（共享）
死亡后分解为材料

# 架构与发布

项目包括前端（用godot编写）和后端，这使得大家可以自由地修改自己的客户端（比如做CLI）或者部署自己的游戏服务器。

## 玩家

- 联机客户端：较轻的客户端，为了避免不必要的更新、减少更新体积以及自定义游戏的兼容性等原因，采取内核与数据库分离的形式
- 单机客户端：在联机客户端的基础上，同时打包进了服务器与AI以便本地运行

## 服务器

只进行必要的数据存储与运算，限制进出数据量
