use std::rc::Rc;

fn pass_var_closure() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);

    let closure = {
        // 这个做法不用像，放在外面定义一个另外名称的
        // num2_clone,  num3_borrowed

        // num1 直接被 moved
        let num2 = num2.clone();
        // borrowed
        let num3 = num3.as_ref();
        move || *num1 + *num2 + *num3
    };
}
