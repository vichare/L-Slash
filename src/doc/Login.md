# 登录系统

## 页面

* 所有页面
所有页面初始都需要验证名字为 auth 的 cookie 。如果验证成功，则获知对应的用户信息
如果不成功，跳转到登录页面

* 登录页面
访问任何页面如果没有登录则跳转到登陆页面，这时候要记住之前的页面在哪里。

如果访问登录页面，但是已经登录，则跳转到首页。
如果访问登录页面，没有登录（有可能是从其他页面跳转过来的），则显示用户名和密码输入框。

* 登录post请求处理页面
登录成功之后跳转回登陆前页面，如果不存在则跳转到首页。
登录不成功则跳转到登录页面

如果前后端分离这里应该用js call API

## Cookie
Cookie 内存储一对 base64 编码的字符串 key 和 secret。
key 唯一确定一个数据库中存贮的session，secret用于验证。

## Session
数据库中的 Session 记录了如下字段：
* key
* secret
* user name
* expire time
* rotation time
* previous secret
* previous secret expire time

## Rotation
每次验证 session 的时候都会检查当前 session 是否需要 rotation 。
如果当前时间大于 rotation time，那么更新 secret，previous secret
并且相应更新所有的时间 field。
