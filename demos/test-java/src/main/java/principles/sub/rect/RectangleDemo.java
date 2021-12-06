package principles.sub.rect;

public class RectangleDemo {
    public static void main(String[] args) {
        Rectangle r = new Rectangle();
        r.setWidth(10);
        r.setHeight(20);
        resize(r);
        print(r);

        Square s = new Square();
        s.setSide(20);
        print(s);
    }

    public static void resize(Rectangle rectangle) {
        while (rectangle.getWidth() <= rectangle.getHeight()) {
            rectangle.setWidth(rectangle.getWidth() + 1);
        }
    }

    public static void print(Quadrilateral quadrilateral) {
        System.out.printf("width: %s\n", quadrilateral.getWidth());
        System.out.printf("height: %s\n", quadrilateral.getHeight());
    }
}
