# ibus 协议分析

TODO


## D-Bus 捕捉

使用工具: Bustle

<https://flathub.org/zh-Hans/apps/org.freedesktop.Bustle>

安装命令:

```sh
flatpak install flathub org.freedesktop.Bustle
```

捕捉命令:

```
> dbus-monitor --pcap --address unix:path=/home/s2/.cache/ibus/dbus-PgQYLUex > 1-ibus.pcap
```

进行一些输入操作, 然后用 Bustle 打开捕捉的文件.


## 切换输入法

根据 Bustle 看到的 D-Bus 捕捉数据, 输入法初始化开始时的重要事件是:

```
Type: Method call
Sender: :1.3
Destination: org.freedesktop.IBus
Path: /org/freedesktop/IBus
Member: org.freedesktop.IBus.SetGlobalEngine
Arguments: ('libpinyin',)
```

然后:

```
Type: Signal
Sender: org.freedesktop.IBus
Path: /org/freedesktop/IBus
Member: org.freedesktop.IBus.GlobalEngineChanged
Arguments: ('libpinyin',)
```

最后:

```
Type: Signal
Sender: :1.13
Path: /org/freedesktop/IBus/Engine/1
Member: org.freedesktop.IBus.Engine.RegisterProperties

Arguments:

(<(
  'IBusPropList', @a{sv} {},
  [

    <(
      'IBusProperty', @a{sv} {},
      'InputMode', uint32 0,
      <(
        'IBusText', @a{sv} {},
        '中文',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      '/usr/share/ibus-libpinyin/icons/chinese.svg',
      <(
        'IBusText', @a{sv} {},
        '切换到英文模式',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      true, true, uint32 0,
      <(
        'IBusPropList', @a{sv} {},
        @av []
      )>,
      <(
        'IBusText', @a{sv} {},
        '中',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>
    )>,

    <(
      'IBusProperty', @a{sv} {},
      'mode.full', uint32 0,
      <(
        'IBusText', @a{sv} {},
        '半角字母',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      '/usr/share/ibus-libpinyin/icons/half.svg',
      <(
        'IBusText', @a{sv} {},
        '切换到全角字母模式',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      true, true, uint32 0,
      <(
        'IBusPropList', @a{sv} {},
        @av []
      )>,
      <(
        'IBusText', @a{sv} {},
        '',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>
    )>,

    <(
      'IBusProperty', @a{sv} {},
      'mode.full_punct', uint32 0,
      <(
        'IBusText', @a{sv} {},
        '全角标点',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      '/usr/share/ibus-libpinyin/icons/full-punct.svg',
      <(
        'IBusText', @a{sv} {},
        '切换到半角标点模式',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      true, true, uint32 0,
      <(
        'IBusPropList', @a{sv} {},
        @av []
      )>,
      <(
        'IBusText', @a{sv} {},
        '',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>
    )>,

    <(
      'IBusProperty', @a{sv} {},
      'mode.simp', uint32 0,
      <(
        'IBusText', @a{sv} {},
        '简体中文',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      '/usr/share/ibus-libpinyin/icons/simp-chinese.svg',
      <(
        'IBusText', @a{sv} {},
        '切换到繁体中文模式',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      true, true, uint32 0,
      <(
        'IBusPropList', @a{sv} {},
        @av []
      )>,
      <(
        'IBusText', @a{sv} {},
        '',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>
    )>,

    <(
      'IBusProperty', @a{sv} {},
      'setup', uint32 0,
      <(
        'IBusText', @a{sv} {},
        '首选项',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      'ibus-setup',
      <(
        'IBusText', @a{sv} {},
        '首选项',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>,
      true, true, uint32 0,
      <(
        'IBusPropList', @a{sv} {},
        @av []
      )>,
      <(
        'IBusText', @a{sv} {},
        '',
        <(
          'IBusAttrList', @a{sv} {},
          @av []
        )>
      )>
    )>

  ]
)>, )
```

### SetGlobalEngine

结合源代码分析 `SetGlobalEngine`.

+ (1) 源文件: `ibus/bus/ibusimpl.c`
  函数: `bus_ibus_impl_service_method_call()`

  代码:

  ```c
  { "SetGlobalEngine",       _ibus_set_global_engine },
  ```

  也就是 D-Bus 的 `SetGlobalEngine`
  实际调用函数 `_ibus_set_global_engine` 来处理.

+ (2) 源文件: `ibus/bus/ibusimpl.c`
  函数: `_ibus_set_global_engine()`

  调用函数: `bus_input_context_set_engine_by_desc()`

+ (3) 源文件: `ibus/bus/inputcontext.c`
  函数: `bus_input_context_set_engine_by_desc()`

  调用函数 `bus_engine_proxy_new()`

+ (4) 源文件: `ibus/bus/engineproxy.c`
  函数: `bus_engine_proxy_new()`

  调用函数: `bus_component_start()`

  TODO

  调用函数: `bus_factory_proxy_create_engine()`

+ (5) 源文件: `ibus/bus/component.c`
  函数: `bus_component_start()`

  这个函数启动组件对应的进程 (结束).

+ (6) 源文件: `ibus/bus/factoryproxy.c`
  函数: `bus_factory_proxy_create_engine()`

  调用 D-Bus `CreateEngine`

  TODO

TODO 初始化分析不完整


## 生成完整代码

```sh
./autogen.sh --disable-emoji-dict --disable-unicode-dict
```

TODO


## 从 ibus-libpinyin 源代码分析初始化过程

+ (1) 源文件: `ibus-libpinyin/src/PYMain.cc` (入口)
  函数: `main()` (入口)

  调用 `start_component()`

+ (2) 源文件同上
  函数: `start_component()`

  - (2.1) 调用 `ibus_init()`

    调用 `ibus_bus_new()`

    调用 `ibus_bus_is_connected()` (忽略)

  - (2.2) 调用 `ibus_bus_get_config()`

  - (2.3) 调用 `ibus_factory_new(ibus_bus_get_connection())`

  - (2.4) 调用 `ibus_factory_add_engine()`

  - (2.5) 调用 `ibus_bus_request_name()`

  - (2.6) 调用 `ibus_main()`

+ (3) 源文件: `ibus/src/ibusshare.c`
  函数: `ibus_init()`

  调用 `_ibus_register_resource()`

  TODO 错误: 无法找到函数 `_ibus_register_resource`

+ (4) 源文件: `ibus/src/ibusbus.c`
  函数: `ibus_bus_new()`

  - (4.1) 调用 `g_object_new()`, 间接调用 `ibus_bus_constructor()` (同文件):

  - (4.2) 调用 `ibus_bus_connect()` (同文件):

    调用 `ibus_get_address()` (获取 D-Bus 地址)

    调用 `g_dbus_connection_new_for_address_sync()` (连接 D-Bus)

  - (4.3) 调用 `ibus_bus_connect_completed()` (同文件):

  - (4.4) 调用 `ibus_bus_hello()` (同文件):

    调用 `g_dbus_connection_get_unique_name()`

+ (5) 源文件: `ibus/src/ibusbus.c`
  函数: `ibus_bus_get_config()`

  调用 `ibus_config_new()`

  - (5.1) 源文件: `ibus/src/ibusconfig.c`
    函数: `ibus_config_new()`

    调用 `g_dbus_proxy_get_name_owner()`

+ (6) 源文件: `ibus/src/ibusfactory.c`
  函数: `ibus_factory_new()`

  - (6.1) 调用 `g_object_new()`, 间接调用 `ibus_factory_class_init()` (同文件):

    重要函数: `ibus_factory_service_method_call()`

    重要函数: `ibus_factory_real_create_engine()`

    调用 `ibus_service_class_add_interfaces()`

+ (7) 源文件: `ibus/src/ibusservice.c`
  函数: `ibus_service_class_add_interfaces()`

  调用 `g_dbus_node_info_new_for_xml()`

+ (8) 源文件: `ibus/src/ibusfactory.c`
  函数: `ibus_factory_service_method_call()`

  重要回调 `CreateEngine`:
  调用 `ibus_factory_real_create_engine()` (同文件):

  路径 `/org/freedesktop/IBus/Engine/?`

  调用 `ibus_engine_new_with_type(ibus_service_get_connection())`

+ (9) 源文件: `ibus/src/ibusengine.c`
  函数: `ibus_engine_new_with_type()`

  - (9.1) 调用 `g_object_new()`, 间接调用 `ibus_engine_class_init()` (同文件):

    重要函数: `ibus_engine_service_method_call()`

    调用 `ibus_service_class_add_interfaces()` (同上)

  - (9.2) 函数 `ibus_engine_service_method_call()`

    TODO

+ (10) 源文件: `ibus/src/ibusfactory.c`
  函数: `ibus_factory_add_engine()`

  (忽略)

  TODO

+ (11) 源文件: `ibus/src/ibusbus.c`
  函数: `ibus_bus_request_name()`

  - (11.1) 调用 `ibus_bus_call_sync()` (`RequestName`) (同文件):

    调用 `g_dbus_connection_call_sync()`

+ (12) 源文件: `ibus/src/ibusshare.c`
  函数: `ibus_main()`

  调用 `g_main_loop_run()`

初始化分析完毕.

TODO
