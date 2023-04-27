package web

// 错误码汇总
const (
	OK   = 0
	FAIL = 500

	// code= 9000... 通用错误
	ERROR_REQUEST_PARAM = 9001
	ERROR_REQUEST_PAGE  = 9002
	ERROR_INVALID_PARAM = 9003
	ERROR_DB_OPE        = 9004

	// code= 1000... 用户模块的错误
	ERROR_USER_NAME_USED  = 1001
	ERROR_USER_PASS_WRONG = 1002
	ERROR_USER_NOT_EXIST  = 1003
	ERROR_USER_NO_RIGHT   = 1009
	ERROR_OLD_PASSWORD    = 1010
	ERROR_USER_DISABLED   = 1011

	// code = 1200... 鉴权相关错误
	ERROR_TOKEN_NOT_EXIST  = 1201
	ERROR_TOKEN_RUNTIME    = 1202
	ERROR_TOKEN_WRONG      = 1203
	ERROR_TOKEN_TYPE_WRONG = 1204
	ERROR_TOKEN_CREATE     = 1205
	ERROR_PERMI_DENIED     = 1206
	FORCE_OFFLINE          = 1207
	LOGOUT                 = 1208
)

var codeMsg = map[int]string{
	OK: "OK",
	// Internal Server Error
	FAIL: "内部服务器错误",

	ERROR_REQUEST_PARAM: "请求参数格式错误",
	ERROR_REQUEST_PAGE:  "分页参数错误",
	ERROR_INVALID_PARAM: "不合法的请求参数",
	ERROR_DB_OPE:        "数据库操作异常",

	ERROR_USER_NAME_USED:  "用户名已存在",
	ERROR_USER_NOT_EXIST:  "该用户不存在",
	ERROR_USER_PASS_WRONG: "账号或密码错误",
	ERROR_USER_NO_RIGHT:   "该用户无权限",
	ERROR_USER_DISABLED:   "该用户无权限",
	ERROR_OLD_PASSWORD:    "旧密码不正确",

	ERROR_TOKEN_NOT_EXIST:  "TOKEN 不存在，请重新登陆",
	ERROR_TOKEN_RUNTIME:    "TOKEN 已过期，请重新登陆",
	ERROR_TOKEN_WRONG:      "TOKEN 不正确，请重新登陆",
	ERROR_TOKEN_TYPE_WRONG: "TOKEN 格式错误，请重新登陆",
	ERROR_TOKEN_CREATE:     "TOKEN 生成失败",
	ERROR_PERMI_DENIED:     "权限不足",
	FORCE_OFFLINE:          "您已被强制下线",
	LOGOUT:                 "您已退出登录",
}

func GetMsg(code int) string {
	return codeMsg[code]
}
