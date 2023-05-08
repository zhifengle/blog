package web

import (
	"strconv"
)

func NewError(code int, msg string) *CodeError {
	return &CodeError{code, msg, nil}
}

func NewCodeError(code int) *CodeError {
	msg := GetMsg(code)
	return &CodeError{code, msg, nil}
}

func NewErrorData(code int, msg string, data interface{}) *CodeError {
	return &CodeError{code, msg, data}
}

type CodeError struct {
	Code int
	Msg  string
	Data interface{}
}

func (e *CodeError) Error() string {
	return strconv.Itoa(e.Code) + ": " + e.Msg
}
