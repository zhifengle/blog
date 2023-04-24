package r

type Response struct {
	Code int    `json:"code"`
	Msg  string `json:"msg"`
	Data any    `json:"data,omitempty"`
}

func NewResponse(code int, message string, data any) *Response {
	return &Response{
		Code: code,
		Msg:  message,
		Data: data,
	}
}

func ResCode(code int) *Response {
	return &Response{
		Code: code,
		Msg:  GetMsg(code),
	}
}

func ResData(code int, data any) *Response {
	return &Response{
		Code: code,
		Msg:  GetMsg(code),
		Data: data,
	}
}
