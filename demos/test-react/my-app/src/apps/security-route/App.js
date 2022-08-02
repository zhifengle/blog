import './App.css';
import {
  BrowserRouter,
  Route,
  Routes,
  Outlet,
  Navigate,
} from 'react-router-dom';
import Home from './Home';
import Public from './Public';
import Private1 from './Private1';
import Private2 from './Private2';
import Login from './Login';
import { useUser, UserProvider } from './user-ctx';

// https://www.robinwieruch.de/react-router-private-routes/
// 这种方式兼容两种写法. 也可以拆开写
function PrivateRoute({ children, redirectPath = '/login' }) {
  // 文章中是提升上一层的组件。把状态单做 prop 传入
  // 比如用户信息里面的 permissions, roles. 也是判断权限的依据
  const { user } = useUser();
  if (!user) {
    return <Navigate to={redirectPath} replace />;
  }
  return children ? children : <Outlet />;
}

// https://reactrouter.com/docs/en/v6/hooks/use-routes
// useRoutes 是配置 Object 的方式，配置路由

// security or private route
function App() {
  return (
    <div className="App">
      <UserProvider>
        <BrowserRouter>
          <Routes>
            <Route exact path="/" element={<Home />} />
            <Route exact path="/public" element={<Public />} />
            <Route
              path="/private1"
              element={
                <PrivateRoute>
                  <Private1 />
                </PrivateRoute>
              }
            />
            {/* 需要相同的布局时采取这种 */}
            <Route element={<PrivateRoute />}>
              <Route path="/private2" element={<Private2 />} />
            </Route>
            <Route exact path="/login" element={<Login />} />
            <Route path="*" element={<p>There's nothing here: 404!</p>} />
          </Routes>
        </BrowserRouter>
      </UserProvider>
    </div>
  );
}

export default App;
