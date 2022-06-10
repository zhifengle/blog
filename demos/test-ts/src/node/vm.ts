import vm from 'vm';

function testVm() {
  const x = 1;

  const context: any = { x: 2 };
  var ctx1 = vm.createContext();
  ctx1.x = 2;
  vm.createContext(context); // 上下文隔离化对象。

  const code = 'x += 40; var y = 17;';
  // `x` 和 `y` 是上下文中的全局变量。
  // 最初，x 的值为 2，因为这是 context.x 的值。
  vm.runInContext(code, ctx1);

  console.log(ctx1.x); // 42
  console.log(ctx1.y); // 17

  console.log(x); // 1; y 未定义。
}

function test2() {
  const context = {
    animal: 'cat',
    count: 2,
  };

  const script = new vm.Script('count += 1; name = "kitty";');

  vm.createContext(context);
  for (let i = 0; i < 10; ++i) {
    script.runInContext(context);
  }

  console.log(context);
  // 打印: { animal: 'cat', count: 12, name: 'kitty' }
}

test2();
