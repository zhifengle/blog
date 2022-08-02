import { useUser } from './user-ctx';

export default function LogoutBtn() {
  const { logout } = useUser();
  return <button onClick={logout}>Logout</button>;
}
