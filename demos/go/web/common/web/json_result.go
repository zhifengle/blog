package web

type JsonResult struct {
	Code int    `json:"code"`
	Msg  string `json:"msg"`
	Data any    `json:"data,omitempty"`
}

func Json(code int, message string, data any) *JsonResult {
	return &JsonResult{
		Code: code,
		Msg:  message,
		Data: data,
	}
}

func JsonData(data any) *JsonResult {
	return &JsonResult{
		Code: OK,
		Data: data,
	}
}

func JsonSuccess() *JsonResult {
	return &JsonResult{
		Code: OK,
		Data: nil,
	}
}

func JsonError(err error) *JsonResult {
	if e, ok := err.(*CodeError); ok {
		return &JsonResult{
			Code: e.Code,
			Msg:  e.Msg,
			Data: e.Data,
		}
	}
	return &JsonResult{
		Code: FAIL,
		Msg:  err.Error(),
		Data: nil,
	}
}

func JsonErrorCode(code int) *JsonResult {
	return &JsonResult{
		Code: code,
		Msg:  GetMsg(code),
		Data: nil,
	}
}

func JsonErrorData(code int, message string, data interface{}) *JsonResult {
	return &JsonResult{
		Code: code,
		Msg:  message,
		Data: data,
	}
}
