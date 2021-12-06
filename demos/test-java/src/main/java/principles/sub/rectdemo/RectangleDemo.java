package principles.sub.rectdemo;

public class RectangleDemo {
    public static void main(String[] args) {
        Rectangle r = new Rectangle();
        r.setWidth(10);
        r.setHeight(20);
        resize(r);
        print(r);
        // 传入 正方形报错了。违反里氏替换原则
    }
    public static void resize(Rectangle rectangle) {
        while (rectangle.getWidth() <= rectangle.getHeight()) {
            rectangle.setWidth(rectangle.getWidth() + 1);
        }
    }
    public static void print(Rectangle rectangle) {
        System.out.printf("width: %s\n", rectangle.getWidth());
        System.out.printf("height: %s\n", rectangle.getHeight());
    }
}
