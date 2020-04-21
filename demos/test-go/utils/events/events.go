package events

import "sync"

type (
	EventName string
	Listener  func(...interface{})
	Events    map[EventName][]Listener

	EventEmitter interface {
		On(EventName, ...Listener)
		//Once(EventName, ...Listener)
		Emit(EventName, ...interface{})
		//Off(EventName)
		//Clear()
	}

	emitter struct {
		events Events
		mu     sync.Mutex
	}
)

func New() EventEmitter {
	return &emitter{events: Events{}}
}

//var defaultEmitter = New()

func (e *emitter) On(evt EventName, listeners ...Listener) {
	if len(listeners) == 0 {
		return
	}
	e.mu.Lock()
	defer e.mu.Unlock()

	if e.events == nil {
		e.events = make(Events)
		//e.curListeners = Events{}
	}
	// TODO 限制监听的事件的大小

	curListeners := e.events[evt]
	if curListeners == nil {
		curListeners = make([]Listener, 0)
	}
	e.events[evt] = append(curListeners, listeners...)
}

func (e *emitter) Emit(evt EventName, data ...interface{}) {
	if e.events == nil {
		return
	}
	if listeners := e.events[evt]; listeners != nil && len(listeners) > 0 {
		for i := range listeners {
			l := listeners[i]
			if l != nil {
				l(data...)
			}
		}
	}
}
