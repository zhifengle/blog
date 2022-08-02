import { useContext, createContext, useState } from 'react';

// 一般是把这三个拆分成三个文件 ？
const UserContext = createContext({});

export function useUser() {
  return useContext(UserContext);
}

export function UserProvider(props) {
  const [user, setUser] = useState(null);
  const logout = () => {
    setUser(null);
  };
  return (
    <UserContext.Provider
      value={{
        user,
        setUser,
        logout,
      }}
    >
      {props.children}
    </UserContext.Provider>
  );
}
