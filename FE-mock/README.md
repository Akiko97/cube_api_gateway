# 前端数据说明文档

- 日 期：2024/09/21

版权声明

> 本文中出现的任何文字叙述、文档格式、插图、照片、方法、过程等内容，除另有特别注明，版权均属可信云科所有，受到有关产权及版权法保护。任何个人、机构未经 **待补充** 的书面授权许可，不得以任何方式复制或引用本文的任何片断。

版本变更记录

| 时间       | 版本 | 说明     | 修改人 |
| ---------- | ---- | -------- | ------ |
| 2024/09/21 | 1.0  | 初始版本 | 待补充 |
| 2024/09/23 | 1.1  | props优化 | 待补充 |

## 1. 文件目录

数据目录列表

```md
FE-mock
├── BASICE // 基础数据
│   ├── ui.json // 基础UI数据，场景数据里所有模块实例的 ui 配置继承自ui.json
│   ├── message.json // 顶部消息配置
│   ├── handles.json // 基础权限配置，场景数据里所有模块实例的权限可继承自handles.json
├── USAGES // 用例数据
│   ├── AliceBob.json
├── SCENES // 场景数据
│   ├── AliceBob.init // 原始场景
│   │   ├── index.json // 场景初始化编排配置和业务逻辑配置
│   │   ├── ui.json // 场景UI配置
│   │   ├── handles.json // 场景权限配置
│   ├── AliceBob.teardrop // 窃听场景
│   │   ├── index.json // 场景初始化编排配置和业务逻辑配置
│   │   ├── ui.json // 场景UI配置
│   │   ├── handles.json // 场景权限配置
│   ├── AliceBob.base64 // Base64编码场景
│   │   ├── index.json // 场景初始化编排配置和业务逻辑配置
│   │   ├── ui.json
│   │   ├── handles.json
│   ├── AliceBob.file_crypt // SM4加解密模块场景
│   │   ├── index.json // 场景初始化编排配置和业务逻辑配置
│   │   ├── ui.json
│   │   ├── handles.json
│   ├── AliceBob.force_attack // 穷举攻击场景
│   │   ├── index.json // 场景初始化编排配置和业务逻辑配置
│   │   ├── ui.json // 场景UI配置
│   │   ├── handles.json // 场景权限配置
```

## 2. 数据说明文档

因为数据量太大，不方便全量标注含义。所以每个描述标题会附带文件路径，方便查看。如 `/BASICE/ui.json`，

### 2.1 基础数据 `/BASICE`

具体每个场景的实例、模块的一些配置都会继承自基础数据

#### ui.json 基础UI数据 `/BASICE/ui.json`

1、模块的基础配置

```json
{
    "MODULE":{
        // 请求模块的基础配置
         "request": {
            "extend": "BASE_MODULE", // 继承自BASE_MODULE
            "type":"request",
            "options": {
                "background": "#2B70FF"
            },
            "data": {
                "title": "请求模块"
            }
        },
        // 其他模块配置
        // 消息文件打印模块的基础配置
        "msg_print": {
            // ...省略
        },
        // 文件分发模块的基础配置
        "file_dealer": {
            // ...省略
        },
        // SM4加密模块的基础配置
        "sm4_crypt": {
            // ...省略
        },
        // 穷举攻击模块的基础配置
        "force_attack": {
            // ...省略

            // 限制在实例的位置，相当于实例左上角的坐标
            "limits": {
                "Alice_crypthub": [
                    {
                        "x": "70",
                        "y": "115"
                      }
                ],
                "Bob_crypthub": [
                    {
                        "x": "70",
                        "y": "115"
                      }
                ]
            }
        }
    },
    // 模块基础配置，所有模块都继承自这个配置
    "BASE_MODULE": {
        "shape": "module", // 模块 自定义组件标识, antd x6 根据这个自定义渲染模块
        "width": 160,
        "height": 52,
        "ports": {
            // 端口配置
            "items": [
                {
                    "id": "port_t",
                    "group": "top"
                },
                {
                    "id": "port_b",
                    "group": "bottom"
                },
                {
                    "id": "port_l",
                    "group": "left"
                },
                {
                    "id": "port_r",
                    "group": "right"
                }
            ],
            // 端口组
            "groups": {
                "top": {
                   // ...省略
                },
                "bottom": {
                   // ...省略
                },
                "left": {
                   // ...省略
                },
                "right": {
                   // ...省略
                }
            }
        }
    }
}
```

2、实例的基础配置

```json
{
   "INSTANCE":{
        // 开始节点的配置
        "start":{
                "x": 70,
                "y": 183,
                "shape": "circle",
                "width": 1,
                "height": 1
        },
        // 发送者实例的配置
        "sender":{
            // 继承自BASE_INSTANCE
            "extend": "BASE_INSTANCE",
            "data": {
                "icon": "Instance1",
                "title": "发送者实例",
                "description": "Alice/sender"
            },
            
        }
        // 传输文件实例的配置
        "transfer":{
            // ...省略        
        }
        // 接收者实例的配置
        "receiver":{
            // ...省略
        },
        // 穷举攻击实例的配置
        "force_attack":{
            // ...省略
        },
        // 加密实例的配置
        "Alice_crypthub":{
            // ...省略
        },
        // 解密实例的配置
        "Bob_crypthub":{
            // ...省略
        }
   },
   // 实例的基础配置，所有实例都继承自这个配置
   "BASE_INSTANCE": {
        "shape": "instance", // 实例 自定义组件标识, antd x6 根据这个自定义渲染实例
        "width": 300,
        "height": 100
    },
}
```

3、边的基础配置

```json
{
    "EDGE": {
        // 主路由配置
        "main-routes": {
            "connector": { "name": "rounded" },
            "router": {
                "name": "manhattan",
                "args": {
                    "excludeNodes": ["root-start", "AliceBob.base64.transfer.point_copy"]
                }
            },
            "zIndex": 50,
            "attrs": {
                "path": {
                    "stroke": "#02B949",
                    "strokeWidth": 1,
                    "targetMarker": "block"
                }
            }
        }
    },
}
```

#### handles.json 基础权限设置 `/BASICE/handles.json`

TODO: 不同状态的权限如何配置暂未实现，待完善。现在是代码逻辑里写的。

权限枚举如下：

```typescript
const MENU_TYPES:MenuTypes = {
  ADD_MODULE: {
    key: 'add_module',
    label: '插入模块',
  },
  REPLACE_MODULE: {
    key: 'replace_module',
    label: '替换模块'
  },
  LINK_MODULE: {
    key: 'link_module',
    label: '连接模块'
  },
  DELETE_MODULE: {
    key: 'delete_module',
    label: '删除模块'
  },
  SHOW_MODULE: {
    key: 'show_module',
    label: '模块查看'
  },
  SETTING_MODULE: {
    key: 'setting_module',
    label: '模块配置'
  },
  RESET_MODULE: {
    key: 'reset_module',
    label: '重置模块'
  },
  SHOW_FILE: {
    key: 'show_file',
    label: '查看文件'
  },
  DELETE_ROUTE: {
    key: 'delete_route',
    label: '删除路径'
  },
  SET_KEYS: {
    key: 'set_keys',
    label: '设置密钥'
  },
  DECODE_TIMES: {
    key: 'decode_times',
    label: '解密时长'
  }
};
```

配置模块的基础权限，具体权限查看 `/SCENES/xxx.xxx/handles.json` 文件里的配置，此处为基础展示。

```json
{
    "request":[
      "show_file"
    ],
    "msg_print": [
      "show_file"
    ],
    "file_dealer":[
      "show_file"
    ],
    "sm4_crypt": [
      "show_file"
    ],
    "sm4_attack": [
      "show_file"
    ],
    "base64_crypt": [
      "show_file"
    ],
    "point_intercept": [
      "show_file"
    ],
    "point_copy": [
      "show_file"
    ]
}
```

#### message.json 基础消息数据 `/BASICE/message.json`

每个场景的消息配置

```json
{
  "AliceBob": {
    "message": {
      "0": [
        {
          "action": "init",
          "tip": "你创建了一个文件转发程序，Alice将通过你的文件转发程序向Bob发送一个文件，请点击【开始运行】按钮观察消息发送流程，并代替Bob点击「接收者实例」中的【文件分发模块】查看发送文件内容。"
        }
      ],
      "1": [
        {
          "action": "init",
          "tip": "文件传输过程中面临这威胁，现在请你扮演攻击者按照如下步骤操作，1、将【复制点】拖动至「传递实例」中；2、将插入的【复制点】与「传递实例」中的【文件分发模块】连接；3、运行程序，在「传递实例」中，查看【文件分发模块】中传递文件内容”。"
        }
      ],
      "2": [
        {
          "action": "init",
          "tip": "为了增强文件安全性Alice和Bob决定采用Base64编码模块，对文件进行编码与解码，1、请将【Base64编码模块】插入至「发送者加密实例」与「接收者解密实例」。2、在「发送者实例」中【文件分发模块】后与「接收者实例」中【文件分发模块】前路由上插入【拦截模块】 。3、将插入的【拦截模块】分别与「发送者加密实例」与「接收者解密实例」中的【Base64编码模块】用路由连接。4、观察「传递实例」中【文件分发模块】中传递的消息内容。"
        },
        {
          "action": "base64_pedding",
          "tip": "我们可以看到文件呈现乱码状态，不可直接查看。攻击者判断出文件使用【base64编码模块】进行了编码，1、请在传递实例中，【文件分发模块】前路由上插入【base64编码模块】。2、运行程序并查看「传递实例」中【文件分发者模块】中传递的消息内容。"
        },
        {
          "action": "base64_complete",
          "tip": "我们可以看到消息内容呈现明文显示。因为Base64编码属于通用编码规则，没有密钥的保护，攻击者拖入【Base64编码模块】后轻松的获取了文件内容。请点击下一场景进入下一场景。"
        }
      ],
      "3": [
        {
          "action": "init",
          "tip": "现在试一试SM4加解密模块的安全性吧。1、请在「发送者加密实例」与「接收者解密实例」中用【SM4加解密模块】替换【Base64编码模块】并设置密钥。2、代替攻击者删除「传递实例」中【Base64编码模块】运行程序，查看「传递实例」中【文件分发模块】传递的文件内容。"
        }
      ],
      "4": [
        {
          "action": "init",
          "tip": "文件在通过【SM4加解密模块】后安全性得到了提升，但攻击者可以通过【穷举攻击模块】破解密钥。1、将【SM4穷举攻击模块】插入「传递实例」中【D复制点」与【文件分发模块】之间的路由上。2、运行程序，穷举攻击完成后查看「传递实例」中【文件分发模块】内容"
        }
      ]
    }
  }
}
```

### 2.2 用例数据 `/SCENES`

#### AliceBob.json AliceBob为例  `/SCENES/AliceBob.json`

```json
{

    "name": "AliceBob", // 用例名称
    "full_name": "AliceBob", // 用例全名
    "type": "USAGE", // 用例类型
    "desc": "基本密码学原理实训，实训内容包括基本应用背景，窃听攻击、编码不是密码，基本SM4加密机制，穷举攻击原理演示五部分!", // 用例描述
    "scenes":[
        // init 场景
        {
            "name": "init", // 场景名称
            "full_name": "AliceBob.init", // 场景全名
            "type": "SCENE", // 场景类型
            "scene_seq": 1, // 排序
            "desc": "展示文件正常传输流程，让用户理解信息流动过程。" // 场景描述
        },
        // teardrop 场景
        {
            "name": "teardrop", // 场景名称
            "full_name": "AliceBob.teardrop", // 场景全名
            "type": "SCENE", // 场景类型
            "scene_seq": 2, // 排序
            "desc": "在「传递实例」中，插入复制点，复制所传输的文件，感受文件在传输过程中所面临的威胁。" // 场景描述
        },
        // base64 场景
        {
            "name": "base64", // 场景名称
            "full_name": "AliceBob.base64", // 场景全名
            "type": "SCENE", // 场景类型
            "scene_seq": 3, // 排序
            "desc": "对文件进行Base64编码，并进行解码，体现Base编码并不能保障文件的安全性" // 场景描述
        },
        {
            "name": "file_crypt", // 场景名称
            "full_name": "AliceBob.file_crypt", // 场景全名
            "type": "SCENE", // 场景类型
            "scene_seq": 4, // 排序
            "desc": "用【SM4加解密模块】替换掉【Base64编码模块】，并查看「传递实例」中【文件分发模块】传递传递的文件内容，体现【SM4加解密模块】的安全性。" // 场景描述
        },
        {
            "name": "force_attack",
            "full_name": "AliceBob.force_attack", // 场景全名
            "type": "SCENE", // 场景类型
            "scene_seq": 5, // 排序
            "desc": "攻击者通过穷举攻击的方式破解密钥，发送者与接收者通过增加密钥复杂性。" // 场景描述
        }
    ]
}
```

### 2.3 场景数据 `/SCENES`

重要字段解释，其他详见示例代码注释

- type: 节点类型
  - NODE 普通节点  
  - INSTANCE 实例节点
  - MODULE 模块节点
- full_name： 作为唯一标识， 在场景中所有节点，模块，边都使用全名作为唯一标识

**以 AliceBob.force_attack 场景数据 为例 `/SCENES/AliceBob.force_attack`**
> AliceBob.force_attack 数据比较全, 作为解释示例。其他场景类似

#### 1、 index.json 场景初始化编排配置和业务逻辑配置 `/SCENES/AliceBob.force_attack/index.json`

```json

{
    // 节点数据: 绘制图形需要所有的节点信息，并提前根据配置注册
    // 实例 > 模块
  "nodes": [
    {
      "full_name": "AliceBob.force_attack.start", // 开始节点
      "name": "start", // 开始节点
      "name_cn": "",
      "type": "NODE", // 节点类型
      "desc": ""
    },
    {
      "full_name": "AliceBob.force_attack.sender", // 发送者实例全名， 作为实例的唯一标识
      "name": "sender", // 发送者实例名称
      "name_cn": "发送者实例", // 发送者实例中文名称
      "type": "INSTANCE", // 实例类型
      "desc": "",
      "modules": [ // 实例保护的模块
        {
          "full_name": "AliceBob.force_attack.sender.request", // 请求模块
          "name": "request",
          "name_cn": "请求模块",
          "type": "MODULE",
          "desc": ""
        },
        {
          "full_name": "AliceBob.force_attack.sender.file_dealer", // 文件分发模块
          "name": "file_dealer",
          "name_cn": "文件分发模块",
          "type": "MODULE",
          "desc": ""
        },
        {
          "full_name": "AliceBob.force_attack.sender.msg_print",
          "name": "msg_print",
          "name_cn": "消息打印模块",
          "type": "MODULE",
          "desc": ""
        }
      ],
      "tangents": [ // 切面点信息，用于链接分支路由
        {
          "full_name": "AliceBob.force_attack.sender.point_intercept",
          "name": "point_intercept", // 拦截点 A
          "name_cn": "拦截点",
          "type": "MODULE",
          "desc": ""
        }
      ]
    },
    {
      "full_name": "AliceBob.force_attack.transfer",
      "name": "transfer",
      "name_cn": "传递实例",
      "type": "INSTANCE",
      "desc": "",
      "modules": [
        {
            // 空的作为占空，后面预留添加模块位置
        },
        {
          "full_name": "AliceBob.force_attack.transfer.file_dealer",
          "name": "file_dealer",
          "name_cn": "文件分发模块",
          "type": "MODULE",
          "desc": ""
        }
      ],
      "tangents": [ // 切面点信息，用于链接分支路由
        {
          "full_name": "AliceBob.force_attack.transfer.point_copy",
          "name": "point_copy", // 复制点 D
          "name_cn": "复制点",
          "type": "MODULE",
          "desc": ""
        }
      ]
    },
    {
      "full_name": "AliceBob.force_attack.receiver",
      "name": "receiver",
      "name_cn": "接收者实例",
      "type": "INSTANCE",
      "desc": "",
      "modules": [
        {
          "full_name": "AliceBob.force_attack.receiver.file_dealer",
          "name": "file_dealer",
          "name_cn": "文件分发模块",
          "type": "MODULE",
          "desc": ""
        }
      ],
      "tangents": [ // 切面点信息，用于链接分支路由
        {
          "full_name": "AliceBob.force_attack.receiver.point_intercept",
          "name": "point_intercept",
          "name_cn": "拦截点",
          "type": "MODULE",
          "desc": ""
        }
      ]
    },
    {
      "full_name": "AliceBob.force_attack.Alice_crypthub",
      "name": "Alice_crypthub",
      "name_cn": "发送者加密实例",
      "type": "INSTANCE",
      "desc": "",
      "modules": [
        {
          "full_name": "AliceBob.force_attack.Alice_crypthub.sm4_crypt",
          "name": "sm4_crypt",
          "name_cn": "",
          "type": "MODULE",
          "desc": ""
        }
      ]
    },
    {
      "full_name": "AliceBob.force_attack.Bob_crypthub",
      "name": "Bob_crypthub",
      "name_cn": "接收者解密实例",
      "type": "INSTANCE",
      "desc": "",
      "modules": [
        {
          "full_name": "AliceBob.force_attack.Bob_crypthub.sm4_crypt",
          "name": "sm4_crypt",
          "name_cn": "",
          "type": "MODULE",
          "desc": ""
        }
      ]
    }
  ],
  "edges": [
    {
     // 主流程路由配置
      "full_name": "AliceBob.force_attack.route<main>",
      "routes": [
        {
          "outRoute": [
            "AliceBob.force_attack.start", // 开始节点
            "AliceBob.force_attack.sender" // 发送者实例  route-id： AliceBob.force_attack.start-AliceBob.force_attack.sender
          ],
          // 前端维护： props移除替换成 route_ui_props
          // "props": { // 路由自定义属性
          //   "into": "leftTop_in", // 进入实例的端口
          //   "out": "rightTop_out", // 出去
          //   "outVertices": [ // 出实例路由的途径点坐标
          //     {
          //       "x": 425,
          //       "y": 125
          //     }
          //   ]
          // },
          "route_ui_props": "AliceBob.force_attack.start$_$AliceBob.force_attack.sender", // 该组路由唯一标识，映射ui里的样式配置文件
         
          "inRoute": [  // 进入实例的内部路径：外部路由最后一个实例内部的路由，此处为 发送者实例 AliceBob.force_attack.sender
            "AliceBob.force_attack.sender.request", // 进入实例的第一个路由 AliceBob.force_attack.sender-AliceBob.force_attack.sender.request
            "AliceBob.force_attack.sender.file_dealer", // 进入实例的第二个路由
            "AliceBob.force_attack.sender.point_intercept" // 进入实例的第三个路由，出去的路由，同时作为拦截点，用于连接其他流程路由
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.sender", // 发送者实例
            "AliceBob.force_attack.transfer" // 传递实例
          ],
          "route_ui_props":"AliceBob.force_attack.sender$_$AliceBob.force_attack.transfer",
          "inRoute": [
            "AliceBob.force_attack.transfer.point_copy"
          ]
        },
        {
          "outRoute": [ // 外部路由
            "AliceBob.force_attack.transfer",
            "AliceBob.force_attack.receiver"
          ],
          "route_ui_props":"AliceBob.force_attack.transfer$_$AliceBob.force_attack.receiver",
          "inRoute": [ // 进入实例的内部路由
              "AliceBob.force_attack.receiver.point_intercept",
            "AliceBob.force_attack.receiver.file_dealer"
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.receiver",
            "AliceBob.force_attack.transfer"
          ],
          "route_ui_props":"AliceBob.force_attack.receiver$_$AliceBob.force_attack.transfer",
          "inRoute": []
        },
        {
          "outRoute": [
            "AliceBob.force_attack.transfer",
            "AliceBob.force_attack.sender"
          ],
          "route_ui_props":"AliceBob.force_attack.transfer$_$AliceBob.force_attack.sender",
          "inRoute": [
            "AliceBob.force_attack.sender.msg_print"
          ]
        }
      ]
    },
    {
     // 分支流程路由配置：此处为复制路由配置
      "full_name": "AliceBob.force_attack.route<D>.transfer",
      "routes": [
        {
          "outRoute": [],
          "inRoute": [
            "AliceBob.force_attack.transfer.point_copy",
            "AliceBob.force_attack.transfer.file_dealer"
          ]
        }
      ]
    },
    {
     // 分支流程路由配置：此处为拦截点路由配置
      "full_name": "AliceBob.force_attack.route<A>.sender",
      "routes": [
        {
          "outRoute": [
            "",
            "AliceBob.force_attack.sender"
          ],
          "route_ui_props":"route<A>.sender$_$_$_$AliceBob.force_attack.sender",
          "inRoute": [
            "AliceBob.force_attack.sender.point_intercept"
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.sender",
            "AliceBob.force_attack.Alice_crypthub"
          ],
          "route_ui_props":"route<A>.sender$_$AliceBob.force_attack.sender$_$AliceBob.force_attack.Alice_crypthub",
          "inRoute": [
            "AliceBob.force_attack.Alice_crypthub.sm4_crypt"
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.Alice_crypthub",
            "AliceBob.force_attack.sender"
          ],
          "route_ui_props":"route<A>.sender$_$AliceBob.force_attack.Alice_crypthub$_$AliceBob.force_attack.sender",
          "inRoute": [
            "AliceBob.force_attack.sender.point_intercept"
          ]
        }
      ]
    },
    {
      //  分支流程路由配置：此处为拦截点路由配置  
      "full_name": "AliceBob.force_attack.route<A>.receiver",
      "routes": [
        {
          "outRoute": [
            "",
            "AliceBob.force_attack.receiver"
          ],
          "route_ui_props":"route<A>.receiver$_$_$_$AliceBob.force_attack.receiver",
          "inRoute": [
            "AliceBob.force_attack.receiver.point_intercept"
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.receiver",
            "AliceBob.force_attack.Bob_crypthub"
          ],
          "route_ui_props":"route<A>.receiver$_$AliceBob.force_attack.receiver$_$AliceBob.force_attack.Bob_crypthub",
          "inRoute": [
            "AliceBob.force_attack.Bob_crypthub.sm4_crypt"
          ]
        },
        {
          "outRoute": [
            "AliceBob.force_attack.Bob_crypthub",
            "AliceBob.force_attack.receiver"
          ],
          "route_ui_props":"route<A>.receiver$_$AliceBob.force_attack.Bob_crypthub$_$AliceBob.force_attack.receiver",
          "inRoute": [
            "AliceBob.force_attack.receiver.point_intercept"
          ]
        }
      ]
    }
  ],
  "tip": ""
}
```

#### 2、handles.json 权限配置 `/SCENES/AliceBob.force_attack/handles.json`

具体配置示例

```json
{
    // 该模块的权限配置
    "AliceBob.force_attack.receiver.file_dealer": [
        "show_file",
        "link_module",
        "replace_module",
        "delete_module"
    ],
    "AliceBob.force_attack.sender.file_dealer": [
        "show_file",
        "link_module",
        "replace_module",
        "delete_module"
    ],
    "AliceBob.force_attack.transfer.file_dealer": [
        "show_file",
        "link_module",
        "replace_module",
        "delete_module"
    ],
    "AliceBob.force_attack.Alice_crypthub.sm4_crypt": [
        "link_module",
        "replace_module",
        "set_keys",
        "delete_module"
    ],
    "AliceBob.force_attack.Bob_crypthub.sm4_crypt": [
        "link_module",
        "replace_module",
        "set_keys",
        "delete_module"
    ],
    "AliceBob.file_crypt.Bob_crypthub.sm4_crypt": [
        "link_module",
        "replace_module",
        "set_keys",
        "delete_module"
    ]
}
```

#### 3、ui 配置 ui.json `/SCENES/AliceBob.force_attack/ui.json`

ui 配置主要用于在模块上展示的配置项，具体配置如下：
主要将 index.json 中的配置的节点（full_name), 关联对应的组件和样式配置，以及位置信息。

```json
{
    "AliceBob.force_attack.start": { // 对应的 index.json 中的 full_name 为 AliceBob.force_attack.start
        "extend": "INSTANCE.start" // 节点样式属性继承自 BASICE/ui.json 中的 INSTANCE.start
    },
    "AliceBob.force_attack.sender": { // 对应的 index.json 中的 full_name 为 AliceBob.force_attack.sender
        "extend": "INSTANCE.sender", // 节点样式属性继承自 BASICE/ui.json 中的 INSTANCE.sender
        "x": 172, // 节点位置
        "y": 43, // 节点位置
        "top": 80, // 距离顶部距离
        // 自定义端口配置，默认会继承 INSTANCE.Bob_crypthub 的端口配置
        "customPorts": [ 
            {
                "position": "rightTop",
                "shape": "PortLG"
            },
            {
                "position": "rightBottom",
                "shape": "PortLY"
            }
        ]
    },
    //  ... 省略
    
    "AliceBob.force_attack.transfer.file_dealer":{
        "extend": "MODULE.file_dealer",
        "isCrypy": true, // 是否加密
        "needSm4": true // 是否需要 sm4 加密
    },
    "AliceBob.force_attack.sender.msg_print": {
         "extend": "MODULE.msg_print"
    },
   // ... 其他省略

    "AliceBob.force_attack.transfer.point_copy":{
        "extend": "MODULE.point_copy",
        "x": 660, // 位置信息
        "y": 108
    },
    // ... 其他省略

    "AliceBob.force_attack.route<main>":{  // 主流程路由样式配置
        "stroke": "#02B949" // 路由颜色
    },
    "AliceBob.force_attack.route<D>.transfer":{ // 复制路由样式配置
        "stroke": "#E844FF"
    },
    "AliceBob.force_attack.route<A>.sender":{ // 拦截点路由样式配置
        "stroke": "#FFB701" // 路由颜色
    },
     "AliceBob.force_attack.route<A>.receiver":{ // 拦截点路由样式配置
        "stroke": "#FFB701" // 路由颜色
    }

    // 路由相关的
}
```

