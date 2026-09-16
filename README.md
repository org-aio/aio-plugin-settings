# AIO 设置中心 / AIO Settings Center

提供常规、外观、关于，以及已安装插件贡献的设置分组。主题与密度通过共享工作台的 `AppearanceScope` 按当前设备及账户保存，默认跟随系统与舒适密度。

Provides General, Appearance, About, plus setting groups contributed by installed plugins. Theme and density are saved per device and account through the shared workbench `AppearanceScope`, defaulting to system appearance and comfortable density.

宿主通过 `ApplicationSettings` 上下文提供权限过滤后的响应式分组与页面挂载槽。设置中心按分组展示，插件组标题下显示“来自插件（插件 title）”，只挂载选中插件的设置内容。安装、停用、卸载和权限变化自动更新，不维护第二份插件清单，不复制插件表单或密钥存储。

The host provides permission-filtered responsive groups and page mounting slots through the `ApplicationSettings` context. The settings center renders groups; plugin group titles show “来自插件（插件 title）” (from plugin &lt;plugin title&gt;), mounting only the selected plugin's settings. Install, disable, uninstall and permission changes update automatically — no second plugin manifest is maintained, and plugin forms or secret storage are never duplicated.

用户 ID、工作区 ID 和权限编码收进“关于”的技术详情。切换工作区、资料编辑使用账户菜单中的现有入口。

User ID, workspace ID and permission codes are collected under the technical details of “About”. Workspace switching and profile editing use the existing entries in the account menu.
