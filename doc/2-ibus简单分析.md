# ibus 简单分析

TODO


## D-Bus 总线地址

ibus 使用了 D-Bus 协议进行通信.

<https://github.com/ibus/ibus>
<https://www.freedesktop.org/wiki/Software/dbus/>

同时 ibus 使用了自己独立的一条 D-Bus 总线.

+ 源代码: `ibus/src/ibusshare.h`
  函数: `ibus_get_address()`

  从两个地方获取 D-Bus socket 地址:

  - 环境变量: `IBUS_ADDRESS`

  - 目录: `~/.config/ibus/bus/`

比如:

```
# This file is created by ibus-daemon, please do not modify it.
# This file allows processes on the machine to find the
# ibus session bus with the below address.
# If the IBUS_ADDRESS environment variable is set, it will
# be used rather than this file.
IBUS_ADDRESS=unix:path=/home/s2/.cache/ibus/dbus-PgQYLUex,guid=5301e438ec81cf6d9fa39fa665c5efdf
IBUS_DAEMON_PID=4201
```

这个文件指向了 D-Bus socket (unix) 路径: `~/.cache/ibus/`

```
> ls -l ~/.cache/ibus
总计 0
drwxr-xr-x 1 s2 s2  16  1月21日 19:17 bus/
srwxr-xr-x 1 s2 s2   0  2月 9日 17:26 dbus-PgQYLUex=
srwxr-xr-x 1 s2 s2   0  2月 9日 17:23 dbus-TI3twEnd=
drwx------ 1 s2 s2 340  2月 9日 16:55 libpinyin/
```

`dbus-PgQYLUex=` 就是最终的 D-Bus 总线地址 (unix socket).


## 适配模块

`ibus/client` 目录是 ibus 框架的适配模块.
<https://github.com/ibus/ibus/tree/main/client>

```
> ls ibus/client
gtk2/  gtk3/  gtk4/  Makefile.am  wayland/  x11/
```

可以看到, ibus 为 `gtk2`, `gtk3`, `gtk4`, `wayland`, `x11`
应用分别做了适配, 以便提供输入功能.

```
> cloc .
      29 text files.
      21 unique files.
       8 files ignored.

github.com/AlDanial/cloc v 1.98  T=0.05 s (461.0 files/s, 186026.7 lines/s)
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
C                                8            815            477           4848
C/C++ Header                     6            122           1011            685
make                             6             75            131            305
Vala                             1              0              0              6
-------------------------------------------------------------------------------
SUM:                            21           1012           1619           5844
-------------------------------------------------------------------------------
```

代码量几千行.


## 进程结构

```
> ps -elwwf | grep 4201
0 S s2         4201    1061  0  80   0 - 97111 do_sys 17:26 ?        00:00:06 /usr/bin/ibus-daemon --panel disable
0 S s2         4380    4201  0  80   0 - 77566 do_sys 17:26 ?        00:00:00 /usr/lib/ibus/ibus-dconf
0 S s2         4382    4201  0  80   0 - 115302 do_sys 17:26 ?       00:00:04 /usr/lib/ibus/ibus-extension-gtk3
0 S s2         4507    4201  0  80   0 - 59136 do_sys 17:26 ?        00:00:01 /usr/lib/ibus/ibus-engine-simple
0 S s2         5935    4201  0  80   0 - 103681 do_sys 17:31 ?       00:00:00 /usr/lib/ibus-libpinyin/ibus-engine-libpinyin --ibus
0 S s2         6691    6440 33  80   0 -  2418 pipe_r 17:48 pts/2    00:00:00 grep --color=auto 4201
```

ibus 主进程 (daemon) 是 `/usr/bin/ibus-daemon`, 主进程又启动了一堆下级进程,
比如 `/usr/lib/ibus-libpinyin/ibus-engine-libpinyin` 就是一个具体的输入法.

```
> pacman -Qo /usr/bin/ibus-daemon
/usr/bin/ibus-daemon 由 ibus 1.5.29-3 所拥有
> pacman -Qo /usr/lib/ibus-libpinyin/ibus-engine-libpinyin
/usr/lib/ibus-libpinyin/ibus-engine-libpinyin 由 ibus-libpinyin 1.15.3-1 所拥有
```


## 注册文件

```
> pacman -Ql ibus-libpinyin
ibus-libpinyin /usr/
ibus-libpinyin /usr/lib/
ibus-libpinyin /usr/lib/ibus-libpinyin/
ibus-libpinyin /usr/lib/ibus-libpinyin/ibus-engine-libpinyin
ibus-libpinyin /usr/lib/ibus-libpinyin/ibus-setup-libpinyin
ibus-libpinyin /usr/share/

ibus-libpinyin /usr/share/ibus/
ibus-libpinyin /usr/share/ibus/component/
ibus-libpinyin /usr/share/ibus/component/libpinyin.xml

```

使用这种方式为 ibus 注册具体的输入法.

```xml
> cat /usr/share/ibus/component/libpinyin.xml
<?xml version="1.0" encoding="utf-8"?>
<!-- filename: pinyin.xml -->
<component>
	<name>org.freedesktop.IBus.Libpinyin</name>
	<description>Libpinyin Component</description>
	<exec>/usr/lib/ibus-libpinyin/ibus-engine-libpinyin --ibus</exec>
	<version>1.15.3</version>
	<author>Peng Wu &lt;alexepico@gmail.com&gt;</author>
	<license>GPL</license>
	<homepage>https://github.com/libpinyin/ibus-libpinyin</homepage>
	<textdomain>ibus-libpinyin</textdomain>

	<engines>
		<engine>
			<name>libpinyin</name>
			<language>zh_CN</language>
			<license>GPL</license>
			<author>
                        Peng Wu &lt;alexepico@gmail.com&gt;
                        Peng Huang &lt;shawn.p.huang@gmail.com&gt;
                        BYVoid &lt;byvoid1@gmail.com&gt;
                        </author>
			<icon>/usr/share/ibus-libpinyin/icons/ibus-pinyin.svg</icon>
			<layout>default</layout>
			<longname>Intelligent Pinyin</longname>
			<description>Intelligent Pinyin input method</description>
			<rank>99</rank>
			<symbol>&#x62FC;</symbol>
			<icon_prop_key>InputMode</icon_prop_key>
			<setup>/usr/lib/ibus-libpinyin/ibus-setup-libpinyin libpinyin</setup>
			<textdomain>ibus-libpinyin</textdomain>
		</engine>
		<engine>
			<name>libbopomofo</name>
			<language>zh_TW</language>
			<license>GPL</license>
			<author>
                        Peng Wu &lt;alexepico@gmail.com&gt;
                        Peng Huang &lt;shawn.p.huang@gmail.com&gt;
                        BYVoid &lt;byvoid1@gmail.com&gt;
                        </author>
			<icon>/usr/share/ibus-libpinyin/icons/ibus-bopomofo.svg</icon>
			<layout>default</layout>
			<longname>Bopomofo</longname>
			<description>Bopomofo input method</description>
			<rank>98</rank>
			<symbol>&#x3109;</symbol>
			<icon_prop_key>InputMode</icon_prop_key>
			<setup>/usr/lib/ibus-libpinyin/ibus-setup-libpinyin libbopomofo</setup>
			<textdomain>ibus-libpinyin</textdomain>
		</engine>
	</engines>
</component>
```

加载逻辑分析:

+ 源代码: `ibus/src/Makefile.am`

  内容: `-DIBUS_DATA_DIR=\"$(pkgdatadir)\"`

  此处定义了宏 `IBUS_DATA_DIR`

+ 源文件: `ibus/src/ibusregistry.c`
  函数: `ibus_registry_load`

  代码:

  ```
  dirname = g_build_filename (IBUS_DATA_DIR, "component", NULL);
  g_ptr_array_add (path, dirname);
  ```

  ```
  ibus_registry_load_in_dir (registry, *d);
  ```

  所以目前会固定加载 `/usr/share/ibus/component/` 目录中的 `.xml` 文件.

  ----

  代码:

  ```c
  envstr = g_getenv ("IBUS_COMPONENT_PATH");
  ```

  可以设置环境变量 `IBUS_COMPONENT_PATH`


## systemd 服务

```
> pacman -Ql ibus

ibus /usr/lib/systemd/
ibus /usr/lib/systemd/user/
ibus /usr/lib/systemd/user/gnome-session.target.wants/
ibus /usr/lib/systemd/user/gnome-session.target.wants/org.freedesktop.IBus.session.GNOME.service
ibus /usr/lib/systemd/user/org.freedesktop.IBus.session.GNOME.service
ibus /usr/lib/systemd/user/org.freedesktop.IBus.session.generic.service
```

```
> cat /usr/lib/systemd/user/org.freedesktop.IBus.session.GNOME.service
[Unit]
Description=IBus Daemon for GNOME
CollectMode=inactive-or-failed

# Require GNOME session and specify startup ordering
Requisite=gnome-session-initialized.target
After=gnome-session-initialized.target
PartOf=gnome-session-initialized.target
Before=gnome-session.target

# Needs to run when DISPLAY/WAYLAND_DISPLAY is set
After=gnome-session-initialized.target
PartOf=gnome-session-initialized.target

# Never run in GDM
Conflicts=gnome-session@gnome-login.target

[Service]
Type=dbus
# Only pull --xim in X11 session, it is done via Xwayland-session.d on Wayland
ExecStart=sh -c 'exec /usr/bin/ibus-daemon --panel disable $([ "$XDG_SESSION_TYPE" = "x11" ] && echo "--xim")'
Restart=on-abnormal
BusName=org.freedesktop.IBus
TimeoutStopSec=5
Slice=session.slice

[Install]
WantedBy=gnome-session.target
```

查看服务状态:

```sh
systemctl --user status org.freedesktop.IBus.session.GNOME
```

查看运行日志:

```sh
journalctl --user -eu org.freedesktop.IBus.session.GNOME
```

修改后的服务文件:

```
> cat ~/.config/systemd/user/org.freedesktop.IBus.session.GNOME.service 
[Unit]
Description=IBus Daemon for GNOME
CollectMode=inactive-or-failed

# Require GNOME session and specify startup ordering
Requisite=gnome-session-initialized.target
After=gnome-session-initialized.target
PartOf=gnome-session-initialized.target
Before=gnome-session.target

# Needs to run when DISPLAY/WAYLAND_DISPLAY is set
After=gnome-session-initialized.target
PartOf=gnome-session-initialized.target

# Never run in GDM
Conflicts=gnome-session@gnome-login.target

[Service]
Type=dbus
# Only pull --xim in X11 session, it is done via Xwayland-session.d on Wayland
ExecStart=sh -c 'exec /usr/bin/ibus-daemon --verbose --panel disable $([ "$XDG_SESSION_TYPE" = "x11" ] && echo "--xim")'
Restart=on-abnormal
BusName=org.freedesktop.IBus
TimeoutStopSec=5
Slice=session.slice

Environment=IBUS_COMPONENT_PATH=/usr/share/ibus/component/:/home/s2/.config/ibus/component/

[Install]
WantedBy=gnome-session.target
```


TODO
