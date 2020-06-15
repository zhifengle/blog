const USER_ID = 'ts/userId'

// localStorage ---> js-cookies
export const setUserId = (data: string) => localStorage.setItem(USER_ID, data)
