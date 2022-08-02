import Raw from './Raw';
import Better from './Better';
import './styles.css';

// https://juejin.cn/post/6889247428797530126
export default function App() {
  return (
    <div className="App">
      <h2>优化前</h2>
      <Raw />
      <h2>优化后</h2>
      <Better />
    </div>
  );
}
