package principles.ocp;

public class Client {
    public static void main(String[] args) {
        // input 对象
        SgInput input = new SgInput();
        // 创建皮肤
//        DefaultSkin skin = new DefaultSkin();
        CustomSkin skin = new CustomSkin();
        // 设置皮肤
        input.setSkin(skin);
        // 显示皮肤
        input.display();
    }
}
