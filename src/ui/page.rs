use skia_safe::Canvas;

trait Page {
    fn enter() {}
    fn exit() {}
    fn draw(canvas: &Canvas) {}
    // TODO Leds
    fn update_leds() {}
    // TODO Events
    fn key_down() {}
    fn key_up() {}
    fn key_press() {}
    fn encoder() {}
    fn midi() {}
}
