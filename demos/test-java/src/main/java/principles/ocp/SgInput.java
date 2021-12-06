package principles.ocp;

public class SgInput {
    private AbstractSkin skin;

    public void setSkin(AbstractSkin skin) {
        this.skin = skin;
    }
    public void display() {
        skin.display();
    }
}
