import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useUser } from './user-ctx';

export default function Login() {
  const [username, setUsername] = useState();
  const [password, setPassword] = useState();
  const { setUser } = useUser();
  const navigate = useNavigate();

  // input 设置 value 后是受控组件
  return (
    <div>
      <h1>Login Page</h1>
      <label htmlFor="username">Username:</label>
      <input
        id="username"
        placeholder="admin"
        name="username"
        type="text"
        value="admin"
        onChange={(evt) => setUsername(evt.target.value)}
      />

      <br />
      <label htmlFor="password">Password:</label>
      <input
        id="password"
        placeholder="password"
        name="password"
        value={'password'}
        type="password"
        onChange={(evt) => setPassword(evt.target.value)}
      />

      <br />
      <button
        onClick={() => {
          setUser({ username, password });
          setTimeout(() => {
            navigate('/', { replace: true });
          }, 300);
        }}
      >
        Login
      </button>
    </div>
  );
}
