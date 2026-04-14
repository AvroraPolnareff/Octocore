use skia_safe::Canvas;

trait Page {
    fn enter() {}
    fn exit() {}
    fn draw(canvas: &Canvas) {}
    // TODO Leds
    fn updateLeds() {}
    // TODO Events
    fn keyDown() {}
    fn keyUp() {}
    fn keyPress() {}
    fn encoder() {}
    fn midi() {}
}
